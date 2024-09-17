
// 51. N-Queens
use std::collections::HashSet;
fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut col: HashSet<i32> = HashSet::new();
    let mut pos: HashSet<i32> = HashSet::new();
    let mut neg: HashSet<i32> = HashSet::new();
    let mut board = vec![".".to_string().repeat(n as usize); n as usize];
    let mut ans = vec![];

    pub fn queens_backtrack(col:&mut HashSet<i32>, pos:&mut HashSet<i32>, neg:&mut HashSet<i32>, board: &mut Vec<String>, ans:&mut Vec<Vec<String>>, row:i32, n:i32) {
        if row == n {
            ans.push(board.clone());
            return ;
        }
    
        for i in 0..n {
            if col.contains(&i) || pos.contains(&(row - i)) || neg.contains(&(row + i)) {
                continue;
            }
    
            col.insert(i);
            pos.insert(row - i);
            neg.insert(row + i);
            board[row as usize].replace_range(i as usize..i as usize+1, "Q");
    
            queens_backtrack(col, pos, neg, board, ans, row + 1, n);
    
            col.remove(&i);
            pos.remove(&(row - i));
            neg.remove(&(row + i));
            board[row as usize].replace_range(i as usize..i as usize+1, ".");
        }
    }

    queens_backtrack(&mut col, &mut pos, &mut neg, &mut board, &mut ans, 0, n);

    ans
}

// puts each of the n queen boards into fen strings instead
// did not fix case for n > 10 since fen string will do ...10Q5/... will be parsed as move 1 and 0 instead of move 10 
pub fn n_queens_fen(n:i32) -> Vec<String> {
    let queens = solve_n_queens(n);
    let mut ans = vec![];

    for board in queens.into_iter() {
        let mut fen = String::new();

        for row in board.into_iter() {
            let mut amount:i32 = 0;

            for letter in row.chars() {
                if letter == '.' {
                    amount += 1;
                }
                else if letter == 'Q' {
                    if amount != 0 {
                        fen += amount.to_string().as_str();
                    }
                    
                    fen += "Q";
                    amount = 0;
                }
            }
            if amount != 0 {
                fen += amount.to_string().as_str();
            }

            fen += "/";
        }
        ans.push(fen);
    }
    ans
}