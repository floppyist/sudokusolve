use std::env;
use std::{thread, time, process};
use std::collections::HashSet;

const CLI_RED: &str = "\x1b[31m";
const CLI_GREEN: &str = "\x1b[32m";
const CLI_YELLOW: &str = "\x1b[33m";
const CLI_BOLD: &str = "\x1b[1m";
const CLI_RESET: &str = "\x1b[0m";

#[derive(Debug)]
enum SudokuError {
    ImpossibleToSolve,
}

struct Sudoku {
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn is_available(&self, row: usize, col: usize, val: u8) -> bool {
        /* Determine upper left corner of the current chunk */
        let row_chunk_pointer = (row / 3) * 3;
        let col_chunk_pointer = (col / 3) * 3; 

        for idx in 0..9 {
            /* Row values */
            if self.grid[row][idx] == val {
                return false;
            }

            /* Col values */
            if self.grid[idx][col] == val {
                return false;
            }

            /* Chunk values */
            let current_row = row_chunk_pointer + (idx / 3);
            let current_col = col_chunk_pointer + (idx % 3);

            if self.grid[current_row][current_col] == val {
                return false;
            }
        }

        true
    }
}

trait Solvable {
    fn solve(&mut self, delay: time::Duration) -> Result<[[u8; 9]; 9], SudokuError>;
    fn print(&self);
}

impl Solvable for Sudoku {
    fn solve(&mut self, delay: time::Duration) -> Result<[[u8; 9]; 9], SudokuError> {
        if backtrack(self, 0, delay) {
            Ok(self.grid)
        } else {
            Err(SudokuError::ImpossibleToSolve)
        }
    }

    fn print(&self) {
        /* Move cursor to [0][0] to overwrite instead of syscall */
        print!("\x1b[H");

        for row in 0..self.grid.len() {
            for col in 0..self.grid.len() {
                if self.grid[row][col] == 0 {
                    print!("  ");
                } else {
                    print!("{} ", self.grid[row][col]);
                }

                if (col + 1) % 3 == 0 {
                    print!(" "); 
                }
            }

            println!();

            if (row + 1) % 3 == 0 {
                println!();
            }
        }
    }
}

fn backtrack(sudoku: &mut Sudoku, idx: usize, delay: time::Duration) -> bool {
    sudoku.print();

    thread::sleep(delay);

    if idx == 81 {
        return true;
    }

    let row: usize = idx / 9;
    let col: usize = idx % 9;
    let element = sudoku.grid[row][col];

    if element != 0 {
        return backtrack(sudoku, idx + 1, delay);
    }

    for i in 1..10 {
        if sudoku.is_available(row, col, i) {
            sudoku.grid[row][col] = i;

            if backtrack(sudoku, idx + 1, delay) {
                return true;
            }

            sudoku.grid[row][col] = 0;
        }
    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut delay: time::Duration = time::Duration::from_millis(0);

    if args.len() > 1 {
        match args[1].parse::<u64>() {
            Ok(v) => {
                delay = time::Duration::from_millis(v);
            }

            Err(e) => {
                println!("{CLI_BOLD}Error: {CLI_RED}{}{CLI_RESET}\n", e);
                println!("{CLI_BOLD}Usage:{CLI_RESET} ./sudokusolve <delay (u64)>");
                return;
            }
        }
    }

    /* Register handler for ^c */
    ctrlc::set_handler(move || {
        /* Show terminal cursor */
        println!("\x1b[?25h");

        process::exit(0);
    }).expect("Error setting abort handler.");

    /* Clear screen */
    print!("{}[2J", 27 as char);

    /* Hide cursor */
    print!("\x1b[?25l");

    let mut grids: Vec<[[u8; 9]; 9]> = Vec::new();

    /* Grid variants */
    let empty: [[u8; 9]; 9] = [[0; 9]; 9];

    let solvable: [[u8; 9]; 9] = [
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

    let unsolvable: [[u8; 9]; 9] = [
        [5, 1, 6, 8, 4, 9, 7, 3, 2],
        [3, 0, 7, 6, 0, 5, 0, 0, 0],
        [8, 0, 9, 7, 0, 0, 0, 6, 5],
        [1, 3, 5, 0, 6, 0, 9, 0, 7],
        [4, 7, 2, 5, 9, 1, 0, 0, 6],
        [9, 6, 8, 3, 7, 0, 0, 5, 0],
        [2, 5, 3, 1, 8, 6, 0, 7, 4],
        [6, 8, 4, 2, 0, 7, 5, 0, 0],
        [7, 9, 1, 0, 5, 0, 6, 0, 8],
    ];

    grids.push(empty);
    grids.push(solvable);
    grids.push(unsolvable);

    for g in grids {
        let mut sudoku = Sudoku { grid: g };

        match sudoku.solve(delay) {
            Ok(_) => {
                sudoku.print();
            }

            Err(e) => {
                println!("{CLI_RED}{:?}{CLI_RESET}", e);
            }
        }
    }

    /* Show terminal cursor */
    print!("\x1b[?25h");
}
