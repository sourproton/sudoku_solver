use std::{
    io::{stdin, BufRead},
    time::SystemTime,
};

fn main() {
    // Reading puzzle from `stdin`
    let mut sudoku = Sudoku::from_stdin();

    // Displaying input puzzle
    println!("\nPuzzle:\n");
    sudoku.show();
    println!();
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
        // Creating empty grid to populate with parsed input
        let mut grid: Vec<Vec<u8>> = vec![];

        // getting input from `stdin` and parsing it to `Vec`s of `u8`
        // ignores whitespace and empty lines (allows user to format input)
        stdin().lock().lines().for_each(|line| {
            let l = line.as_ref().expect("error reading input");
            if !l.is_empty() {
                grid.push(
                    line.expect("error reading input")
                        .split_whitespace()
                        .map(|i| i.parse().expect("error parsing input"))
                        .collect(),
                )
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
        for row in 0..=2 {
            println!(
                "{} {} {}  {} {} {}  {} {} {}",
                self.grid[row][0],
                self.grid[row][1],
                self.grid[row][2],
                self.grid[row][3],
                self.grid[row][4],
                self.grid[row][5],
                self.grid[row][6],
                self.grid[row][7],
                self.grid[row][8]
            );
        }
        println!();
        for row in 3..=5 {
            println!(
                "{} {} {}  {} {} {}  {} {} {}",
                self.grid[row][0],
                self.grid[row][1],
                self.grid[row][2],
                self.grid[row][3],
                self.grid[row][4],
                self.grid[row][5],
                self.grid[row][6],
                self.grid[row][7],
                self.grid[row][8]
            );
        }
        println!();
        for row in 6..=8 {
            println!(
                "{} {} {}  {} {} {}  {} {} {}",
                self.grid[row][0],
                self.grid[row][1],
                self.grid[row][2],
                self.grid[row][3],
                self.grid[row][4],
                self.grid[row][5],
                self.grid[row][6],
                self.grid[row][7],
                self.grid[row][8]
            );
        }
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
