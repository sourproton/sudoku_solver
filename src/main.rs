use std::{
    io::{stdin, Read},
    time::SystemTime,
};

fn main() {
    // Reading puzzle from `stdin`
    let mut sudoku = Sudoku::from_stdin();

    // Displaying input puzzle
    println!("\nInput:\n");
    sudoku.show();
    println!();

    // Solving puzzle (modifies `sudoku`)
    let time = SystemTime::now();
    let solved = sudoku.solve(0, 0);
    let elapsed = time.elapsed().unwrap().as_nanos();

    // Displaying output
    if solved {
        if elapsed < 1_000 {
            println!("Solution found in {} ns:\n", elapsed);
        } else if elapsed < 1_000_000 {
            println!("Solution found in {} µs:\n", elapsed / 1_000);
        } else {
            println!("Solution found in {} ms:\n", elapsed / 1_000_000);
        }
        sudoku.show();
    } else {
        println!("No solution found");
    }
}

/// Holds the `Sudoku`'s `grid` and its methods
struct Sudoku {
    grid: Vec<Vec<u8>>,
}

impl Sudoku {
    /// Builds a `Sudoku` from stdin
    fn from_stdin() -> Self {
        // Creating 9x9 grid of zeros to populate with parsed input
        let mut grid: Vec<Vec<u8>> = vec![vec![0; 9]; 9];

        // getting input from `stdin` and parsing it to `Vec`s of `u8`
        // ignores whitespace and empty lines (allows user to format input)
        let mut input = String::new();

        stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("Couldn't read input");

        input
            .chars()
            .filter(|c| *c != ' ' && *c != '\n')
            .enumerate()
            .for_each(|(i, c)| {
                if c != '.' {
                    grid[i / 9][i % 9] = c.to_digit(10).expect("invalid digit in input") as u8;
                }
            });

        Sudoku { grid }
    }

    /// Checks if `n` is possible at position `[row][col]`
    fn is_possible(&self, row: usize, col: usize, n: u8) -> bool {
        // check row
        for i in 0..=8 {
            if self.grid[row][i] == n {
                return false;
            }
        }

        // check column
        for i in 0..=8 {
            if self.grid[i][col] == n {
                return false;
            }
        }

        // check subgrid
        let r0 = (row / 3) * 3;
        let c0 = (col / 3) * 3;
        for r in r0..=r0 + 2 {
            for c in c0..=c0 + 2 {
                if self.grid[r][c] == n {
                    return false;
                }
            }
        }

        true
    }

    /// Solves the `Sudoku`, starting the iteration at position `[row][col]`, returning
    /// `true` if succeeded and `false` otherwise
    fn solve(&mut self, row: usize, col: usize) -> bool {
        // if solved
        if row == 9 {
            return true;
        }

        let next_row;
        let next_col;

        if col == 8 {
            next_col = 0;
            next_row = row + 1;
        } else {
            next_col = col + 1;
            next_row = row;
        }

        // skip if this square already has a value
        if self.grid[row][col] != 0 {
            return self.solve(next_row, next_col);
        }

        // try all possible combinations for this square
        for n in 1..=9 {
            if self.is_possible(row, col, n) {
                self.grid[row][col] = n;

                if self.solve(next_row, next_col) {
                    return true;
                }
            }

            // back tracking
            self.grid[row][col] = 0;
        }

        // unsolvable
        false
    }

    /// Prints the solved sudoku to the `stdout`
    fn show(&self) {
        self.grid.iter().flatten().enumerate().for_each(|(i, n)| {
            // spacing
            if i % 9 != 0 {
                print!(" ");
            }
            // element
            if *n == 0 {
                print!(".");
            } else {
                print!("{n}");
            }

            // col separator
            if i % 9 == 2 || i % 9 == 5 {
                print!(" ");
            }

            // change line
            if (i + 1) % 9 == 0 {
                println!();

                // line separator
                if i / 9 == 2 || i / 9 == 5 {
                    println!();
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_possible() {
        let my_sudoku = Sudoku {
            grid: vec![
                vec![3, 7, 0, 8, 6, 0, 0, 1, 2],
                vec![6, 0, 0, 9, 0, 0, 8, 0, 7],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
                vec![0, 8, 3, 7, 2, 0, 4, 5, 0],
                vec![5, 4, 0, 0, 0, 6, 1, 0, 0],
                vec![2, 6, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 0, 9, 0, 8, 0],
                vec![1, 0, 0, 0, 8, 0, 0, 0, 5],
                vec![8, 2, 6, 5, 4, 0, 3, 9, 0],
            ],
        };

        assert!(my_sudoku.is_possible(0, 2, 9));
    }

    #[test]
    fn test_solve() {
        let mut my_sudoku = Sudoku {
            grid: vec![
                vec![3, 7, 0, 8, 6, 0, 0, 1, 2],
                vec![6, 0, 0, 9, 0, 0, 8, 0, 7],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
                vec![0, 8, 3, 7, 2, 0, 4, 5, 0],
                vec![5, 4, 0, 0, 0, 6, 1, 0, 0],
                vec![2, 6, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 2, 0, 9, 0, 8, 0],
                vec![1, 0, 0, 0, 8, 0, 0, 0, 5],
                vec![8, 2, 6, 5, 4, 0, 3, 9, 0],
            ],
        };

        assert!(my_sudoku.solve(0, 0));
    }
}
