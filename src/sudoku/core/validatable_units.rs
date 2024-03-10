use crate::pretty::aliases::*;
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

        let all_cell_values: Vector<u8> = 
            cell_group
            .cells
            .iterate()
            .filter_map(|weak| weak.upgrade())
            .filter_map(|rc| rc.borrow().value)
            .collect();

        let mut deduped: Vector<u8> = all_cell_values.to_vec();
        deduped.dedup();

        return all_cell_values.len() == deduped.len();

    }

    fn is_complete(&self, cell_group: &CellGroup) -> bool {
        return self.is_valid(cell_group) &&
            cell_group
            .cells
            .iterate()
            .all(|weak| weak.upgrade().is_some_and(|rc| rc.borrow().value.is_some()));

    }
}
pub struct CellGroup {
    pub cells: Vector<Weak<RefCell<Cell>>>
}

impl CellGroup {
    pub fn new(cells: Vector<Weak<RefCell<Cell>>>) -> Self {
        return Self {
            cells: cells
        };
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;

    fn cell_reference_from_value(cell_value_option: Option<u8>) -> Rc<RefCell<Cell>> {
        
        let new_cell_ref = Rc::new(RefCell::new(Cell::new()));
        if let Some(value) = cell_value_option {
            new_cell_ref.borrow_mut().set_value(value);
        }
        return new_cell_ref;
    }

    #[test]
    fn is_valid_true_when_empty_cells() {

        let cells = vec![
            cell_reference_from_value(None),
            cell_reference_from_value(None),
        ];

        let weak: Vector<Weak<RefCell<Cell>>> = cells.iterate().map(|cell| Rc::downgrade(&cell)).collect();
        let group = CellGroup::new(weak);
        let unit_validator = UnitValidator::new();

        assert_eq!(unit_validator.is_valid(&group), true);
    }

    #[test]
    fn is_valid_true_when_different_values() {
        
        let cells = vec![
            cell_reference_from_value(None),
            cell_reference_from_value(None),
            cell_reference_from_value(Some(1)),
            cell_reference_from_value(Some(2))
        ];

        let weak: Vector<Weak<RefCell<Cell>>> = cells.iterate().map(|cell| Rc::downgrade(&cell)).collect();
        let group = CellGroup::new(weak);
        let unit_validator = UnitValidator::new();

        assert_eq!(unit_validator.is_valid(&group), true);
    }

    #[test]
    fn is_complete_false_when_different_values_but_some_none() {

        let cells = vec![
            cell_reference_from_value(None),
            cell_reference_from_value(Some(1)),
            cell_reference_from_value(Some(2))
        ];

        let weak: Vector<Weak<RefCell<Cell>>> = cells.iterate().map(|cell| Rc::downgrade(&cell)).collect();
        let group = CellGroup::new(weak);
        let unit_validator = UnitValidator::new();

        assert_eq!(unit_validator.is_complete(&group), false);
    }

    #[test]
    fn is_complete_true_when_different_values_but_and_all_have_values() {

        let cells = vec![
            cell_reference_from_value(Some(1)),
            cell_reference_from_value(Some(2)),
            cell_reference_from_value(Some(3)),
            cell_reference_from_value(Some(4)),
            cell_reference_from_value(Some(5)),
        ];

        let weak: Vector<Weak<RefCell<Cell>>> = cells.iterate().map(|cell| Rc::downgrade(&cell)).collect();
        let group = CellGroup::new(weak);
        let unit_validator = UnitValidator::new();

        assert_eq!(unit_validator.is_complete(&group), true);
    }

    #[test]
    fn is_valid_false_when_duplicates() {
        
        let cells = vec![
            cell_reference_from_value(Some(1)),
            cell_reference_from_value(Some(1))
        ];

        let weak: Vector<Weak<RefCell<Cell>>> = cells.iterate().map(|cell| Rc::downgrade(&cell)).collect();
        let group = CellGroup::new(weak);
        let unit_validator = UnitValidator::new();

        assert_eq!(unit_validator.is_valid(&group), false);
    }
}