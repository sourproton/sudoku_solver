# Sudoku solver

The goal of this project was to learn about [backtracking](https://en.wikipedia.org/wiki/Backtracking), the [wave function collapse](https://en.wikipedia.org/wiki/Wave_function_collapse#Use_in_procedural_generation) and to be able to feed a Rust program with a file via the standard input.

## Instalation

You can install this program with [cargo](https://github.com/rust-lang/cargo), which is available with [rustc](https://github.com/rust-lang/rust). I recommend [rustup](https://rustup.rs/) for managing your Rust installation.

1. Clone the repository

```
git clone https://github.com/sourproton/sudoku_solver
```

2. `cd` into it

```
cd sudoku_solver
```

3. Install it

```
cargo install --path .
```

4. Restart the terminal and check if the `sudoku_solver` command is available

```
which sudoku_solver
```

5. To uninstall, run

```
cargo uninstall sudoku_solver
```

## Usage

1. Save a file containing the sudoku puzzle to be solved. Spaces and new lines are ignored. Blanks must be entered as dots.

Example of formatted input (`examples/possible_formatted.txt`):

```
3 7 .  8 6 .  . 1 2
6 . .  9 . .  8 . 7
. . .  . . .  . . 3

. 8 3  7 2 .  4 5 .
5 4 .  . . 6  1 . .
2 6 .  . . .  . . .

. . .  2 . 9  . 8 .
1 . .  . 8 .  . . 5
8 2 6  5 4 .  3 9 .
```

Example of unformatted input (`examples/possible_unformatted.txt`):

```
..3..6....5.1.....6...234...7.....5....9....7.64.3.8...4.....91..2..83...........
```

Both forms are valid

2. Redirect the input to the `sudoku_solver` command

```
sudoku_solver < example/possible_formatted.txt
```

Note: another option is to write the puzzle directly into the terminal, without saving the input file, like that:

```
sudoku_solver <<< ..3..6....5.1.....6...234...7.....5....9....7.64.3.8...4.....91..2..83...........
```

3. The output is printed to the terminal

```
Input:

3 7 .  8 6 .  . 1 2
6 . .  9 . .  8 . 7
. . .  . . .  . . 3

. 8 3  7 2 .  4 5 .
5 4 .  . . 6  1 . .
2 6 .  . . .  . . .

. . .  2 . 9  . 8 .
1 . .  . 8 .  . . 5
8 2 6  5 4 .  3 9 .

Solution found in 19 µs:

3 7 9  8 6 4  5 1 2
6 1 2  9 3 5  8 4 7
4 5 8  1 7 2  9 6 3

9 8 3  7 2 1  4 5 6
5 4 7  3 9 6  1 2 8
2 6 1  4 5 8  7 3 9

7 3 5  2 1 9  6 8 4
1 9 4  6 8 3  2 7 5
8 2 6  5 4 7  3 9 1
```
