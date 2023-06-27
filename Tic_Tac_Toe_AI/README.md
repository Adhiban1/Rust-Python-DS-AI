# Tic Tac Toe AI

I recently completed a project where I used the minimax RL algorithm in Rust to create a game AI that could play Tic-Tac-Toe optimally. The minimax RL algorithm is a recursive algorithm used in decision-making and game theory to find the optimal move for a player, assuming that your opponent also plays optimally. It is widely used in two-player turn-based games such as Tic-Tac-Toe, Backgammon, Mancala, Chess, etc.

In my project, I used the **minimax RL algorithm** to create an agent that would explore the environment randomly. The agent would start by assigning a value of 0 to any state that it had not yet visited. Then, it would explore the environment by randomly making moves. After each move, the agent would update the value of the state that it had just visited to the **maximum value of the next possible states**. If the agent reached a state where it lost the game, it would assign that state to -1 in the Q-table. If it won the game, it would assign that state to 1 in the Q-table.

This process would continue until the agent had explored the entire environment. At that point, the agent would have a value for every state in the environment. The agent could then use these values to make optimal moves in the game.

Rust is a modern programming language that is designed to be safe, fast, and expressive. It is a good choice for projects that require high performance and reliability. Rust is also a relatively new language, so it is still under development. This means that there are many opportunities to contribute to the Rust community and help shape the future of the language.

I am very happy with the results of my project. I was able to learn a lot about the minimax RL algorithm and Rust. I am excited to continue learning about Rust and using it to create more projects in the future.

*Here are some of the advantages of Rust:*

**Safety:** Rust is a statically typed language, which means that the compiler can check for errors at compile time. This makes Rust programs more reliable and less likely to crash.

**Speed:** Rust is a compiled language, which means that it is typically faster than interpreted languages like Python or JavaScript.

**Expressiveness:** Rust is a powerful language that allows you to express complex ideas in a concise and elegant way.

**Concurrency:** Rust has built-in support for concurrency, which makes it easy to write parallel programs.

**Memory safety:** Rust's memory management system is designed to prevent memory leaks and other memory-related errors. This is important for game AI, as games often need to store large amounts of data.

I am also writing code in Python to do this same project. I am interested in comparing the performance of the two languages for this type of application. I am also curious to see how the different programming styles of Rust and Python affect the development process.

I think it is great that you are writing the code in Python as well. It will be interesting to see how the two languages compare in terms of performance and development speed. I am also curious to see how the different programming styles of Rust and Python affect the code.

I am sure that you will learn a lot from this project, no matter which language you choose. I am excited to see how your project turns out!

## Python Code

```python
class Game:
    def __init__(self):
        self.board = [
            ['#', '#', '#'],
            ['#', '#', '#'],
            ['#', '#', '#']
        ]
    # Loading...
```

> Loading...

**I am still typing Python code. I will type it soon and share it with you.**

## Rust Code

### main.rs

> Training the agents:

```rust
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
}
```

> Agent vs Human play:

```rust
loop {
    game.reset();
    loop {
        x.play(&mut game);
        if game.isgameover() {
            break;
        }
        game.human_play('o');
        if game.isgameover() {
            break;
        }
    }
}
```

### Game

|Function|Description|
|---|---|
|reset|It resets the board into initial state|
|print|It prints the board|
|play|This gets row:usize, column:usize and name:char and assign the name to row and column of the board|
|human_play|It gets the row and column value from human|
|isgameover|It return true or false|
|winner|It returns 'x' or 'o' or '#' (Draw)|

### Player

|Function|Description|
|---|---|
|rand_play|Agent playes a random move|
|next_moves (private)|It returns all possible next states|
|other_player (private)|It return other player name|
|train_play|Agent plays a random move and updates Q-table|
|play|Agent plays the best move by seeing Q-table|

## Output:

```
Training: 100% | x: 5159 | o: 4835
----------------------------------------
New Game:

AI:
 x  #  #
 #  #  #
 #  #  #

Row: 2
Column: 2


Human:
 x  #  #
 #  o  #
 #  #  #

AI:
 x  x  #
 #  o  #
 #  #  #

Row:
```

*I am a beginner in Rust, and as such, this code may not be fully optimized or may not work on all platforms. Additionally, there is a possibility that the code may contain errors that could lead to inaccurate results in the AI's performance.*