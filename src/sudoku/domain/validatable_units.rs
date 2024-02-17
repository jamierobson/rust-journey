use super::{cell::Cell, cell_grid::CellGrid};

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

pub trait GameStateValidator {
    fn is_valid(&self, grid: &CellGrid) -> bool;
    fn is_complete(&self, grid: &CellGrid) -> bool;
}

pub trait CellGroupValidator {
    fn is_valid(&self, cells: &Vec<& Cell>) -> bool;
    fn is_complete(&self, cells: &Vec<& Cell>) -> bool;
}

pub struct UnitValidator {
}

impl UnitValidator {
    pub fn new() -> Self{
        return Self {}
    }

}

impl CellGroupValidator for UnitValidator {
    fn is_valid(&self, cells: &Vec<& Cell>) -> bool {
        return cells_are_unique(&cells.to_vec());
    }
    fn is_complete(&self, cells: &Vec<& Cell>) -> bool {
        return all_cells_filled_in_and_unique(&cells.to_vec())   
    }
}

pub struct RowValidator {
    row_number: usize,
    validator: UnitValidator
}

impl RowValidator {
    pub fn new(row_number: usize) -> Self{
        return Self {
            row_number: row_number,
            validator: UnitValidator::new()
        }
    }

    pub fn is_valid(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid[self.row_number].iter().collect();
        return cells_are_unique(&cells);
    }

    pub fn is_complete(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid[self.row_number].iter().collect();
        return all_cells_filled_in_and_unique(&cells);
    }
}



pub struct ColumnValidator {
    column_number: usize,
    validator: UnitValidator
}

impl ColumnValidator {
    pub fn new(column_number: usize) -> Self{
        return Self {
            column_number: column_number,
            validator: UnitValidator::new()
        }
    }

    pub fn is_valid(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid.grid.iter().map(|row| &row[self.column_number]).collect();
        return cells_are_unique(&cells);
    }

    pub fn is_complete(&self, cell_grid: &CellGrid) -> bool {
        let cells = cell_grid.grid.iter().map(|row| &row[self.column_number]).collect();
        return all_cells_filled_in_and_unique(&cells);
    }
}



fn cells_are_unique(cells: &Vec<& Cell>) -> bool{
    let actual_values: Vec<u8> = cells.iter().filter(|&cell| cell.value.is_some()).map(|&cell| cell.value.unwrap()).collect();
    let mut deduped: Vec<u8> = actual_values.to_vec();
    deduped.dedup();

    return actual_values.len() == deduped.len();
}

fn all_cells_filled_in_and_unique(cells: &Vec<& Cell>) -> bool {
    return cells.iter().all(|&c| c.value.is_some()) && cells_are_unique(&cells);
}
