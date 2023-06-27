from game import Game
import random

def f(board):
    l = []
    for i in board:
        l.append(tuple(i))
    return tuple(l)

class Player:
    def __init__(self, n):
        self.name = n
        self.q_table = {}

    def rand_play(self, game):
        v = []
        for i in range(3):
            for j in range(3):
                if game.board[i][j] == '#':
                    v.append((i,j))

        if len(v) != 0:
            index = random.randint(0, len(v)-1)
            row, column = v[index]
            game.play(row, column, self.name)

    def next_moves(self, game):
        board = game.board.copy()
        v = []
        for i in range(3):
            for j in range(3):
                if game.board[i][j] == '#':
                    board[i][j] = self.name
                    v.append(board)
                    board = game.board.copy()
        return v

    def other_player(self):
        if self.name == 'x':
            return 'o'
        else:
            return 'x'

    def train_play(self, game):
        moves = self.next_moves(game)

        for b in moves:
            game2 = Game()
            game2.board = b
            if game2.winner() == self.name:
                self.q_table[f(b)] = 1
                self.q_table[f(game.board)] = 1
            elif game2.winner() == self.other_player():
                self.q_table[f(b)] = -1
            elif game2.isgameover() and game2.winner() == '#':
                self.q_table[f(b)] = -1
            else:
                val = self.q_table.get(f(b))
                self.q_table[f(b)] = 0 if val == None else val

        max = -1
        for b in moves:
            a = self.q_table.get(f(b))
            a = 0 if a == None else a
            if a > max:
                max = a

        self.q_table[f(game.board)] = max

        game.board = moves[random.randint(0, len(moves)-1)]

    def play(self, game):
        moves = self.next_moves(game)
        if len(moves) == 1:
            game.board = moves[0]
        else:
            board = moves[0]
            val = self.q_table.get(f(board))
            val = 0 if val == None else val
            for i in range(1, len(moves) - 1):
                temp_board = moves[i]
                temp_val = self.q_table.get(f(temp_board))
                temp_val = 0 if temp_val == None else temp_val
                if temp_val > val:
                    val = temp_val
                    board = temp_board
                
            game.board = board
