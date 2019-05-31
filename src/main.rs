use std::io::Write;
use std::{io, thread, time};

type Sudoku = [[u8; 9]; 9];

static SHOW_PROGRESS: bool = false;

fn main() {
    let mut sudoku = create_sudoku();
    print_sudoku(&sudoku);
    solve_step(0, 0, &mut sudoku);
    print_sudoku(&sudoku);
}

fn solve_step(x: usize, y: usize, mut sudoku: &mut Sudoku) -> bool {
    let (next_x, next_y) = next_coordinates(x, y);

    // are we done?
    if x == 0 && y == 9 {
        return true;
    }

    // is the current cell prefilled?
    if sudoku[y][x] > 0 {
        return solve_step(next_x, next_y, &mut sudoku);
    }

    // try values for current cell
    for i in 1u8..=9 {
        if is_valid(x, y, i, &sudoku) {
            sudoku[y][x] = i;
            if SHOW_PROGRESS {
                thread::sleep(time::Duration::from_millis(100));
                print_sudoku(&sudoku);
            }
            if solve_step(next_x, next_y, &mut sudoku) {
                return true;
            }
            sudoku[y][x] = 0;
        }
    }

    false
}

fn is_valid(x: usize, y: usize, val: u8, sudoku: &Sudoku) -> bool {
    for i in 0..9 {
        if i != y && sudoku[i][x] == val {
            return false;
        }
        if i != x && sudoku[y][i] == val {
            return false;
        }
    }

    let x_modifier = if x < 3 {
        0
    } else if x < 6 {
        3
    } else {
        6
    };
    let y_modifier = if y < 3 {
        0
    } else if y < 6 {
        3
    } else {
        6
    };

    for i in 0..3 {
        for ii in 0..3 {
            if (i + x_modifier) != x
                && (ii + y_modifier) != y
                && sudoku[ii + y_modifier][i + x_modifier] == val
            {
                return false;
            }
        }
    }

    true
}

fn next_coordinates(x: usize, y: usize) -> (usize, usize) {
    if (x + 1) % 9 == 0 {
        (0, y + 1)
    } else {
        (x + 1, y)
    }
}

fn create_sudoku() -> Sudoku {
    [
        [6, 1, 7, 0, 0, 0, 0, 0, 0],
        [0, 2, 0, 0, 0, 0, 6, 3, 9],
        [0, 0, 0, 4, 2, 6, 0, 0, 0],
        [5, 9, 3, 0, 0, 0, 0, 4, 0],
        [0, 0, 0, 9, 0, 0, 2, 5, 1],
        [1, 0, 0, 7, 5, 4, 0, 0, 3],
        [2, 0, 0, 6, 0, 0, 8, 0, 0],
        [4, 7, 6, 0, 0, 0, 3, 0, 0],
        [9, 0, 0, 3, 0, 1, 4, 0, 2],
    ]
}

fn print_sudoku(sudoku: &Sudoku) {
    if SHOW_PROGRESS {
        erase();
    }
    for (i, row) in sudoku.iter().enumerate() {
        for (ii, v) in row.iter().enumerate() {
            if *v == 0 {
                print!(" ");
            } else {
                print!("{}", v);
            }
            if ii < 8 && (ii + 1) % 3 == 0 {
                print!("|");
            }
        }
        println!();
        if i < 8 && (i + 1) % 3 == 0 {
            for _ in 0..11 {
                print!("-");
            }
            println!();
        }
    }
    println!();
}

fn erase() {
    print!("\x1Bc");
    io::stdout().flush().unwrap();
}
