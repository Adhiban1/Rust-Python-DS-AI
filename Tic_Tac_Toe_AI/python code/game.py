import sys

class Game:
    def __init__(self):
        self.board = [
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#']
        ]

    def reset(self):
        self.board = [
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#']
        ]

    def print(self):
        for i in self.board:
            for j in i:
                print(f" {j} ", end='')
            print("")

    def play(self, row, column, name):
        self.board[row][column] = name

    def human_play(self, name):
        print("\nRow: ", end='')
        row = int(input()) - 1

        print("Column: ", end='')
        column = int(input()) - 1

        if self.board[row][column] == '#':
            self.board[row][column] = name
            print("")
        else:
            print("Illegal Move")
            sys.exit()

    def isgameover(self):
        if self.winner() != '#':
            return True
        else:
            over = True
            break_loop = False
            for i in self.board:
                for j in i:
                    if j == '#':
                        over = False
                        break_loop = True
                        break
                if break_loop:
                    break
            return over

    def winner(self):
        b = self.board
        if b[0][0] == 'x' and b[0][1] == 'x' and b[0][2] == 'x':
            return 'x'
        elif b[1][0] == 'x' and b[1][1] == 'x' and b[1][2] == 'x':
            return 'x'
        elif b[2][0] == 'x' and b[2][1] == 'x' and b[2][2] == 'x':
            return 'x'
        elif b[0][0] == 'x' and b[1][0] == 'x' and b[2][0] == 'x':
            return 'x'
        elif b[0][1] == 'x' and b[1][1] == 'x' and b[2][1] == 'x':
            return 'x'
        elif b[0][2] == 'x' and b[1][2] == 'x' and b[2][2] == 'x':
            return 'x'
        elif b[0][0] == 'x' and b[1][1] == 'x' and b[2][2] == 'x':
            return 'x'
        elif b[0][2] == 'x' and b[1][1] == 'x' and b[2][0] == 'x':
            return 'x'
        
        elif b[0][0] == 'o' and b[0][1] == 'o' and b[0][2] == 'o':
            return 'o'
        elif b[1][0] == 'o' and b[1][1] == 'o' and b[1][2] == 'o':
            return 'o'
        elif b[2][0] == 'o' and b[2][1] == 'o' and b[2][2] == 'o':
            return 'o'
        elif b[0][0] == 'o' and b[1][0] == 'o' and b[2][0] == 'o':
            return 'o'
        elif b[0][1] == 'o' and b[1][1] == 'o' and b[2][1] == 'o':
            return 'o'
        elif b[0][2] == 'o' and b[1][2] == 'o' and b[2][2] == 'o':
            return 'o'
        elif b[0][0] == 'o' and b[1][1] == 'o' and b[2][2] == 'o':
            return 'o'
        elif b[0][2] == 'o' and b[1][1] == 'o' and b[2][0] == 'o':
            return 'o'
        
        else:
            return '#'