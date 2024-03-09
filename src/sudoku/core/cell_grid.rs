use std::{cell::RefCell, ops::{Index, IndexMut}, rc::Rc};
use super::{cell::Cell, consts::PUZZLE_DIMENTION, game::{SeedGrid, SeedRow}};

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

    pub fn from_seed(initial_values: &SeedGrid) -> Self {
        let cell_grid = grid_from_raw_values(&initial_values);

        return Self {
            grid: cell_grid
        };
    }
}

fn grid_from_raw_values(initial_values: &SeedGrid) -> GridOfReferences {
    return core::array::from_fn(|i| row_from_raw_values(&initial_values[i]));
}

fn row_from_raw_values(initial_values: &SeedRow) -> RowOfReferences {
    return core::array::from_fn(|i| Rc::new(RefCell::new(Cell::from_value(initial_values[i]))))
}

fn empty_grid() -> GridOfReferences {
    return core::array::from_fn(|_i| empty_row_array());
}

fn empty_row_array() -> RowOfReferences {
    return core::array::from_fn(|_i: usize| Rc::new(RefCell::new(Cell::new())));
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

#[cfg(test)]
mod tests {
        use super::*;

        #[test]
        fn cell_grid_initialized_with_all_empty_cells() {
            let cell_grid = CellGrid::new();

            let any_cells_have_value = 
                cell_grid.grid
                .iter()
                .flat_map(|row| row.iter())
                .any(|rc| rc.borrow().value.is_some());


            assert_eq!(any_cells_have_value, false);
        }

}