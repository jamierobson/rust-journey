use super::{cell::Cell, cell_grid::CellGrid, consts::{PUZZLE_SUB_GRID_HEIGHT, PUZZLE_SUB_GRID_WIDTH}};

pub trait CellGroupSelector {
    fn get_cells(&self, cell_grid: &CellGrid) -> Vec<&Cell>;
}

pub struct RowSelector {
    pub row_number: usize
}

impl RowSelector {
    pub fn new(row_number: usize) -> Self{
        return Self {
            row_number: row_number,
        }
    }
}

impl CellGroupSelector for RowSelector {
    fn get_cells(&self, cell_grid: &CellGrid) -> Vec<&Cell> {
        return cell_grid[self.row_number].iter().collect();
    }
}

pub struct ColumnSelector {
    pub column_number: usize
}

impl ColumnSelector {
    pub fn new(column_number: usize) -> Self{
        return Self {
            column_number: column_number
        }
    }
}

impl CellGroupSelector for ColumnSelector {
    fn get_cells(&self, cell_grid: &CellGrid) -> Vec<&Cell> {
        return cell_grid.grid.iter().map(|row| &row[self.column_number]).collect();
    }
}

pub struct BlockSelector {
    pub block_row_number: usize,
    pub block_column_number: usize,
    column_range_lower_index: usize,
    column_range_upper_index: usize,
    row_range_lower_index: usize,
    row_range_upper_index: usize
}

impl BlockSelector {
    pub fn new(block_row_number: usize, block_column_number: usize) -> Self{
        return Self {
            block_row_number: block_row_number,
            block_column_number: block_column_number,
            row_range_lower_index: block_row_number * PUZZLE_SUB_GRID_HEIGHT,
            row_range_upper_index: block_row_number * PUZZLE_SUB_GRID_HEIGHT + PUZZLE_SUB_GRID_HEIGHT,
            column_range_lower_index: block_column_number * PUZZLE_SUB_GRID_WIDTH,
            column_range_upper_index: block_column_number * PUZZLE_SUB_GRID_WIDTH + PUZZLE_SUB_GRID_WIDTH
        }
    }
}

impl CellGroupSelector for BlockSelector {
    fn get_cells(&self, cell_grid: &CellGrid) -> Vec<&Cell> {
        return cell_grid.grid[self.row_range_lower_index .. self.row_range_upper_index]
        .iter()
        .flat_map(|row| row[self.column_range_lower_index .. self.column_range_upper_index]
        .iter())
        .collect()
    }
}