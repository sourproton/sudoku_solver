# Sudoku solver

The goal of this project was to learn about [backtracking](https://en.wikipedia.org/wiki/Backtracking) and to be able to feed a Rust program with a file via the standard input.

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

1. Save a file containing the sudoku puzzle to be solved. Each square must be separated by at least one white space and blanks should be typed as zeros. Blank lines are ignored (should not contain any white space). Example (`examples/possible.txt`):

```
3 7 0  8 6 0  0 1 2
6 0 0  9 0 0  8 0 7
0 0 0  0 0 0  0 0 3

0 8 3  7 2 0  4 5 0
5 4 0  0 0 6  1 0 0
2 6 0  0 0 0  0 0 0

0 0 0  2 0 9  0 8 0
1 0 0  0 8 0  0 0 5
8 2 6  5 4 0  3 9 0
```

2. Redirect the file to the `sudoku_solver` command

```
sudoku_solver < example/possible.txt
```

3. The output is printed to the terminal

```
Puzzle:

3 7 0  8 6 0  0 1 2
6 0 0  9 0 0  8 0 7
0 0 0  0 0 0  0 0 3

0 8 3  7 2 0  4 5 0
5 4 0  0 0 6  1 0 0
2 6 0  0 0 0  0 0 0

0 0 0  2 0 9  0 8 0
1 0 0  0 8 0  0 0 5
8 2 6  5 4 0  3 9 0


Solution found in 39 µs:

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
