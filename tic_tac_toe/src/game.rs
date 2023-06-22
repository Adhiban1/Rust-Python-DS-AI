use std::io::{Write, stdin, stdout};

pub struct Game {
    board:[[char;3];3],
    player:char
}

impl Game {
    pub fn new() -> Self {
        Self{board:[
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#'],
        ], player:'x'}
    }

    pub fn print_board(&self) {
        stdout().write_all(b"\x1B[2J\x1B[1;1H").unwrap();
        for i in self.board {
            for j in i {
                print!("{}", j);
            }
            println!("");
        }
    }

    pub fn get_play_input(&mut self) {
        print!("Move: ");
        stdout().flush().unwrap();
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let s = s.trim().split(",").collect::<Vec<_>>();
        let row = s[0].parse::<usize>().unwrap();
        let column = s[1].parse::<usize>().unwrap();
        self.play(row, column);
    }

    pub fn play(&mut self, row:usize, column:usize) {
        if self.player == 'x'{
            self.board[row-1][column-1] = 'x';
            self.player = 'o';
        } else {
            self.board[row-1][column-1] = 'o';
            self.player = 'x';
        }
    }

    pub fn winner(&self) -> char {
        if self.board[0][0] == 'x' && self.board[0][1] == 'x' && self.board[0][2] == 'x' {
            'x'
        } else if self.board[1][0] == 'x' && self.board[1][1] == 'x' && self.board[1][2] == 'x' {
            'x'
        } else if self.board[2][0] == 'x' && self.board[2][1] == 'x' && self.board[2][2] == 'x' {
            'x'
        } else if self.board[0][0] == 'x' && self.board[1][0] == 'x' && self.board[2][0] == 'x' {
            'x'
        } else if self.board[0][1] == 'x' && self.board[1][1] == 'x' && self.board[2][1] == 'x' {
            'x'
        } else if self.board[0][2] == 'x' && self.board[1][2] == 'x' && self.board[2][2] == 'x' {
            'x'
        } else if self.board[0][0] == 'x' && self.board[1][1] == 'x' && self.board[2][2] == 'x' {
            'x'
        } else if self.board[0][2] == 'x' && self.board[1][1] == 'x' && self.board[2][0] == 'x' {
            'x'
        } else if self.board[0][0] == 'o' && self.board[0][1] == 'o' && self.board[0][2] == 'o' {
            'o'
        } else if self.board[1][0] == 'o' && self.board[1][1] == 'o' && self.board[1][2] == 'o' {
            'o'
        } else if self.board[2][0] == 'o' && self.board[2][1] == 'o' && self.board[2][2] == 'o' {
            'o'
        } else if self.board[0][0] == 'o' && self.board[1][0] == 'o' && self.board[2][0] == 'o' {
            'o'
        } else if self.board[0][1] == 'o' && self.board[1][1] == 'o' && self.board[2][1] == 'o' {
            'o'
        } else if self.board[0][2] == 'o' && self.board[1][2] == 'o' && self.board[2][2] == 'o' {
            'o'
        } else if self.board[0][0] == 'o' && self.board[1][1] == 'o' && self.board[2][2] == 'o' {
            'o'
        } else if self.board[0][2] == 'o' && self.board[1][1] == 'o' && self.board[2][0] == 'o' {
            'o'
        }
         else {
            '#'
        }
    }
}