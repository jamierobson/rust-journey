use std::ops::{IndexMut, Index};

use super::{cell::Cell, consts};

pub struct CellGrid {
    grid: [[Cell; consts::PUZZLE_DIMENTION as usize]; consts::PUZZLE_DIMENTION as usize]
}

impl CellGrid{
    pub fn new() -> Self {
        return Self {
            grid: empty_grid()
        };
    }
}

fn empty_grid() -> [[Cell; consts::PUZZLE_DIMENTION as usize]; consts::PUZZLE_DIMENTION as usize] {
    return core::array::from_fn(|_i| empty_row_array());

}

fn empty_row_array() -> [Cell; consts::PUZZLE_DIMENTION as usize]{
    return core::array::from_fn(|_i| Cell::new());
}


// allow index syntax on the cell grid itself
impl Index<usize> for CellGrid{
    type Output = [Cell; consts::PUZZLE_DIMENTION as usize];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.grid[index];
    }
}

impl IndexMut<usize> for CellGrid{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.grid[index];
    }
}