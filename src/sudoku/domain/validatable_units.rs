use super::{cell::Cell, cell_grid::CellGrid, consts::{PUZZLE_SUB_GRID_HEIGHT, PUZZLE_SUB_GRID_WIDTH}};

pub trait GameStateValidator {
    fn is_valid(&self, grid: &CellGrid) -> bool;
    fn is_complete(&self, grid: &CellGrid) -> bool;
}

pub trait CellGroupValidator {
    fn is_valid(&self, cells: &Vec<& Cell>) -> bool;
    fn is_complete(&self, cells: &Vec<& Cell>) -> bool;
}

pub struct UnitValidator {
}

impl UnitValidator {
    pub fn new() -> Self{
        return Self {}
    }
    
    pub fn cells_are_unique(&self, cells: &Vec<& Cell>) -> bool{
        let actual_values: Vec<u8> = cells.iter().filter(|&cell| cell.value.is_some()).map(|&cell| cell.value.unwrap()).collect();
        let mut deduped: Vec<u8> = actual_values.to_vec();
        deduped.dedup();

        return actual_values.len() == deduped.len();
    }

    pub fn all_cells_filled_in_and_unique(&self, cells: &Vec<& Cell>) -> bool {
        return cells.iter().all(|&c| c.value.is_some()) && self.cells_are_unique(&cells);
    }

}

impl CellGroupValidator for UnitValidator {
    fn is_valid(&self, cells: &Vec<& Cell>) -> bool {
        return self.cells_are_unique(&cells.to_vec());
    }
    fn is_complete(&self, cells: &Vec<& Cell>) -> bool {
        return self.all_cells_filled_in_and_unique(&cells.to_vec())   
    }
}

// todo: the value in these "validators" are the logic in pulling the relevant rows out of the grid.
// It _might_ be worth considering that the game, or the grid, also has these views available, 
// and then reuses the same single validator. It might be worthwhile to look at
pub struct RowValidator {
    pub row_number: usize,
    validator: UnitValidator
}

impl RowValidator {
    pub fn new(row_number: usize) -> Self{
        return Self {
            row_number: row_number,
            validator: UnitValidator::new()
        }
    }

    pub fn is_valid(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid[self.row_number].iter().collect();
        return self.validator.is_valid(&cells);
    }

    pub fn is_complete(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid[self.row_number].iter().collect();
        return self.validator.is_complete(&cells);
    }
}

pub struct ColumnValidator {
    pub column_number: usize,
    validator: UnitValidator
}

impl ColumnValidator {
    pub fn new(column_number: usize) -> Self{
        return Self {
            column_number: column_number,
            validator: UnitValidator::new()
        }
    }

    pub fn is_valid(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid.grid.iter().map(|row| &row[self.column_number]).collect();
        return self.validator.is_valid(&cells);
    }

    pub fn is_complete(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid.grid.iter().map(|row| &row[self.column_number]).collect();
        return self.validator.is_complete(&cells);
    }
}

pub struct BlockValidator {
    pub block_row_number: usize,
    pub block_column_number: usize,
    column_range_lower_index: usize,
    column_range_upper_index: usize,
    row_range_lower_index: usize,
    row_range_upper_index: usize,
    validator: UnitValidator
}

impl BlockValidator {
    pub fn new(block_row_number: usize, block_column_number: usize) -> Self{
        return Self {
            block_row_number: block_row_number,
            block_column_number: block_column_number,
            row_range_lower_index: block_row_number * PUZZLE_SUB_GRID_HEIGHT,
            row_range_upper_index: block_row_number * PUZZLE_SUB_GRID_HEIGHT + PUZZLE_SUB_GRID_HEIGHT,
            column_range_lower_index: block_column_number * PUZZLE_SUB_GRID_WIDTH,
            column_range_upper_index: block_column_number * PUZZLE_SUB_GRID_WIDTH + PUZZLE_SUB_GRID_WIDTH,
            validator: UnitValidator::new()
        }
    }

    pub fn is_valid(&self, cell_grid: &CellGrid) -> bool {

        let cells: Vec<&Cell> = cell_grid.grid[self.row_range_lower_index .. self.row_range_upper_index]
            .iter()
            .flat_map(|row| row[self.column_range_lower_index .. self.column_range_upper_index]
            .iter())
            .collect();
        
            return self.validator.is_valid(&cells);
    }

    pub fn is_complete(&self, cell_grid: &CellGrid) -> bool {

        // todo: Extracting this code to its own function that I can share with the above, even private to the type, 
        // leads to lifetime problems which is infuriating me.
        // This is the best I have at the moment
        // If I wanted to hold a reference to the cells at construction time, I'd also need lifetimes...
        // argh
        let cells: Vec<&Cell> = cell_grid.grid[self.row_range_lower_index .. self.row_range_upper_index]
            .iter()
            .flat_map(|row| row[self.column_range_lower_index .. self.column_range_upper_index]
            .iter())
            .collect();
        
            return self.validator.is_complete(&cells);
    }
}