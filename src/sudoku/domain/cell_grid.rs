use std::{cell::RefCell, ops::{Index, IndexMut}, rc::Rc};
use super::{cell::Cell, consts::PUZZLE_DIMENTION};

pub type RowOfReferences = [Rc<RefCell<Cell>>; PUZZLE_DIMENTION];
pub type GridOfReferences = [RowOfReferences; PUZZLE_DIMENTION];

pub struct CellGrid {
    pub grid: GridOfReferences
}

impl CellGrid{
    pub fn new() -> Self {
        let cell_grid = empty_grid();
        return Self {
            grid: cell_grid
        };
    }
}

fn empty_grid() -> GridOfReferences {
    return core::array::from_fn(|_i| empty_row_array());
}

fn empty_row_array() -> RowOfReferences {
    return core::array::from_fn(|_i| Rc::new(RefCell::new(Cell::new())));
}

// allow index syntax on the cell grid itself
impl Index<usize> for CellGrid{
    type Output = RowOfReferences;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.grid[index];
    }
}

impl IndexMut<usize> for CellGrid{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.grid[index];
    }
}