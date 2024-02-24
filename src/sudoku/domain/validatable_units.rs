use std::rc::Rc;

use super::cell::Cell;

pub trait GameStateValidator {
    fn is_valid(&self) -> bool;
    fn is_complete(&self) -> bool;
}

pub trait CellGroupValidator {
    fn is_valid(&self, cells: &CellGroup) -> bool;
    fn is_complete(&self, cells: &CellGroup) -> bool;
}

pub struct UnitValidator {
}

impl UnitValidator {
    pub fn new() -> Self{
        return Self {}
    }
    
    pub fn cells_are_unique(&self, cell_group: &CellGroup) -> bool{
        let actual_values: Vec<u8> = cell_group.cells.iter().filter(|&cell| cell.value.is_some()).map(|cell| cell.value.unwrap()).collect();
        let mut deduped: Vec<u8> = actual_values.to_vec();
        deduped.dedup();

        return actual_values.len() == deduped.len();
    }

    pub fn all_cells_filled_in_and_unique(&self, cell_group: &CellGroup) -> bool {
        return cell_group.cells.iter().all(|cell| cell.value.is_some()) && self.cells_are_unique(cell_group);
    }

}

impl CellGroupValidator for UnitValidator {
    fn is_valid(&self, cell_group: &CellGroup) -> bool {
        return self.cells_are_unique(&cell_group);
    }

    fn is_complete(&self, cell_group: &CellGroup) -> bool {
        return self.all_cells_filled_in_and_unique(cell_group)   
    }
}
pub struct CellGroup {
    pub cells: Vec<Rc<Cell>>
}

impl CellGroup {
    pub fn new(cells: Vec<Rc<Cell>>) -> Self {
        return Self {
            cells: cells
        };
    }
}