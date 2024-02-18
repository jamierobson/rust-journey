use super::cell::Cell;

pub trait GameStateValidator {
    fn is_valid(&self) -> bool;
    fn is_complete(&self) -> bool;
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
    
    pub fn cells_are_unique(&self, cells: &Vec<& Cell>) -> bool{
        let actual_values: Vec<u8> = cells.iter().filter(|&cell| cell.value.is_some()).map(|&cell| cell.value.unwrap()).collect();
        let mut deduped: Vec<u8> = actual_values.to_vec();
        deduped.dedup();

        return actual_values.len() == deduped.len();
    }

    pub fn all_cells_filled_in_and_unique(&self, cells: &Vec<& Cell>) -> bool {
        return cells.iter().all(|&cell| cell.value.is_some()) && self.cells_are_unique(&cells);
    }

}

impl CellGroupValidator for UnitValidator {
    fn is_valid(&self, cells: &Vec<& Cell>) -> bool {
        return self.cells_are_unique(&cells.to_vec());
    }

    fn is_complete(&self, cells: &Vec<& Cell>) -> bool {
        return self.all_cells_filled_in_and_unique(&cells.to_vec())   
    }
}