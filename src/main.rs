use std::{
    io::{stdin, Read},
    time::SystemTime,
};

fn main() {
    // Reading puzzle from `stdin`
    let mut sudoku = Sudoku::from_stdin();

    // Displaying input puzzle
    println!("\nInput:\n");
    sudoku.show_values();
    println!();

    // Solving puzzle (modifies `sudoku`)
    let time = SystemTime::now();
    let solved = sudoku.solve();
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
        sudoku.show_values();
    } else {
        println!("No solution found");
    }
}

/// Holds the `Sudoku`'s `values`, the `possible_values` for each non-collapsed cell, and its methods
struct Sudoku {
    values: Vec<u8>,
    possible_values: Vec<Vec<u8>>,
}

impl Sudoku {
    /// Builds a `Sudoku` from stdin
    fn from_stdin() -> Self {
        // getting input from `stdin` and parsing it
        // ignores whitespace and empty lines (allows user to format input)
        let mut input = String::new();

        stdin()
            .lock()
            .read_to_string(&mut input)
            .expect("Couldn't read input");

        // Creating vec of zeros to populate with parsed input
        let mut values = vec![0; 81];

        input
            .chars()
            .filter(|c| *c != ' ' && *c != '\n')
            .enumerate()
            .for_each(|(i, c)| {
                if c != '.' {
                    values[i] = c.to_digit(10).expect("invalid digit in input") as u8;
                }
            });

        Self::from_values(values)
    }

    /// Builds a `Sudoku` from a vector of values
    fn from_values(values: Vec<u8>) -> Self {
        // default sudoku with all zeros and every superposition
        let mut sudoku = Sudoku {
            values: vec![0; 81],
            possible_values: vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]; 81],
        };

        // read input and collapse for each value
        values
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != 0)
            .for_each(|(i, v)| {
                sudoku.collapse(i, *v);
            });

        sudoku
    }

    /// Solves the `Sudoku`, collapsing the cell with lowest entropy, returning
    /// `true` if succeeded and `false` otherwise
    fn solve(&mut self) -> bool {
        if let Some(index) = self.lowest_entropy() {
            // try all possible combinations for this square
            // for value in 1..=9 {
            //     if self.is_possible(index, value) {
            for value in self.possible_values[index].clone() {
                self.collapse(index, value);

                if self.solve() {
                    return true;
                }

                // back tracking
                self.collapse(index, 0);
                // }
            }

            // unsolvable
            false
        } else {
            // `lowest_entropy` couldn't find any non-zero cell, so the grid is solved
            true
        }
    }

    /// Collapse cell `index` and propagates the information to its row, col and box
    fn collapse(&mut self, index: usize, value: u8) {
        let row = index / 9;
        let col = index % 9;

        // box limits
        let r0 = (row / 3) * 3;
        let r1 = r0 + 2;
        let c0 = (col / 3) * 3;
        let c1 = c0 + 2;

        // backtracking
        // set value back to 0 and push the old value back to the cells where it's possible
        if value == 0 {
            let old_value = self.values[index];
            self.values[index] = value;

            // box
            for row in r0..=r1 {
                for col in c0..=c1 {
                    if self.is_possible(row * 9 + col, old_value) {
                        self.possible_values[row * 9 + col].push(old_value);
                    }
                }
            }

            // row
            for col in 0..c0 {
                if self.is_possible(row * 9 + col, old_value) {
                    self.possible_values[row * 9 + col].push(old_value);
                }
            }
            for col in c1 + 1..9 {
                if self.is_possible(row * 9 + col, old_value) {
                    self.possible_values[row * 9 + col].push(old_value);
                }
            }

            // column
            for row in 0..r0 {
                if self.is_possible(row * 9 + col, old_value) {
                    self.possible_values[row * 9 + col].push(old_value);
                }
            }
            for row in r1 + 1..9 {
                if self.is_possible(row * 9 + col, old_value) {
                    self.possible_values[row * 9 + col].push(old_value);
                }
            }
        }
        // propagating
        // set cell to value and remove value from possibility of other cells in row, col and box
        else {
            self.values[index] = value;

            // box
            for row in r0..=r1 {
                for col in c0..=c1 {
                    if let Some(pos) = self.possible_values[row * 9 + col]
                        .iter()
                        .position(|v| *v == value)
                    {
                        self.possible_values[row * 9 + col].swap_remove(pos);
                    }
                }
            }

            // row
            for col in 0..c0 {
                if let Some(pos) = self.possible_values[row * 9 + col]
                    .iter()
                    .position(|v| *v == value)
                {
                    self.possible_values[row * 9 + col].swap_remove(pos);
                }
            }
            for col in c1 + 1..9 {
                if let Some(pos) = self.possible_values[row * 9 + col]
                    .iter()
                    .position(|v| *v == value)
                {
                    self.possible_values[row * 9 + col].swap_remove(pos);
                }
            }

            // column
            for row in 0..r0 {
                if let Some(pos) = self.possible_values[row * 9 + col]
                    .iter()
                    .position(|v| *v == value)
                {
                    self.possible_values[row * 9 + col].swap_remove(pos);
                }
            }
            for row in r1 + 1..9 {
                if let Some(pos) = self.possible_values[row * 9 + col]
                    .iter()
                    .position(|v| *v == value)
                {
                    self.possible_values[row * 9 + col].swap_remove(pos);
                }
            }
        }
    }

    /// Wheter or not is it possible for cell `index` to have value `value`
    fn is_possible(&self, index: usize, value: u8) -> bool {
        let row = index / 9;
        let col = index % 9;

        // box limits
        let r0 = (row / 3) * 3;
        let c0 = (col / 3) * 3;

        // box
        for r in r0..=r0 + 2 {
            for c in c0..=c0 + 2 {
                if self.values[r * 9 + c] == value {
                    return false;
                }
            }
        }

        // row
        for c in 0..c0 {
            if self.values[row * 9 + c] == value {
                return false;
            }
        }
        for c in c0 + 3..9 {
            if self.values[row * 9 + c] == value {
                return false;
            }
        }

        // column
        for r in 0..r0 {
            if self.values[r * 9 + col] == value {
                return false;
            }
        }
        for r in r0 + 3..9 {
            if self.values[r * 9 + col] == value {
                return false;
            }
        }

        true
    }

    /// Finds cell with lowest entropy
    fn lowest_entropy(&self) -> Option<usize> {
        let index = self
            .values
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == 0)
            .min_by(|x, y| {
                self.possible_values[x.0]
                    .len()
                    .cmp(&self.possible_values[y.0].len())
            });

        index.map(|x| x.0)
    }

    /// Prints the sudoku values to the `stdout`
    fn show_values(&self) {
        self.values.iter().enumerate().for_each(|(i, n)| {
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

    impl Sudoku {
        /// Prints the sudoku possible values to the `stdout`
        fn show_possible_values(&self) {
            self.possible_values.iter().enumerate().for_each(|(i, v)| {
                println!("{i}: {v:?}");
            })
        }
    }

    #[test]
    fn test_solve_full() {
        let values = vec![
            3, 7, 0, 8, 6, 0, 0, 1, 2, 6, 0, 0, 9, 0, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 8,
            3, 7, 2, 0, 4, 5, 0, 5, 4, 0, 0, 0, 6, 1, 0, 0, 2, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
            0, 9, 0, 8, 0, 1, 0, 0, 0, 8, 0, 0, 0, 5, 8, 2, 6, 5, 4, 0, 3, 9, 0,
        ];

        let mut sudoku = Sudoku::from_values(values);

        assert!(sudoku.solve());

        sudoku.show_values();
    }

    #[test]
    fn test_solve_impossible() {
        let values = vec![
            0, 0, 3, 0, 0, 6, 0, 0, 0, 0, 5, 0, 1, 0, 0, 0, 0, 0, 6, 0, 0, 0, 2, 3, 4, 0, 0, 0, 7,
            0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 9, 0, 0, 0, 0, 7, 0, 6, 4, 0, 3, 0, 8, 0, 0, 0, 4, 0, 0,
            0, 0, 0, 9, 1, 0, 0, 2, 0, 0, 8, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        let mut sudoku = Sudoku::from_values(values);

        assert!(!sudoku.solve());

        sudoku.show_values();
    }

    #[test]
    fn test_solve_empty() {
        let values = vec![0; 81];

        let mut sudoku = Sudoku::from_values(values);

        assert!(sudoku.solve());

        sudoku.show_values();
    }
}
