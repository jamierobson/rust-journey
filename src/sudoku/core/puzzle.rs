use crate::pretty::aliases::*;

use super::{cell_grid::{CellGrid, GridOfReferences}, consts::{PUZZLE_BLOCK_HEIGHT, PUZZLE_BLOCK_WIDTH, PUZZLE_DIMENTION}, validatable_units::{CellGroup, CellGroupValidator, PuzzleValidator, UnitValidator}};

pub type SeedRow = [Option<u8>; PUZZLE_DIMENTION];
pub type SeedGrid = [SeedRow; PUZZLE_DIMENTION];

pub struct Puzzle {
    pub cell_grid: CellGrid,
    pub rows: Vector<CellGroup>,
    pub columns: Vector<CellGroup>,
    pub blocks: Vector<CellGroup>,
    unit_validator: UnitValidator,
}

impl Puzzle {
    pub fn default() -> Self {
        let cell_grid =  CellGrid::new();
        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cell_grid.grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cell_grid.grid)).collect();
        let mut blocks = Vector::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_WIDTH {
            blocks.push(get_block(x, y, &cell_grid.grid));
        }}

        Self {
            cell_grid,
            rows,
            columns,
            blocks,
            unit_validator: UnitValidator::new()
        }
    }

    pub fn new(seed: &[[Option<u8>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION]) -> Self {
        //todo: Split out what's copied here, even though constructor bits are awkward with referenced items
        let cell_grid =  CellGrid::from_seed(seed);
        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cell_grid.grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cell_grid.grid)).collect();
        let mut blocks = Vector::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_WIDTH {
            blocks.push(get_block(x, y, &cell_grid.grid));
        }}

        Self {
            cell_grid,
            rows,
            columns,
            blocks,
            unit_validator: UnitValidator::new()
        }
    }

    pub fn count_cells_with_value(&self) -> usize {
        return self.cell_grid.grid.iterate().flatten().filter(|&rc| rc.borrow().value.is_some()).count();
    }
}

impl Clone for Puzzle {
    fn clone(&self) -> Self {
        let cloned_cell_grid = self.cell_grid.clone();
        let rows = (0..PUZZLE_DIMENTION).map(|i| get_row(i, &cloned_cell_grid.grid)).collect();
        let columns = (0..PUZZLE_DIMENTION).map(|i| get_column(i, &cloned_cell_grid.grid)).collect();
        let mut blocks = Vector::<CellGroup>::new();

        for x in 0 .. PUZZLE_BLOCK_HEIGHT {
        for y in 0 .. PUZZLE_BLOCK_WIDTH {
            blocks.push(get_block(x, y, &cloned_cell_grid.grid));
        }}
        
        Self {
            cell_grid: cloned_cell_grid,
            rows,
            columns,
            blocks,
            unit_validator: UnitValidator::new()
        }
    }
}

fn get_block(block_row_number: usize, block_column_number:usize, cell_grid: &GridOfReferences) -> CellGroup {

    let row_range_lower_index = block_row_number * PUZZLE_BLOCK_HEIGHT;
    let row_range_upper_index = block_row_number * PUZZLE_BLOCK_HEIGHT + PUZZLE_BLOCK_HEIGHT;
    let column_range_lower_index = block_column_number * PUZZLE_BLOCK_WIDTH;
    let column_range_upper_index = block_column_number * PUZZLE_BLOCK_WIDTH + PUZZLE_BLOCK_WIDTH;

    let cells  = cell_grid[row_range_lower_index .. row_range_upper_index]
    .iterate()
    .flat_map(|row| row[column_range_lower_index .. column_range_upper_index].iterate())
    .cloned()
    .collect();

    return CellGroup::new(cells);
}

fn get_row(row_number: usize, cell_grid: &GridOfReferences) -> CellGroup {
    let cells = cell_grid[row_number].iterate().map(|cell| cell.clone()).collect();
    return CellGroup::new(cells);
}

fn get_column(column_number: usize, cell_grid: &GridOfReferences) -> CellGroup {

    let cells = cell_grid.iterate().map(|row| row[column_number].clone()).collect();
    return CellGroup::new(cells);
}

impl PuzzleValidator for Puzzle {
    fn is_valid(&self) -> bool {
            return self.rows.iterate().all(|r| self.unit_validator.is_valid(r))
            && self.columns.iterate().all(|r| self.unit_validator.is_valid(r))
            && self.blocks.iterate().all(|r| self.unit_validator.is_valid(r));
    }

    fn is_complete(&self) -> bool {
        return self.rows.iterate().all(|r| self.unit_validator.is_complete(r))
        && self.columns.iterate().all(|r| self.unit_validator.is_complete(r))
        && self.blocks.iterate().all(|r| self.unit_validator.is_complete(r));
    }
}