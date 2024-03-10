use crate::{pretty::aliases::*, sudoku::core::cell_grid::CellReference};
use std::collections::{hash_map, HashMap};

use crate::sudoku::core::{cell::Cell, game::Game, validatable_units::CellGroup};

pub fn eliminate_closed_candidate_groups(game: &mut Game) {
    eliminate_options_from_groups(&mut game.rows);
    eliminate_options_from_groups(&mut game.columns);
    eliminate_options_from_groups(&mut game.blocks);
}

fn key_from_potential_values(potentials: &CellReference) -> String {
    return potentials.borrow().potentially_valid_values
    .iterate()
    .map(|value| value.to_string())
    .collect::<Vector<_>>()
    .join("-");

}

fn eliminate_options_from_groups(collection: &mut Vector<CellGroup>) {

    for cell_group in collection {
        
        let mut hashmap : HashMap<String, Vector<CellReference>>= HashMap::new();

        // Group cells by their potential values, so that we can identify if, for example, 
        // two cells both have the exact same 2 candidates, 
        // meaning that none other of the cells could have those
        for cell_reference in &cell_group.cells {
            cell_reference.borrow_mut().potentially_valid_values.sort();
            let key = key_from_potential_values(cell_reference);
            
            if let Some(cells) = hashmap.get_mut(&key) {
                cells.push(cell_reference.clone());
            } else {
                hashmap.insert(key, vec![cell_reference.clone()]);
            }
        }

        println!("{:?}", hashmap);
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sudoku::core::{cell::Cell, cell_grid::CellReference, validatable_units::CellGroup};

    use super::*;

    //todo: This is copied from validatable_units. Share this properly
    fn cell_reference_from_value(cell_value_option: Option<u8>) -> CellReference {
        
        let new_cell_ref = Rc::new(RefCell::new(Cell::new()));
        if let Some(value) = cell_value_option {
            new_cell_ref.borrow_mut().set_value(value);
        }
        return new_cell_ref;
    }

    #[test]
    fn just_test_the_grouping_method() {
        let cell_a = cell_reference_from_value(None);
        let cell_b = cell_reference_from_value(None);
        let cell_c = cell_reference_from_value(None);
        let cell_d = cell_reference_from_value(None);

        cell_a.borrow_mut().discount_range(1..=7);
        cell_b.borrow_mut().discount_range(1..=7);
        cell_c.borrow_mut().discount_range(1..=6);
        cell_d.borrow_mut().discount_value(1);
        
        let references = vec![
            cell_a,
            cell_b,
            cell_c,
            cell_d
        ];

        let group = CellGroup::new(references);
        eliminate_options_from_groups(&mut vec![group]);
    }
}