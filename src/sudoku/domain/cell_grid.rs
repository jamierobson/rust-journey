use std::ops::{IndexMut, Index};

use super::{cell::Cell, consts::{PUZZLE_DIMENTION, PUZZLE_BLOCK_HEIGHT, PUZZLE_BLOCK_WIDTH}, validatable_units::{CellGroupValidator, GameStateValidator, UnitValidator}};

// This type is a workaround to help locate cells in a grid _without_ lifetimes, 
// so that we don't have to continuously iterate over the whole grid constantly.
// when wanting to check the validity of the single row-column-subblock impaced
// by changing a cell
// 
// My preference is the idea whereby a cellgrid holds the cells, and the game holds "views" of the cellgrid, 
// with rows, columns, subblocks holding references to cells, and then validation can ask these validatable units
// for validity
//
// I want to avoid lifetimes so that I can compile to wasm, though if I can find some way to build a rudimentary
// UI that allows me to use lifetimes, then I'll give that a go.
// For now, this should get me going

// pub struct LocatableCell {
//     pub cell: Cell,
//     pub grid_column_number: u8,
//     pub grid_row_number: u8,
// }

// pub struct CoordinatesAscendingFromTopLeft {
//     row: u8,
//     column: u8
// }

// impl CoordinatesAscendingFromTopLeft {
//     fn new(row: u8, column: u8) -> Self {
//         return Self {
//             row: row,
//             column: column
//         };
//     }
// }

pub struct CellGrid {
    pub grid: [[Cell; PUZZLE_DIMENTION]; PUZZLE_DIMENTION],
    // row_validators: Vec<RowSelector>,
    // column_validators: Vec<ColumnSelector>,
    // block_validators: Vec<BlockSelector>,
    unit_validator: UnitValidator
}

impl CellGrid{
    pub fn new() -> Self {
        return Self {
            grid: empty_grid(),
            // row_validators: (0..PUZZLE_DIMENTION).map(|i| RowSelector::new(i)).collect(),
            // column_validators: (0..PUZZLE_DIMENTION).map(|i| ColumnSelector::new(i)).collect(),
            // block_validators: (0 .. PUZZLE_BLOCK_WIDTH).flat_map(|x| (0 .. PUZZLE_BLOCK_HEIGHT).map(|y| BlockSelector::new(x, y))).collect(),
            unit_validator: UnitValidator::new()
        };
    }
}

impl GameStateValidator for CellGrid {
    fn is_valid(&self) -> bool {
        return 
            self.row_validators.iter().all(|x| self.unit_validator.is_valid(&x.get_cells(self)))
            && self.column_validators.iter().all(|x| self.unit_validator.is_valid(&x.get_cells(self)))
            && self.block_validators.iter().all(|x| self.unit_validator.is_valid(&x.get_cells(self)));
    }

    fn is_complete(&self) -> bool {
        return 
            self.row_validators.iter().all(|x| self.unit_validator.is_complete(&x.get_cells(self)))
            && self.column_validators.iter().all(|x| self.unit_validator.is_complete(&x.get_cells(self)))
            && self.block_validators.iter().all(|x| self.unit_validator.is_complete(&x.get_cells(self)));
    }
}

fn empty_grid() -> [[Cell; PUZZLE_DIMENTION]; PUZZLE_DIMENTION] {
    return core::array::from_fn(|_i| empty_row_array());

}

fn empty_row_array() -> [Cell; PUZZLE_DIMENTION]{
    return core::array::from_fn(|_i| Cell::new());
}


// allow index syntax on the cell grid itself
impl Index<usize> for CellGrid{
    type Output = [Cell; PUZZLE_DIMENTION];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.grid[index];
    }
}

impl IndexMut<usize> for CellGrid{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.grid[index];
    }
}

fn get_row(row_number: usize, cell_grid: &CellGrid) -> Vec<&Cell> {
    return cell_grid[row_number].iter().collect();
}

fn get_column(column_number: usize, cell_grid: &CellGrid) -> Vec<&Cell> {
    return cell_grid.grid.iter().map(|&row| &row[column_number]).collect();
}

fn get_block(block_row_number: usize, block_column_number:usize, cell_grid: &CellGrid) -> Vec<&Cell> {

    let row_range_lower_index = block_row_number * PUZZLE_BLOCK_HEIGHT;
    let row_range_upper_index = block_row_number * PUZZLE_BLOCK_HEIGHT + PUZZLE_BLOCK_HEIGHT;
    let column_range_lower_index = block_column_number * PUZZLE_BLOCK_WIDTH;
    let column_range_upper_index = block_column_number * PUZZLE_BLOCK_WIDTH + PUZZLE_BLOCK_WIDTH;

    return cell_grid.grid[row_range_lower_index .. row_range_upper_index]
    .iter()
    .flat_map(|&row| row[column_range_lower_index .. column_range_upper_index].iter())
    .collect()
}