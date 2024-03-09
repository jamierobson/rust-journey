mod hello_world;
mod sudoku;
use sudoku::core::game::Game;
use sudoku::draw::terminal_print::{draw_all_rows, draw_all_cells};
use sudoku::techniques::implicitly_solved;

fn main() {
    hello_world::greeter::say_hello();
    let mut sudoku = Game::default();
    set_some_arbitrary_values(&mut sudoku);

    draw_all_cells(&sudoku.cell_grid); //This draws as (y, x) because we index into rows first, then the column value from there
    draw_all_rows(&sudoku.rows);
}

fn set_some_arbitrary_values(sudoku: &mut Game){
    sudoku.cell_grid[2][1].borrow_mut().set_value(9); 
    sudoku.cell_grid[6][6].borrow_mut().set_value(6);
    sudoku.cell_grid[7][7].borrow_mut().set_value(1);
    sudoku.cell_grid[5][5].borrow_mut().set_value(4);
    sudoku.cell_grid[7][8].borrow_mut().set_value(2);
}