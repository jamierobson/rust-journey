use std::{rc::Rc};

use super::{cell_grid::{CellGrid, GridOfReferences}, consts::{PUZZLE_BLOCK_HEIGHT, PUZZLE_BLOCK_WIDTH, PUZZLE_DIMENTION}, validatable_units::{CellGroup, CellGroupValidator, GameStateValidator, UnitValidator}};

pub type SeedRow = [Option<u8>; PUZZLE_DIMENTION];
pub type SeedGrid = [SeedRow; PUZZLE_DIMENTION];

pub struct Game {
    pub cell_grid: CellGrid,
    pub rows: Vec<CellGroup>,
    pub columns: Vec<CellGroup>,
    pub blocks: Vec<CellGroup>,
    unit_validator: UnitValidator,
}

impl Game {
    pub fn default() -> Self {
        let cell_grid =  CellGrid::new();
        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cell_grid.grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cell_grid.grid)).collect();
        let mut blocks = Vec::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_WIDTH {
            blocks.push(get_block(x, y, &cell_grid.grid));
        }}

        Self {
            cell_grid: cell_grid,
            rows: rows,
            columns: columns,
            blocks: blocks,
            unit_validator: UnitValidator::new()
        }
    }

    pub fn new(seed: &[[Option<u8>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION]) -> Self {
        //todo: Split out what's copied here, even though constructor bits are awkward with referenced items
        let cell_grid =  CellGrid::from_seed(seed);
        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cell_grid.grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cell_grid.grid)).collect();
        let mut blocks = Vec::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_WIDTH {
            blocks.push(get_block(x, y, &cell_grid.grid));
        }}

        Self {
            cell_grid: cell_grid,
            rows: rows,
            columns: columns,
            blocks: blocks,
            unit_validator: UnitValidator::new()
        }
    }

    pub fn count_cells_with_value(&self) -> usize {
        return self.cell_grid.grid.iter().flatten().filter(|&rc| rc.borrow().value.is_some()).count();
    }
}

fn get_block(block_row_number: usize, block_column_number:usize, cell_grid: &GridOfReferences) -> CellGroup {

    let row_range_lower_index = block_row_number * PUZZLE_BLOCK_HEIGHT;
    let row_range_upper_index = block_row_number * PUZZLE_BLOCK_HEIGHT + PUZZLE_BLOCK_HEIGHT;
    let column_range_lower_index = block_column_number * PUZZLE_BLOCK_WIDTH;
    let column_range_upper_index = block_column_number * PUZZLE_BLOCK_WIDTH + PUZZLE_BLOCK_WIDTH;

    let cells  = cell_grid[row_range_lower_index .. row_range_upper_index]
    .iter()
    .flat_map(|row| row[column_range_lower_index .. column_range_upper_index].iter())
    .map(|c| Rc::downgrade(&c))
    .collect();

    return CellGroup::new(cells);
}

fn get_row(row_number: usize, cell_grid: &GridOfReferences) -> CellGroup {
    let cells = cell_grid[row_number].iter().map(|c| Rc::downgrade(&c)).collect();
    return CellGroup::new(cells);
}

fn get_column(column_number: usize, cell_grid: &GridOfReferences) -> CellGroup {

    let cells = cell_grid.iter().map(|row| Rc::downgrade(&row[column_number])).collect();
    return CellGroup::new(cells);
}

impl GameStateValidator for Game {
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