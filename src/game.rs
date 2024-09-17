use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use piston::BuildFromWindowSettings;

use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::input::*;
use std::collections::HashSet;
use std::path::Path;
use piston::event_loop::*;

use crate::board::*;
use crate::nqueens::n_queens_fen;

pub fn init_game_window(opengl:OpenGL, initial_window_size: u32) -> GlutinWindow {
    let mut window = WindowSettings::new(
        "chess",
        [initial_window_size, initial_window_size],
    )
    .exit_on_esc(true);
    window.set_samples(16);
    window.set_graphics_api(opengl);

    <GlutinWindow as BuildFromWindowSettings>::build_from_window_settings(&window).unwrap()
}

pub struct Game {
    gl: GlGraphics,
    win_size: i32,
    piece_textures: std::collections::HashMap<i32, Texture>,
}

impl Game {
    pub fn new(g: GlGraphics, size: i32) -> Self {
        let pieces = Piece::all_pieces();
        let mut textures = std::collections::HashMap::new();

        for piece in pieces.iter() {
            let image_location = piece.get_file_location();
            let piece_image =
                Texture::from_path(Path::new(&image_location), &TextureSettings::new()).unwrap();
            textures.insert(piece.get_id(), piece_image);
        }

        Game {
            gl: g,
            win_size: size,
            piece_textures: textures,
        }
    }

    // actually draws the board
    pub fn board(&mut self, board: &Board, arg: &RenderArgs, possible_moves: &HashSet<i32>, prev: i32, post: i32) {
        // get rgb value divide by 255 to get float representation
        // let white:[f32;4] = [1.0, 1.0, 1.0, 1.0];
        // let black:[f32;4] = [0.0, 0.0, 0.0, 1.0];
        let brown: [f32; 4] = [0.66, 0.47, 0.39, 1.0];
        let light_brown: [f32; 4] = [0.94, 0.85, 0.75, 1.0];
        let yellow: [f32; 4] = [0.83, 0.71, 0.32, 1.0];
        let orange: [f32; 4] = [1.0, 0.6392156, 0.1019607, 1.0];

        let c = board.get_size();
        let square_size: f64 = (self.win_size / c) as f64;
        
        for i in 0..c {
            for j in 0..c {
                let square = graphics::rectangle::square(
                    square_size * i as f64,
                    square_size * j as f64,
                    square_size,
                );

                let point = i + j * c;
                let color: [f32; 4];
                if possible_moves.contains(&((j * c) + i)) {
                    color = orange;
                }
                else if point == prev || point == post {
                    color = yellow;
                }
                else {
                    color = match (i + j) % 2 {
                        1 => brown,
                        0 => light_brown,
                        _ => brown, // technically should not even be possible for this loop unless integer overflow
                    };
                }

                self.gl.draw(arg.viewport(), |c, gl| {
                    let transform = c.transform;
                    graphics::rectangle(color, square, transform, gl);
                });
            }
        }
    }

    pub fn draw_pieces(&mut self, board: &Board, arg: &RenderArgs) {
        let s = board.get_size();
        for (i, piece) in board.get_pieces().iter().enumerate() {
            if let Some(p) = piece {
                let square_size: f64 = (self.win_size / s) as f64;
                let img = graphics::Image::new().rect(graphics::rectangle::square(
                    square_size * (i % s as usize) as f64,
                    square_size * (i / s as usize) as f64,
                    square_size,
                ));

                // finds the texture for the specific piece textures defined in new function
                let piece_image = self.piece_textures.get(&p.get_id()).unwrap();

                self.gl.draw(arg.viewport(), |c, gl| {
                    img.draw(
                        piece_image,
                        &graphics::DrawState::new_alpha(),
                        c.transform,
                        gl,
                    );
                });
            }
        }
    }

    pub fn move_piece(&mut self, board: &mut Board, possible_moves:&HashSet<i32>,old_pos: [f64; 2], new_pos: [f64; 2], turn: &mut Color) -> [i32; 2] {
        // if coords are less than the size of the window
        if old_pos[0] < 0.0
            || old_pos[1] < 0.0
            || new_pos[0] < 0.0
            || new_pos[1] < 0.0
            || new_pos[0] > self.win_size as f64
        {
            return [-1, -1];
        }

        let board_size = board.get_size();
        let square_size = self.win_size / board_size;

        let o_pos =
            old_pos[0] as i32 / square_size + (old_pos[1] as i32 / square_size) * board_size;
        let n_pos =
            new_pos[0] as i32 / square_size + (new_pos[1] as i32 / square_size) * board_size;

        match board.move_piece(o_pos as usize, n_pos as usize, possible_moves, turn) {
            true => [o_pos, n_pos],
            false => [-1, -1],
        }
    }

    pub fn get_board_pos_from_cursor(&self, board: &Board, cursor_pos: [f64; 2]) -> Option<usize> {
        if cursor_pos[0] < 0.0
            || cursor_pos[1] < 0.0
            || cursor_pos[0] > self.win_size as f64
            || cursor_pos[1] > self.win_size as f64
        {
            return None;
        }

        let board_size = board.get_size();
        let square_size = self.win_size / board_size;
        Some(
            (cursor_pos[0] as i32 / square_size + (cursor_pos[1] as i32 / square_size) * board_size)
                as usize,
        )
    }

    pub fn change_win_size(&mut self, win_size: i32) {
        self.win_size = win_size;
    }

    pub fn n_queens(&mut self, game_window: &mut GlutinWindow, board_size:i32, time_till_switch:u64) {
        //let mut b: Board = Board::new("".to_string(), board_size);
        let mut event = Events::new(EventSettings::new());

        let queen_fens = n_queens_fen(board_size);
        let boards:Vec<Board> = queen_fens.into_iter().map(|x| Board::new(x, board_size)).collect();
        let mut boards_iter = boards.iter();

        let tmp = HashSet::new();

        while let Some(e) = event.next(game_window) {
            if let Some(r) = e.render_args() {
                if let Some(item) = boards_iter.next() {
                    self.board(item,&r, &tmp, -1, -1);
                    self.draw_pieces(item,&r);
                    std::thread::sleep(std::time::Duration::from_millis(time_till_switch));
                }
            }
        }
    }
}
