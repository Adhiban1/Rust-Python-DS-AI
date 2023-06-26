use std::io::Write;

pub struct Game {
    pub board:[[char;3];3]
}

impl Game {
    pub fn new() -> Self {
        Self {board:[
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#']
        ]}
    }

    #[allow(unused)]
    pub fn reset(&mut self) {
        self.board = [
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#']
        ];
    }


    pub fn print(&self) {
        for i in self.board {
            for j in i {
                print!(" {} ", j);
            }
            println!("");
        }
    }

    pub fn play(&mut self, row:usize, column:usize, name:char) {
        self.board[row][column] = name;
    }

    pub fn human_play(&mut self, name:char) {
        print!("\nRow: ");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let row:usize = s.trim().parse::<usize>().unwrap() - 1;

        print!("Column: ");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let column:usize = s.trim().parse::<usize>().unwrap() - 1;

        if self.board[row][column] == '#' {
            self.board[row][column] = name;
            println!("");
        } else {
            panic!("Illegal Move");
        }
        
    }

    pub fn isgameover(&self) -> bool {
        if self.winner() != '#' {
            true
        } else {
            let mut over = true;
            let mut break_loop = false;
            for i in self.board {
                for j in i {
                    if j == '#' {
                        over = false;
                        break_loop = true;
                        break;
                    }
                }
                if break_loop {
                    break;
                }
            }
            over
        }
    }

    pub fn winner(&self) -> char {
        let b = self.board;
        if b[0][0] == 'x' && b[0][1] == 'x' && b[0][2] == 'x' {
            'x'
        } else if b[1][0] == 'x' && b[1][1] == 'x' && b[1][2] == 'x' {
            'x'
        } else if b[2][0] == 'x' && b[2][1] == 'x' && b[2][2] == 'x' {
            'x'
        } else if b[0][0] == 'x' && b[1][0] == 'x' && b[2][0] == 'x' {
            'x'
        } else if b[0][1] == 'x' && b[1][1] == 'x' && b[2][1] == 'x' {
            'x'
        } else if b[0][2] == 'x' && b[1][2] == 'x' && b[2][2] == 'x' {
            'x'
        } else if b[0][0] == 'x' && b[1][1] == 'x' && b[2][2] == 'x' {
            'x'
        } else if b[0][2] == 'x' && b[1][1] == 'x' && b[2][0] == 'x' {
            'x'
        } 
        
        else if b[0][0] == 'o' && b[0][1] == 'o' && b[0][2] == 'o' {
            'o'
        } else if b[1][0] == 'o' && b[1][1] == 'o' && b[1][2] == 'o' {
            'o'
        } else if b[2][0] == 'o' && b[2][1] == 'o' && b[2][2] == 'o' {
            'o'
        } else if b[0][0] == 'o' && b[1][0] == 'o' && b[2][0] == 'o' {
            'o'
        } else if b[0][1] == 'o' && b[1][1] == 'o' && b[2][1] == 'o' {
            'o'
        } else if b[0][2] == 'o' && b[1][2] == 'o' && b[2][2] == 'o' {
            'o'
        } else if b[0][0] == 'o' && b[1][1] == 'o' && b[2][2] == 'o' {
            'o'
        } else if b[0][2] == 'o' && b[1][1] == 'o' && b[2][0] == 'o' {
            'o'
        } 
        
        else {
            '#'
        }
    }
}