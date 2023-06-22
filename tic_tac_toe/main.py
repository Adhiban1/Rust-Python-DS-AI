from game import Game

game = Game()
game.print_board()

while True:
    game.get_play_input()
    game.print_board()
