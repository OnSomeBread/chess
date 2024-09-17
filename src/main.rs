extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use game::init_game_window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use std::collections::HashSet;

mod board;
mod game;
use crate::board::*;

mod nqueens;

fn main() {
    let opengl = OpenGL::V4_5;

    // the window_size should be cleanly divisible by board_size otherwise there may be blank pixels on the edges of the screen
    let initial_window_size = 860_i32;
    let board_size = 8_i32;

    // the starting board as a fen string along with some other example fen strings
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    //let fen = "rnbqkbnr/pppppppp/////PPPPPPPP/RNBQKBNR";
    //let fen = "r1b1k1nr/p2p1pNp/n2B4/1p1NP2P/6P1/3P1Q2/P1P1K3/q5b1";
    //let fen = "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8";
    //let fen = "///////";
    //let fen = "b/r5n//q2k//Q2K/R5N/B6B";

    // this does a different event loop where the window shows different solutions to the n queens problem
    let n_queens = false;

    let mut game_window = init_game_window(opengl, initial_window_size as u32);
    let mut game = game::Game::new(GlGraphics::new(opengl), initial_window_size);

    // run the n-queens window loop instead
    if n_queens {
        game.n_queens(&mut game_window, board_size, 250);
        return ;
    }

    let mut b = Board::new(fen.to_string(), board_size);
    let mut event = Events::new(EventSettings::new());
    event.set_lazy(true);

    let mut prev: i32 = -1;
    let mut post: i32 = -1;
    let mut turn = Color::Light;

    let mut last_cursor_pos = [-1.0, -1.0];
    let mut start_cursor_pos = [-1.0, -1.0];
    let mut possible_moves: HashSet<i32> = HashSet::new();

    // the main game loop
    while let Some(e) = event.next(&mut game_window) {
        if let Some(r) = e.resize_args() {
            game.change_win_size(std::cmp::min(
                r.window_size[0] as i32,
                r.window_size[1] as i32,
            ));
        }

        if let Some(r) = e.render_args() {
            game.board(&b, &r, &possible_moves, prev, post);
            game.draw_pieces(&b, &r);
        }

        if let Some(r) = e.button_args() {
            // if user left clicks
            if r.button == Button::Mouse(MouseButton::Left) && r.state == ButtonState::Press {
                start_cursor_pos = last_cursor_pos;

                // grab board position that the user clicked on
                if let Some(v) = game.get_board_pos_from_cursor(&b, last_cursor_pos) {
                    if possible_moves.contains(&(v as i32)) {
                        let points = game.move_piece(&mut b, &possible_moves, start_cursor_pos, last_cursor_pos, &mut turn);

                        // this handles the yellow background on the piece that just moved
                        if points[0] != points[1] {
                            prev = points[0];
                            post = points[1];
                        }
                    }
                    possible_moves = b.get_valid_moves(v);
                };
            }
            // if user stops left clicking
            if r.button == Button::Mouse(MouseButton::Left) && r.state == ButtonState::Release {
                let points = game.move_piece(&mut b, &possible_moves, start_cursor_pos, last_cursor_pos, &mut turn);

                // this handles the yellow background on the piece that just moved
                if points[0] != points[1] {
                    prev = points[0];
                    post = points[1];
                    possible_moves = HashSet::new();
                }

                println!("MOVE FROM {:?} TO {:?}", start_cursor_pos, last_cursor_pos);
                start_cursor_pos = [-1.0, -1.0];
            }
        }

        if let Some(cursor_args) = e.mouse_cursor_args() {
            last_cursor_pos = cursor_args;
        }

        // if let Some(b) = e.mouse_relative_args() {
        //     println!("{:?}", b);
        // }
    }
}
