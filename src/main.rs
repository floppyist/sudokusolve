use std::env;
use std::{process, thread, time};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BOLD: &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

const UPPER_LEFT: &str = "╔";
const UPPER_RIGHT: &str = "╗";
const BOTTOM_LEFT: &str = "╚";
const BOTTOM_RIGHT: &str = "╝";
const HORIZONTAL: &str = "═";
const HORIZONTAL_TOP: &str = "╩";
const HORIZONTAL_BOTTOM: &str = "╦";
const VERTICAL: &str = "║";
const VERTICAL_LEFT: &str = "╣";
const VERTICAL_RIGHT: &str = "╠";
const CROSS: &str = "╬";

#[derive(Debug)]
enum SudokuError {
    ImpossibleToSolve,
}

struct Sudoku {
    grid: [[u8; 9]; 9],
    opts: SudokuOptions,
}

impl Sudoku {
    fn new(opts: SudokuOptions) -> Self {
        Self {
            grid: opts.initial_grid,
            opts,
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
    measure: bool,
    initial_grid: [[u8; 9]; 9],
}

trait Solvable {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError>;
    fn print(&self);
}

impl Solvable for Sudoku {
    fn solve(&mut self) -> Result<[[u8; 9]; 9], SudokuError> {
        if backtrack(self, 0, self.opts.delay, self.opts.print) {
            Ok(self.grid)
        } else {
            Err(SudokuError::ImpossibleToSolve)
        }
    }

    fn print(&self) {
        /* Move cursor to [0][0] to overwrite instead of syscall */
        print!("\x1b[H");

        println!(
            "{UPPER_LEFT}{0}{HORIZONTAL_BOTTOM}{0}{HORIZONTAL_BOTTOM}{0}{UPPER_RIGHT}",
            HORIZONTAL.repeat(7)
        );

        for row in 0..self.grid.len() {
            for col in 0..self.grid.len() {
                if col == 0 {
                    print!("{VERTICAL} ");
                }

                if self.opts.initial_grid[row][col] != 0 {
                    print!("{BOLD}{RED}{}{RESET} ", self.grid[row][col]);
                } else if self.grid[row][col] != 0 {
                    print!("{GREEN}{}{RESET} ", self.grid[row][col]);
                } else {
                    print!("  ");
                }

                if (col + 1) % 3 == 0 {
                    print!("{VERTICAL} ");
                }
            }

            println!();

            if (row + 1) % 3 == 0 && row != 8 {
                println!(
                    "{VERTICAL_RIGHT}{0}{CROSS}{0}{CROSS}{0}{VERTICAL_LEFT}",
                    HORIZONTAL.repeat(7)
                );
            }
        }

        println!(
            "{BOTTOM_LEFT}{0}{HORIZONTAL_TOP}{0}{HORIZONTAL_TOP}{0}{BOTTOM_RIGHT}",
            HORIZONTAL.repeat(7)
        );
    }
}

fn backtrack(sudoku: &mut Sudoku, idx: usize, delay: time::Duration, show: bool) -> bool {
    /* Just print if print flag was set */
    if show {
        sudoku.print()
    };

    /* Only use thread sleep if delay was set */
    if delay.as_millis() > 0 {
        thread::sleep(delay)
    };

    if idx == 81 {
        return true;
    }

    let row: usize = idx / 9;
    let col: usize = idx % 9;
    let element = sudoku.grid[row][col];

    if element != 0 {
        return backtrack(sudoku, idx + 1, delay, show);
    }

    for i in 1..10 {
        if sudoku.is_available(row, col, i) {
            sudoku.grid[row][col] = i;

            if backtrack(sudoku, idx + 1, delay, show) {
                return true;
            }

            sudoku.grid[row][col] = 0;
        }
    }

    false
}

#[derive(Debug)]
enum ArgumentError {
    InvalidErr,
    ParseErr,
}

fn handle_args(args: &[String]) -> Result<SudokuOptions, ArgumentError> {
    let mut opts = SudokuOptions {
        print: false,
        delay: time::Duration::from_millis(0),
        measure: false,
        initial_grid: [[0; 9]; 9],
    };

    let mut iter = args.iter().skip(1);

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-g" | "--grid" => { /* TODO: Implement grid input [0.1.2...5..3.3.24.324 ...] */ 
                if let Some(next_arg) = iter.next() {
                    opts.initial_grid = parse_grid(next_arg);

                }
            }
            "-p" | "--print" => {
                opts.print = true;
            }

            "-d" | "--delay" => {
                if let Some(next_arg) = iter.next() {
                    match next_arg.parse() {
                        Ok(v) => {
                            opts.delay = time::Duration::from_millis(v);
                        }

                        Err(_) => {
                            return Err(ArgumentError::ParseErr);
                        }
                    }
                }
            }

            "-m" | "--measure" => {
                opts.measure = true;
            }

            "-h" | "--help" => {
                print_help();
                process::exit(0);
            }

            _ => {
                print_help();
                return Err(ArgumentError::InvalidErr);
            }
        }
    }

    Ok(opts)
}

fn parse_grid(grid_str: &str) -> [[u8; 9]; 9] {
    let mut result: [[u8; 9]; 9] = [[0; 9]; 9];

    for (i, c) in grid_str.chars().enumerate() {
        if i == 81 { break; }

        match c.to_digit(10) {
            Some(v) if v >= 1 => {
                result[i / 9][i % 9] = v as u8;
            }

            _ => {
                result[i / 9][i % 9] = 0;
            }
        }
    }

    result
}

fn print_help() {
    println!("{BOLD}Usage: ./sudokusolve{RESET} [options]\n");

    println!("{BOLD}Example:{RESET}");
    println!("  Creates a 9x9 Sudoku, prints every iteration with a delay of 250ms.");
    println!(
    " {BOLD}./sudokusolve -p -d{RESET} 250 {BOLD}-g{RESET} 53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79\n"
);

    println!("{BOLD}Options:{RESET}");
    println!(" {BOLD}-g, --grid    <grid>{RESET} sets the grid to be solved");
    println!(" {BOLD}-p, --print         {RESET} if set every iteration will be visualized");
    println!(" {BOLD}-d, --delay   <ms>  {RESET} sets the delay of every iteration");
    println!(" {BOLD}-m, --measure       {RESET} if set time will be measured and shown");
    println!(" {BOLD}-h, --help          {RESET} shows this help dialog");
}

fn main() {
    /* Register handler for ^c */
    /* INFO: Must be in the first position because “move” is used here, which ensures that the ownership of all surrounding variables is transferred to the closure. */
    ctrlc::set_handler(move || {
        /* Show terminal cursor */
        println!("\x1b[?25h");

        process::exit(0);
    })
        .expect("[{RED}{BOLD}CRITICAL{RESET}]: Error setting abort handler.");

    let args: Vec<String> = env::args().collect();

    let opts = match handle_args(&args) {
        Ok(o) => o,
        Err(e) => {
            /* Show terminal cursor */
            print!("\x1b[?25h");

            eprintln!("[{RED}ERROR{RESET}]: {:?}", e);
            process::exit(1);
        }
    };

    /* Clear screen */
    print!("\x1b[2J");

    /* Hide cursor */
    print!("\x1b[?25l");

    let mut sudoku = Sudoku::new(opts);

    /* Measure process time */
    let now = time::Instant::now();

    match sudoku.solve() {
        Ok(_) => {
            sudoku.print();
        }

        Err(e) => {
            sudoku.print();
            println!("[{RED}ERROR{RESET}]: {:?}", e);
        }
    }

    if sudoku.opts.measure {
        println!(
            "[{YELLOW}INFO{RESET}]: Finished in {BOLD}{:.2?}.{RESET}",
            now.elapsed()
        );
    }

    /* Show terminal cursor */
    print!("\x1b[?25h");
}
