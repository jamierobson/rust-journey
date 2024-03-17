mod hello_world;
mod sudoku;
mod pretty;
use sudoku::core::puzzle::Puzzle;
use sudoku::draw::terminal_print::{draw_all_rows, draw_all_cells};

fn main() {
    hello_world::greeter::say_hello();
    let mut sudoku = Puzzle::default();
    set_some_arbitrary_values(&mut sudoku);
    draw_all_rows(&sudoku.rows);
}

fn set_some_arbitrary_values(sudoku: &mut Puzzle){
    sudoku.cell_grid[2][1].borrow_mut().set_value(9); 
    sudoku.cell_grid[6][6].borrow_mut().set_value(6);
    sudoku.cell_grid[7][7].borrow_mut().set_value(1);
    sudoku.cell_grid[5][5].borrow_mut().set_value(4);
    sudoku.cell_grid[7][8].borrow_mut().set_value(2);
}