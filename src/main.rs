use std::collections::HashSet;

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

struct SudokuState {
    result: [[u8; 9]; 9],
    path: Vec<u8>,
}

trait Solvable {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError>;
    fn print(&self);
}

impl Solvable for Sudoku {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError> {     
        let result = [[0; 9]; 9];
        let path: Vec<u8> = Vec::new();

        let mut state = SudokuState {
            result,
            path,
        };

        backtrack(self, 0, &mut state);

        Ok(self.grid)
    }

    fn print(&self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                print!("{} ", self.grid[x][y]);
            }

            println!();
        }
    }
}

fn backtrack(sudoku: &mut Sudoku, idx: usize, state: &mut SudokuState) -> bool {
    print!("\x1b[H");
    sudoku.print();

    if idx == 81 {
        for i in 0..state.path.len() {
            // Inflate array to 2d array
            state.result[i % 9][i / 9] = state.path[i];
        }

        return true;
    }

    let x: usize = idx % 9;
    let y: usize = idx / 9;
    let element = sudoku.grid[y][x];
    
    if element != 0 {
        return backtrack(sudoku, idx + 1, state);
    }

    let available = sudoku.get_available_numbers(y, x);

    for i in available {
        // DO value
        sudoku.grid[y][x] = i;

        if backtrack(sudoku, idx + 1, state) {
            return true;
        }

        // UNDO value
        sudoku.grid[y][x] = 0;
    }

    false
}

fn main() {
    print!("{}[2J", 27 as char);
    let grid: [[u8; 9]; 9] = [
        [0, 0, 8, 0, 0, 0, 0, 0, 0],
        [4, 9, 0, 1, 5, 7, 0, 0, 2],
        [0, 0, 3, 0, 0, 4, 1, 9, 0],
        [1, 8, 5, 0, 6, 0, 0, 2, 0],
        [0, 0, 0, 0, 2, 0, 0, 6, 0],
        [9, 6, 0, 4, 0, 5, 3, 0, 0],
        [0, 3, 0, 0, 7, 2, 0, 0, 4],
        [0, 4, 9, 0, 3, 0, 0, 5, 7],
        [8, 2, 7, 0, 0, 9, 0, 1, 3],
    ];

    let mut sudoku = Sudoku{
        grid,
    };

    let solved = sudoku.solve();
    sudoku.print();


    let x = 3;
    let y = 0;
    println!("[{}][{}] -> {:?} ({})", x, y, sudoku.get_available_numbers(x, y), sudoku.grid[x][y]);
}
