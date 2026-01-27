use std::env;
use std::{process, thread, time};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

#[derive(Debug)]
enum SudokuError {
    ImpossibleToSolve,
}

struct Sudoku {
    init: [[u8; 9]; 9],
    grid: [[u8; 9]; 9],
}

impl Sudoku {
    fn new(grid: [[u8; 9]; 9]) -> Self {
        Self {
            init: grid,
            grid,
        }
    }

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

struct SudokuOptions {
    print: bool,
    delay: time::Duration,
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
                } else if self.init[row][col] != 0 {
                    print!("{BOLD}{}{RESET} ", self.grid[row][col]);
                } else {
                    print!("{GREEN}{}{RESET} ", self.grid[row][col]);
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
    /* Just print if delay was set */
    if delay.as_millis() > 0 { sudoku.print() };

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

fn handle_args(args: &[String]) -> SudokuOptions {
    let opts = SudokuOptions {
        print: false,
        delay: time::Duration::from_millis(0),
    };

    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-g" | "--grid" => {

            }

            "-p" | "--print" => {

            }

            "-d" | "--delay" => {

            }

            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }

            a => {
                println!("[{RED}ERROR{RESET}]: Invalid parameter: {}", a);
            }
        }
    }

    opts
}

fn print_help() {
    println!("{BOLD}Usage: ./sudokusolve{RESET} [options]\n");

    println!("{BOLD}Example:{RESET}");
    println!("{BOLD} ./sudokusolve -p -d {RESET}250 {BOLD}-g{RESET} \"53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79\"\n");

    println!("{BOLD}Options:{RESET}");
    println!(" {BOLD}-g, --grid  <grid>{RESET} sets the grid to be solved");
    println!(" {BOLD}-p, --print       {RESET} if set every iteration will be visualized");
    println!(" {BOLD}-d, --delay <ms>  {RESET} sets the delay of every iteration");
    println!(" {BOLD}-h, --help        {RESET} shows this help dialog");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opts = handle_args(&args);

    /* Register handler for ^c */
    ctrlc::set_handler(move || {
        /* Show terminal cursor */
        println!("\x1b[?25h");

        process::exit(0);
    }).expect("Error setting abort handler.");

    /* Clear screen */
    print!("\x1b[2J");

    /* Hide cursor */
    print!("\x1b[?25l");

    /* Grid */
    let example: [[u8; 9]; 9] = [
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

    let mut sudoku = Sudoku::new(example);

    match sudoku.solve(opts.delay) {
        Ok(_) => {
            sudoku.print();
        }

        Err(e) => {
            println!("{RED}{:?}{RESET}", e);
        }
    }

    /* Show terminal cursor */
    print!("\x1b[?25h");
}
