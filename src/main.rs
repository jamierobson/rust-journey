mod sudoku;
use sudoku::domain::{cell::Cell, cell_grid::CellGrid, consts::{self, PUZZLE_DIMENTION}};

mod hello_world;

fn main() {
    hello_world::greeter::say_hello();
    let mut cell_grid = CellGrid::new();

    // We've implemented index and indexmut so that we can index into the grid and keep the grid structure itself private
    // todo: I might still prefer to enforce something like get_value and set_value on the grid type itself, 
    // so that it and only it interfaces with the underlying data structure.
    // For now, I think this is ok enough to proceed
    cell_grid[2][1].set_value(9); 
    cell_grid[6][6].set_value(6);

    print!("Ok let's try drawing the whole grid! \n");
    
    //todo: This draws as (y, x) because we index into rows first, then the column value from there. Change? 
    // Or just accept the quirk?
    draw_full_grid(&cell_grid);

    print!("Is this valid? {}", &cell_grid.is_valid());
}

fn draw_full_grid(cell_grid: &CellGrid){

    let separator_line_length = create_row_line(&cell_grid[0]).len();

    draw_separator_line(separator_line_length);
    for i in 0..consts::PUZZLE_DIMENTION {
        draw_row(&cell_grid[i]);
        draw_separator_line(separator_line_length);     
        if include_extra_separator(i.try_into().unwrap(), consts::PUZZLE_SUB_GRID_HEIGHT as u8) {
            draw_separator_line(separator_line_length);     
        }
    }
}

fn draw_row(row: &[Cell; consts::PUZZLE_DIMENTION]) {
    print!("{}\n", create_row_line(row));
}

fn create_row_line(row: &[Cell; consts::PUZZLE_DIMENTION]) -> String {

    let mut row_line_display: String = "|".to_owned();

    for i in 0..consts::PUZZLE_DIMENTION {
        row_line_display.push(' ');
        row_line_display.push_str(value_or_letter_x(&row[i].value).as_str());
        row_line_display.push_str(" |");
     
        if include_extra_separator(i.try_into().unwrap(), consts::PUZZLE_SUB_GRID_WIDTH as u8) {
            row_line_display.push('|');
        }
    }

    return row_line_display;
}

fn include_extra_separator(index: u8, if_divisible_by: u8) -> bool{
    return (1 + index) % if_divisible_by == 0 && (1 + index) != PUZZLE_DIMENTION as u8;
}

fn value_or_letter_x(value: &Option<u8>) -> String {
    return match value {
        None => "x".into(),
        _ => value.as_ref().unwrap().to_string()
    };
}

fn draw_separator_line(length: usize) {
    print!("{} \n", "_".repeat(length));
}