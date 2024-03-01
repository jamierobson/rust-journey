use std::{cell::RefCell, rc::Weak};
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
}

impl CellGroupValidator for UnitValidator {
    fn is_valid(&self, cell_group: &CellGroup) -> bool {

        let all_cell_values: Vec<u8> = 
            cell_group
            .cells
            .iter()
            .filter_map(|weak| weak.upgrade())
            .filter_map(|rc| rc.borrow().value)
            .collect();

        let mut deduped: Vec<u8> = all_cell_values.to_vec();
        deduped.dedup();

        return all_cell_values.len() == deduped.len();

    }

    fn is_complete(&self, cell_group: &CellGroup) -> bool {
        return self.is_valid(cell_group) &&
            cell_group
            .cells
            .iter()
            .all(|weak| weak.upgrade().is_some_and(|rc| rc.borrow().value.is_some()));

    }
}
pub struct CellGroup {
    pub cells: Vec<Weak<RefCell<Cell>>>
}

impl CellGroup {
    pub fn new(cells: Vec<Weak<RefCell<Cell>>>) -> Self {
        return Self {
            cells: cells
        };
    }
}