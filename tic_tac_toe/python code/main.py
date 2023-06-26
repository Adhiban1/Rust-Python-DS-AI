import os
from game import Game
from ai import Player

os.system('cls')
game = Game()
x = Player('x')
o = Player('o')
n = 10000
for i in range(1, n+1):
    game.reset()
    while True:
        x.train_play(game)
        if game.isgameover():
            break
        o.train_play(game)
        if game.isgameover():
            break
    print("\rTraining: {:.2f}% | x: {} | o: {}   ".format(i*100/n, len(x.q_table), len(o.q_table)), end='')

print("")
while True:
    print("----------------------------------------\nNew Game:")
    game.reset()
    while True:
        x.play(game)
        if game.isgameover():
            game.print()
            print(f"Winner: {game.winner()}\n")
            break

        print("\nAI:")
        game.print()
        
        game.human_play('o')
        if game.isgameover():
            game.print()
            print(f"Winner: {game.winner()}\n")
            break

        print("\nHuman:")
        game.print()
