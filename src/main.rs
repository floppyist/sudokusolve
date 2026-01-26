use std::collections::HashSet;

#[derive(Debug)]
enum SudokuError {
    ImpossibleToSolve,
}

struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn get_available_numbers(&self, x_idx: usize, y_idx: usize) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut working: HashSet<u8> = HashSet::new();

        // Get chunk values
        let x_chunk = x_idx / 3;
        let y_chunk = y_idx / 3;

        // Get first chunk item (upper left corner)
        let mut x_chunk_start = x_chunk * 3;
        let mut y_chunk_start = y_chunk * 3;

        for idx in 0..9 {
            let x_val = self.grid[x_idx][idx];
            let y_val = self.grid[idx][y_idx];

            // Col
            if x_val != 0 {
                working.insert(x_val);
            }

            // Row
            if y_val != 0 {
                working.insert(y_val);
            }

            // Chunk
            let val = self.grid[x_chunk_start][y_chunk_start];

            if val != 0 {
                working.insert(val);
            }

            if (idx + 1) % 3 == 0 {
                x_chunk_start -= 2;
                y_chunk_start += 1;
            } else {
                x_chunk_start += 1;
            }
        }

        // Invert
        for idx in 1..10 {
            if working.iter().any(|&e| e == idx) {
                continue;
            }

            result.push(idx);
        }

        result
    }
}

trait Solvable {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError>;
}

impl Solvable for Sudoku {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError> {
        if backtrack(self, 0) {
            Ok(self.grid)
        } else {
            Err(SudokuError::ImpossibleToSolve)
        }
    }
}

fn backtrack(sudoku: &mut Sudoku, idx: usize) -> bool {
    if idx == 81 {
        return true;
    }

    let x: usize = idx % 9;
    let y: usize = idx / 9;
    let element = sudoku.grid[y][x];

    if element != 0 {
        return backtrack(sudoku, idx + 1);
    }

    let available = sudoku.get_available_numbers(y, x);

    for i in available {
        // DO value
        sudoku.grid[y][x] = i;

        // Note successfull run
        if backtrack(sudoku, idx + 1) {
            return true;
        }

        // UNDO value
        sudoku.grid[y][x] = 0;
    }

    false
}

fn main() {
    let grid: [[u8; 9]; 9] = [
        [1, 0, 0, 0, 0, 7, 0, 9, 0],
        [0, 3, 0, 0, 2, 0, 0, 0, 8],
        [0, 0, 9, 6, 0, 0, 5, 0, 0],
        [0, 0, 5, 3, 0, 0, 9, 0, 0],
        [0, 1, 0, 0, 8, 0, 0, 0, 2],
        [6, 0, 0, 0, 0, 4, 0, 0, 0],
        [3, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 4, 1, 0, 0, 0, 0, 0, 7],
        [0, 0, 7, 0, 0, 0, 3, 0, 0],
    ];

    let mut sudoku = Sudoku { grid };

    println!("{:?}", sudoku.solve().unwrap());
}
