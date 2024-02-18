use std::ops::{IndexMut, Index};

use super::{cell::Cell, consts, validatable_units::{BlockValidator, ColumnValidator, GameStateValidator, RowValidator, UnitValidator}};

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
    pub grid: [[Cell; consts::PUZZLE_DIMENTION]; consts::PUZZLE_DIMENTION],
    row_validators: Vec<RowValidator>,
    column_validators: Vec<ColumnValidator>,
    block_validators: Vec<BlockValidator>,
    unit_validator: UnitValidator
}

impl CellGrid{
    pub fn new() -> Self {
        return Self {
            grid: empty_grid()
        };
    }
}

impl GameStateValidator for CellGrid {
    fn is_valid(&self, grid: &CellGrid) -> bool {
        todo!()
    }

    fn is_complete(&self, grid: &CellGrid) -> bool {
        todo!()
    }
}

fn empty_grid() -> [[Cell; consts::PUZZLE_DIMENTION]; consts::PUZZLE_DIMENTION] {
    return core::array::from_fn(|_i| empty_row_array());

}

fn empty_row_array() -> [Cell; consts::PUZZLE_DIMENTION]{
    return core::array::from_fn(|_i| Cell::new());
}


// allow index syntax on the cell grid itself
impl Index<usize> for CellGrid{
    type Output = [Cell; consts::PUZZLE_DIMENTION];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.grid[index];
    }
}

impl IndexMut<usize> for CellGrid{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.grid[index];
    }
}