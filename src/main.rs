fn main() {
    let mut my_sudoku = Sudoku::new(vec![
        vec![3, 7, 0, 8, 6, 0, 0, 1, 2],
        vec![6, 0, 0, 9, 0, 0, 8, 0, 7],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![0, 8, 3, 7, 2, 0, 4, 5, 0],
        vec![5, 4, 0, 0, 0, 6, 1, 0, 0],
        vec![2, 6, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 2, 0, 9, 0, 8, 0],
        vec![1, 0, 0, 0, 8, 0, 0, 0, 5],
        vec![8, 2, 6, 5, 4, 0, 3, 9, 0],
    ]);

    my_sudoku.solve(0, 0);

    for row in my_sudoku.grid {
        let to_print: String = row.iter().map(|n| n.to_string()).collect();
        println!("{to_print}");
    }
}

/// Holds the `Sudoku` ` grid` and its methods
struct Sudoku {
    grid: Vec<Vec<u8>>,
}

impl Sudoku {
    /// Builds a `Sudoku` from a vector of vectors
    fn new(grid: Vec<Vec<u8>>) -> Self {
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
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_possible() {
        let my_sudoku = Sudoku::new(vec![
            vec![3, 7, 0, 8, 6, 0, 0, 1, 2],
            vec![6, 0, 0, 9, 0, 0, 8, 0, 7],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![0, 8, 3, 7, 2, 0, 4, 5, 0],
            vec![5, 4, 0, 0, 0, 6, 1, 0, 0],
            vec![2, 6, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0, 9, 0, 8, 0],
            vec![1, 0, 0, 0, 8, 0, 0, 0, 5],
            vec![8, 2, 6, 5, 4, 0, 3, 9, 0],
        ]);

        assert!(my_sudoku.is_possible(0, 2, 9));
    }

    #[test]
    fn test_solve() {
        let mut my_sudoku = Sudoku::new(vec![
            vec![3, 7, 0, 8, 6, 0, 0, 1, 2],
            vec![6, 0, 0, 9, 0, 0, 8, 0, 7],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![0, 8, 3, 7, 2, 0, 4, 5, 0],
            vec![5, 4, 0, 0, 0, 6, 1, 0, 0],
            vec![2, 6, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 2, 0, 9, 0, 8, 0],
            vec![1, 0, 0, 0, 8, 0, 0, 0, 5],
            vec![8, 2, 6, 5, 4, 0, 3, 9, 0],
        ]);

        assert!(my_sudoku.solve(0, 0));
    }
}
