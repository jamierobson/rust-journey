use std::{cell::RefCell, ops::{Index, IndexMut}, rc::Rc};

use super::{cell::Cell, consts::{PUZZLE_BLOCK_HEIGHT, PUZZLE_BLOCK_WIDTH, PUZZLE_DIMENTION}, validatable_units::{CellGroup, CellGroupValidator, GameStateValidator, UnitValidator}};

pub struct CellGrid {
    pub grid: [[Rc<RefCell<Cell>>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION],
    pub rows: Vec<CellGroup>,
    pub columns: Vec<CellGroup>,
    pub blocks: Vec<CellGroup>,
    unit_validator: UnitValidator
}

impl CellGrid{
    pub fn new() -> Self {
        let cell_grid = empty_grid();
        // let cell_reference_grid = cell_reference_grid(&cell_grid);

        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cell_grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cell_grid)).collect();
        let mut blocks = Vec::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_HEIGHT {
            blocks.push(get_block(x, y, &cell_grid));
        }}


        return Self {
            rows: rows,
            columns: columns,
            blocks: blocks,
            unit_validator: UnitValidator::new(),
            grid: cell_grid
        };
    }
}

impl GameStateValidator for CellGrid {
    fn is_valid(&self) -> bool {
            return self.rows.iter().all(|r| self.unit_validator.is_valid(r))
            && self.columns.iter().all(|r| self.unit_validator.is_valid(r))
            && self.blocks.iter().all(|r| self.unit_validator.is_valid(r));
    }

    fn is_complete(&self) -> bool {
        return self.rows.iter().all(|r| self.unit_validator.is_complete(r))
        && self.columns.iter().all(|r| self.unit_validator.is_complete(r))
        && self.blocks.iter().all(|r| self.unit_validator.is_complete(r));
    }
}

fn empty_grid() -> [[Rc<RefCell<Cell>>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION] {
    return core::array::from_fn(|_i| empty_row_array());

}

fn empty_row_array() -> [Rc<RefCell<Cell>>; PUZZLE_DIMENTION]{
    // return core::array::from_fn(|_i| Rc::new(Cell::new()));
    return core::array::from_fn(|_i| Rc::new(RefCell::new(Cell::new())));
}

fn get_row(row_number: usize, cell_grid: &[[Rc<RefCell<Cell>>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION]) -> CellGroup {
    let cells = cell_grid[row_number].iter().map(|c| c.clone()).collect();
    return CellGroup::new(cells);
}

fn get_column(column_number: usize, cell_grid: &[[Rc<RefCell<Cell>>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION]) -> CellGroup {

    let cells = cell_grid.iter().map(|row| row[column_number].clone()).collect();
    return CellGroup::new(cells);
}

fn get_block(block_row_number: usize, block_column_number:usize, cell_grid: &[[Rc<RefCell<Cell>>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION]) -> CellGroup {

    let row_range_lower_index = block_row_number * PUZZLE_BLOCK_HEIGHT;
    let row_range_upper_index = block_row_number * PUZZLE_BLOCK_HEIGHT + PUZZLE_BLOCK_HEIGHT;
    let column_range_lower_index = block_column_number * PUZZLE_BLOCK_WIDTH;
    let column_range_upper_index = block_column_number * PUZZLE_BLOCK_WIDTH + PUZZLE_BLOCK_WIDTH;

    let cells  = cell_grid[row_range_lower_index .. row_range_upper_index]
    .iter()
    .flat_map(|row| row[column_range_lower_index .. column_range_upper_index].iter())
    .map(|c| c.clone())
    .collect();

    return CellGroup::new(cells);
}

// allow index syntax on the cell grid itself
impl Index<usize> for CellGrid{
    type Output = [Rc<RefCell<Cell>>; PUZZLE_DIMENTION];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.grid[index];
    }
}

impl IndexMut<usize> for CellGrid{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.grid[index];
    }
}