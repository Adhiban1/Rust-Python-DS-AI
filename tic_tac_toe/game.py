import os

class Game:
    def __init__(self):
        self.board = [
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#'],
        ]
        self.player = 'x'

    def print_board(self):
        os.system('cls')
        for i in self.board:
            for j in i:
                print(j, end='');
            print()

    def get_play_input(self):
        row, column = tuple(map(int, input("Move: ").strip().split(',')))
        self.play(row, column);

    def play(self, row, column):
        if self.player == 'x':
            self.board[row-1][column-1] = 'x'
            self.player = 'o'
        else:
            self.board[row-1][column-1] = 'o'
            self.player = 'x'