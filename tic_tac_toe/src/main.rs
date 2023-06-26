mod game;
mod ai;
use game::Game;
use ai::Player;
use std::io::Write;

fn main() {
    print!("\x1B[2J");
    let mut game = Game::new();
    let mut x = Player::new('x');
    let mut o = Player::new('o');
    let n = 10000;
    for i in 1..=n {
        game.reset();
        loop {
            x.train_play(&mut game);
            if game.isgameover() {
                break;
            }
            o.train_play(&mut game);
            if game.isgameover() {
                break;
            }
        }
        print!("\rTraining: {}% | x: {} | o: {}   ", i*100/n, x.q_table.len(), o.q_table.len());
        std::io::stdout().flush().unwrap();
    }
    println!("");
    loop {
        println!("----------------------------------------\nNew Game:");
        game.reset();
        loop {
            x.play(&mut game);
            if game.isgameover() {
                println!("\nAI:");
                game.print();
                println!("Winner: {}\n", game.winner());
                break;
            }

            println!("\nAI:");
            game.print();
            
            game.human_play('o');
            if game.isgameover() {
                println!("\nHuman: ");
                game.print();
                println!("Winner: {}\n", game.winner());
                break;
            }

            println!("\nHuman:");
            game.print();
        }
    }
}
