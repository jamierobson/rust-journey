use super::{cell::Cell, cell_grid::{self, CellGrid}, consts::PUZZLE_DIMENTION};

// pub trait UnitValidator {

//     fn is_valid(&self) -> bool;
//     fn is_complete(&self) -> bool;
// }

// pub struct Unit<'a> {
//     pub cells: [&'a Cell; PUZZLE_DIMENTION as usize]
// }

// impl<'a> Unit<'a> {
//     fn new(cells: [&'a Cell; PUZZLE_DIMENTION as usize]) -> Self{
//         return Self {
//             cells
//         };
//     }
// }

// impl<'a> UnitValidator for Unit<'a> {
//     fn is_valid(&self) -> bool {
//         let actual_values: Vec<u8> = self.cells.iter().filter(|&cell| cell.value.is_some()).map(|&cell| cell.value.unwrap()).collect();
//         let mut deduped: Vec<u8> = actual_values.to_vec();
//         deduped.dedup();

//         return actual_values.len() == deduped.len();
//     }

//     fn is_complete(&self) -> bool {
//         return self.cells.iter().all(|&c| c.value.is_some()) && self.is_valid();
//     }
// }

// todo: This lifetime stuff is here because I want to be smarter about building a reference once, 
pub struct Row<'a> {
    row_number: usize,
    cell_group: [&'a Cell; PUZZLE_DIMENTION as usize]
}

impl<'a> Row<'a> {
    fn new(cell_grid: &CellGrid, row_number: usize){
        
    }
}

pub struct Column<'a> {
    column_number: usize,
    cell_group: [&'a Cell; PUZZLE_DIMENTION as usize]
}

impl<'a> Column<'a> {
    fn new(cell_grid: &CellGrid, column_number: usize){
        
    }
}

pub struct Block<'a> {
    zero_index_block_from_left: usize,
    zero_index_block_from_top: usize,
    cell_group: [&'a Cell; PUZZLE_DIMENTION as usize]
}

impl<'a> Block<'a> {
    fn new(cell_grid: &CellGrid, column_number: usize){
        
    }
}

fn is_valid(cells: &[& Cell; PUZZLE_DIMENTION as usize]) -> bool{
    let actual_values: Vec<u8> = cells.iter().filter(|&cell| cell.value.is_some()).map(|&cell| cell.value.unwrap()).collect();
    let mut deduped: Vec<u8> = actual_values.to_vec();
    deduped.dedup();

    return actual_values.len() == deduped.len();
}

fn is_complete(&cells: &[& Cell; PUZZLE_DIMENTION as usize]) -> bool {
    return cells.iter().all(|&c| c.value.is_some()) && is_valid(&cells);
}
