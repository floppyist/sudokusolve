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
    /* TODO: Dirty code */
    fn get_available_numbers(&self, col: usize, row: usize) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut working: HashSet<u8> = HashSet::new();

        /* Get chunk values */
        let col_chunk = col / 3; 
        let row_chunk = row / 3;

        /* Get first chunk item (upper left corner) */
        let mut col_chunk_start = col_chunk * 3;
        let mut row_chunk_start = row_chunk * 3;

        for idx in 0..9 {
            let x_val = self.grid[col][idx];
            let y_val = self.grid[idx][row];

            /* Col */
            if x_val != 0 {
                working.insert(x_val);
            }

            /* Row */
            if y_val != 0 {
                working.insert(y_val);
            }

            /* Chunk */
            let val = self.grid[col_chunk_start][row_chunk_start];

            if val != 0 {
                working.insert(val);
            }

            if (idx + 1) % 3 == 0 {
                col_chunk_start -= 2;
                row_chunk_start += 1;
            } else {
                col_chunk_start += 1;
            }
        }

        /* Invert */
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

        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                print!("{} ", self.grid[y][x]);

                if (x + 1) % 3 == 0 {
                    print!(" "); 
                }
            }

            println!();

            if (y + 1) % 3 == 0 {
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

    let row: usize = idx % 9;
    let col: usize = idx / 9;
    let element = sudoku.grid[col][row];

    if element != 0 {
        return backtrack(sudoku, idx + 1, delay);
    }

    let available = sudoku.get_available_numbers(col, row);

    for i in available {
        /* DO */
        sudoku.grid[col][row] = i;

        /* Remember successful path */
        if backtrack(sudoku, idx + 1, delay) {
            return true;
        }

        /* UNDO */
        sudoku.grid[col][row] = 0;
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
