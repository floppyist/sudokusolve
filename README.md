# 🧩 SudokuSolve-RS

A Sudoku solver written in Rust that utilizes backtracking and ANSI terminal visualization to solve puzzles.

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- Visualization: Watch the backtracking algorithm work live in your terminal.
- Customizable: Adjustable delay for the visualization process.
- Colorful: Uses ANSI escape sequences for clear distinction (initial values in red, solved numbers in green).
- Performance Tracking: Integrated timer to measure solving speed.

## 🚀 Installation

Ensure you have the Rust compiler (cargo) installed.

# Clone the repository
git clone https://github.com/floppyist/sudokusolve.git
cd sudokusolve

# Build the project
cargo build --release

## 🛠 Usage

Run the solver directly via cargo or the binary directory:

./target/release/sudokusolve -g "YOUR_SUDOKU_STRING" [options]

### Options

| Flag | Long Form | Description |
| :--- | :--- | :--- |
| -g | --grid | The Sudoku grid as a string (81 chars, use . for empty cells). |
| -p | --print | Enables live visualization in the terminal. |
| -d | --delay | Set delay in milliseconds between steps (e.g., 10). |
| -m | --measure | Displays the time elapsed after completion. |
| -h | --help | Displays the help dialog. |

### Example

Solve a Sudoku with visualization (25ms delay):

./sudokusolve -p -d 25 -m -g "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79"
