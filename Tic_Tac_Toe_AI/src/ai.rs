use crate::Game;
use rand::Rng;
use std::collections::HashMap;

pub struct Player {
    name:char,
    #[allow(unused)]
    pub q_table:HashMap<[[char;3];3], i32>
}

impl Player {
    pub fn new(n:char) -> Self {
        Self {name:n, q_table:HashMap::new()}
    }

    #[allow(unused)]
    pub fn rand_play(&self, game:&mut Game) {
        let mut r = rand::thread_rng();
        let mut v = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if game.board[i][j] == '#' {
                    v.push((i,j));
                }
            }
        }
        
        if v.len() != 0 {
            let index = r.gen_range(0..v.len());
            let (row, column) = v[index];
            game.play(row, column, self.name);
        }
    }

    #[allow(unused)]
    fn next_moves(&self, game:&Game) -> Vec<[[char; 3]; 3]> {
        let mut board = game.board.clone();
        let mut v = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                if game.board[i][j] == '#' {
                    board[i][j] = self.name;
                    v.push(board);
                    board = game.board.clone();
                }
            }
        }
        v
    }

    #[allow(unused)]
    fn other_player(&self) -> char {
        if self.name == 'x' {
            'o'
        } else {
            'x'
        }
    }

    #[allow(unused)]
    pub fn train_play(&mut self, game:&mut Game) {
        let moves = self.next_moves(game);

        for b in &moves {
            let game2 = Game{board:*b};
            if game2.winner() == self.name {
                self.q_table.insert(*b, 1);
                self.q_table.insert(game.board, 1);
            } else if game2.winner() == self.other_player() {
                self.q_table.insert(*b, -1);
            } else if game2.isgameover() && game2.winner() == '#' {
                self.q_table.insert(*b, -1);
            } else {
                let val = match self.q_table.get(b) {
                    Some(a) => a,
                    None => &0
                };
                self.q_table.insert(*b, *val);
            }
        }

        let mut max = -1;
        for b in &moves {
            let a = self.q_table.get(b).unwrap_or(&0);
            if *a > max {
                max = *a
            }
        }

        self.q_table.insert(game.board, max);

        let mut r = rand::thread_rng();
        game.board = moves[r.gen_range(0..moves.len())];

    }

    #[allow(unused)]
    pub fn play(&self, game:&mut Game) {
        let moves = self.next_moves(game);
        if moves.len() == 1 {
            game.board = moves[0];
        } else {
            let mut board = moves[0];
            let mut val = self.q_table.get(&board).unwrap_or(&0);
            for i in 1..moves.len() {
                let temp_board = moves[i];
                let temp_val = self.q_table.get(&temp_board).unwrap_or(&0);
                if temp_val > val {
                    val = temp_val;
                    board = temp_board;
                }
            }
            game.board = board;
        }
    }
}