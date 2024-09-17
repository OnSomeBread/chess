use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Light,
    Dark,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    King,
    Queen,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    piece: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(code: char) -> Option<Self> {
        let col = match code.is_alphabetic() {
            true => match code.is_uppercase() {
                false => Some(Color::Dark),
                true => Some(Color::Light),
            },
            false => None,
        };

        let c = code.to_lowercase().next();
        let p = match c {
            Some('p') => Some(PieceType::Pawn),
            Some('n') => Some(PieceType::Knight),
            Some('b') => Some(PieceType::Bishop),
            Some('r') => Some(PieceType::Rook),
            Some('q') => Some(PieceType::Queen),
            Some('k') => Some(PieceType::King),
            _ => None,
        };
        if p.is_some() && col.is_some() {
            #[allow(clippy::unnecessary_unwrap)]
            return Some(Piece {
                piece: p.unwrap(),
                color: col.unwrap(),
            });
        }
        None
    }

    pub fn get_file_location(&self) -> String {
        let mut ans = match self.color {
            Color::Light => "pieces/w".to_string(),
            Color::Dark => "pieces/b".to_string(),
        };

        ans += match self.piece {
            PieceType::Pawn => "p.png",
            PieceType::Knight => "n.png",
            PieceType::Bishop => "b.png",
            PieceType::Rook => "r.png",
            PieceType::Queen => "q.png",
            PieceType::King => "k.png",
        };

        ans
    }

    pub fn all_pieces() -> Vec<Piece> {
        let piece_types = vec![
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ];
        let mut ans = vec![];
        for p in piece_types.into_iter() {
            ans.push(Piece {
                piece: p,
                color: Color::Light,
            });
            ans.push(Piece {
                piece: p,
                color: Color::Dark,
            });
        }
        ans
    }

    pub fn get_id(&self) -> i32 {
        let mut ans = match self.piece {
            PieceType::Pawn => 1,
            PieceType::Knight => 2,
            PieceType::Bishop => 3,
            PieceType::Rook => 4,
            PieceType::Queen => 5,
            PieceType::King => 6,
        };
        ans += match self.color {
            Color::Light => 0,
            Color::Dark => 6,
        };
        ans
    }

    pub fn get_piecetype(&self) -> PieceType {
        self.piece
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    #[allow(dead_code)]
    pub fn get_value(&self) -> i32 {
        let ans = match self.piece {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 8,
            PieceType::King => 10,
        };
        match self.color {
            Color::Light => ans,
            Color::Dark => -ans,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pieces: Vec<Option<Piece>>,
    size: i32,
}

impl Board {
    pub fn new(fen: String, s: i32) -> Self {
        let mut board = vec![];

        let mut col = 0;
        let mut iterator = fen.chars();
        while let Some(mut i) = iterator.next() {
            if i.is_ascii_digit() {
                let mut num = String::new();
                num.push(i);

                #[allow(clippy::while_let_on_iterator)]
                while let Some(c) = iterator.next() {
                    i = c;
                    if c.is_ascii_digit() {
                        num.push(c);
                    }
                    else {
                        break;
                    }
                }
                
                let count = num.parse::<i32>().unwrap();
                
                // let count = match i.to_digit(10) {
                //     Some(val) => val as i32-1,
                //     None => 0,
                // };
                //row += count;
                for _j in 0..count {
                    col += 1;
                    board.push(Piece::new('#'));
                }
            }
            if i == '/' {
                if col < s {
                    for _j in col..s {
                        board.push(Piece::new('#'));
                    }
                }
                col = 0;
            }
            else if !i.is_ascii_digit() {
                col += 1;
                board.push(Piece::new(i));
            }
        }

        let c = (s * s) as usize;
        while board.len() < c {
            board.push(Piece::new('#'));
        }

        Board {
            pieces: board,
            size: s,
        }
    }

    pub fn move_piece(&mut self, o_pos: usize, n_pos: usize, possible_moves:&HashSet<i32>, turn: &mut Color) -> bool {
        if o_pos == n_pos
            || o_pos >= self.pieces.len()
            || n_pos >= self.pieces.len()
            || self.pieces[o_pos].is_none()
        {
            return false;
        }

        // if self.pieces[o_pos].unwrap().get_color() != turn {
        //     return false;
        // }

        if !possible_moves.contains(&(n_pos as i32)) {
            return false;
        }

        // switches player turn if piece successfully moves
        if *turn == Color::Light {
            *turn = Color::Dark;
        } else {
            *turn = Color::Light;
        }

        self.pieces[n_pos] = self.pieces[o_pos];
        self.pieces[o_pos] = None;
        true
    }

    // used for the below piece functions
    fn is_safe(&self, pos: i32, color: Color) -> Option<i32> {
        if self.pieces[pos as usize].is_none()
            || self.pieces[pos as usize].unwrap().get_color() != color
        {
            return Some(pos);
        }
        None
    }

    fn pawn_moves(&self, pos:usize, color:Color) -> HashSet<i32> {
        let pos = pos as i32;
        let mut ans = HashSet::new();
        match color {
            Color::Light => {
                if self.pieces[(pos-self.size) as usize].is_none() {
                    ans.insert(pos-self.size);
                    // checks second rank
                    if pos/self.size == self.size - 2 && self.pieces[(pos-self.size*2) as usize].is_none() {
                        ans.insert(pos-self.size*2);
                    }
                }
            },
            Color::Dark => {
                if self.pieces[(pos+self.size) as usize].is_none() {
                    ans.insert(pos+self.size);
                    // checks 7th rank for a typical 8*8 board
                    if pos/self.size == 1 && self.pieces[(pos+self.size*2) as usize].is_none() {
                        ans.insert(pos+self.size*2);
                    }
                }
            },
        }
        ans
    }

    fn knight_moves(&self, pos:usize, color:Color) -> HashSet<i32> {
        let pos = pos as i32;
        let mut ans = HashSet::new();
        let moves = vec![pos - self.size*2 - 1, pos - self.size*2 + 1, pos - 2 - self.size, pos + 2 - self.size, pos + self.size - 2, pos + self.size + 2, pos + self.size * 2 - 1, pos + self.size * 2 + 1];

        for mov in moves.iter() {
            if mov >= &0 && mov < &(self.size*self.size) {
                if let Some(v) = self.is_safe(*mov, color) {
                    ans.insert(v);
                }
            }
        }

        ans
    }

    fn bishop_moves(&self, pos: usize, color: Color) -> HashSet<i32> {
        let pos = pos as i32;
        let mut ans = HashSet::new();

        let mut ul = pos - self.size - 1;
        let mut x = pos % self.size - 1;
        while x >= 0 && ul >= 0 {
            if let Some(v) = self.is_safe(ul, color) {
                ans.insert(v);
            }
            if self.pieces[ul as usize].is_some() {
                break;
            }
            x -= 1;
            ul -= self.size + 1;
        }

        let lim = self.size * self.size;

        let mut bl = pos + self.size - 1;
        let mut x = pos % self.size - 1;
        while x >= 0 && bl < lim {
            if let Some(v) = self.is_safe(bl, color) {
                ans.insert(v);
            }
            if self.pieces[bl as usize].is_some() {
                break;
            }
            x -= 1;
            bl += self.size - 1;
        }

        let mut ur = pos - self.size + 1;
        let mut x = pos % self.size + 1;
        while x < self.size && ur >= 0 {
            if let Some(v) = self.is_safe(ur, color) {
                ans.insert(v);
            }
            if self.pieces[ur as usize].is_some() {
                break;
            }
            x += 1;
            ur -= self.size - 1;
        }

        let mut br = pos + self.size + 1;
        let mut x = pos % self.size + 1;
        while x < self.size && br < lim {
            if let Some(v) = self.is_safe(br, color) {
                ans.insert(v);
            }
            if self.pieces[br as usize].is_some() {
                break;
            }
            x += 1;
            br += self.size + 1;
        }

        ans
    }

    fn rook_moves(&self, pos: usize, color: Color) -> HashSet<i32> {
        let pos = pos as i32;
        let mut ans = HashSet::new();

        // checks left and right directions
        let mut l = pos - 1;
        let mut r = pos + 1;
        let left_bound = (pos / self.size) * self.size;
        while l >= left_bound {
            if let Some(v) = self.is_safe(l, color) {
                ans.insert(v);
            }
            if self.pieces[l as usize].is_some() {
                break;
            }
            l -= 1;
        }

        while r < left_bound + self.size {
            if let Some(v) = self.is_safe(r, color) {
                ans.insert(v);
            }
            if self.pieces[r as usize].is_some() {
                break;
            }
            r += 1;
        }

        // checks up and down directions
        let mut u = pos - self.size;
        let mut d = pos + self.size;
        let bound = pos % self.size;
        while u >= bound {
            if let Some(v) = self.is_safe(u, color) {
                ans.insert(v);
            }
            if self.pieces[u as usize].is_some() {
                break;
            }
            u -= self.size;
        }

        while d < self.size * self.size {
            if let Some(v) = self.is_safe(d, color) {
                ans.insert(v);
            }
            if self.pieces[d as usize].is_some() {
                break;
            }
            d += self.size;
        }
        ans
    }

    fn queen_moves(&self, pos:usize, color:Color) -> HashSet<i32> {
        let mut ans = self.bishop_moves(pos, color);
        ans.extend(self.rook_moves(pos, color));
        ans
    }

    fn king_moves(&self, pos:usize, color:Color) -> HashSet<i32> {
        let pos = pos as i32;
        let mut ans = HashSet::new();
        let moves = vec![pos-self.size-1, pos-self.size, pos-self.size+1, pos-1, pos+1, pos+self.size-1, pos+self.size, pos+self.size+1];

        for mov in moves.iter() {
            if mov >= &0 && mov < &(self.size*self.size) {
                if let Some(v) = self.is_safe(*mov, color) {
                    ans.insert(v);
                }
            }
        }

        ans
    }

    pub fn get_valid_moves(&self, pos: usize) -> HashSet<i32> {
        let p = self.pieces.get(pos);
        if p.is_none() {
            return HashSet::new();
        }
        let p = p.unwrap();
        if p.is_none() {
            return HashSet::new();
        }

        let p = p.unwrap();
        match p.get_piecetype() {
            PieceType::Pawn => {
                Self::pawn_moves(self, pos, p.get_color())
            }
            PieceType::Knight => {
                Self::knight_moves(self, pos, p.get_color())
            }
            PieceType::Bishop => Self::bishop_moves(self, pos, p.get_color()),
            PieceType::Rook => Self::rook_moves(self, pos, p.get_color()),
            PieceType::Queen => Self::queen_moves(self, pos, p.get_color()),
            PieceType::King => {
                Self::king_moves(self, pos, p.get_color())
            }
        }
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_pieces(&self) -> Vec<Option<Piece>> {
        self.pieces.clone()
    }
}
