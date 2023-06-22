mod game;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.print_board();

    loop {
        game.get_play_input();
        game.print_board();
        let winner = game.winner();
        println!("Winner: {}", winner);
        if winner != '#' {
            break;
        }
    }
}
