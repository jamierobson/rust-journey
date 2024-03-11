use crate::{pretty::aliases::*, sudoku::core::{cell_grid::CellReference, consts::PUZZLE_TOTAL_CELL_COUNT}};
use std::collections::{hash_map, HashMap};

use crate::sudoku::core::{cell::Cell, game::Game, validatable_units::CellGroup};

pub fn eliminate_candidates_from_closed_groups(game: &mut Game) {
    let mut i = 0;
    loop {
        i+=1;

        let pre_iteration_completed_cell_count = game.count_cells_with_value();
        eliminate_closed_candidate_groups(game);

        if game.count_cells_with_value() == pre_iteration_completed_cell_count {
            break;
        }

        if i >= PUZZLE_TOTAL_CELL_COUNT {
            break;
        }
    }
}

fn eliminate_closed_candidate_groups(game: &mut Game) {

    naive_eliminate_options_from_groups(&mut game.rows);
    naive_eliminate_options_from_groups(&mut game.columns);
    naive_eliminate_options_from_groups(&mut game.blocks);
}

fn key_from_potential_values(potentials: &CellReference) -> String {
    return potentials.borrow().potentially_valid_values
    .iterate()
    .map(|value| value.to_string())
    .collect::<Vector<_>>()
    .join("-");

}

fn naive_eliminate_options_from_groups(collection: &mut Vector<CellGroup>) {

    for cell_group in collection {
        
        let mut hashmap : HashMap<String, Vector<CellReference>>= HashMap::new();

        // Group cells by their potential values, so that we can identify if, for example, 
        // two cells both have the exact same 2 candidates, 
        // meaning that none other of the cells could have those
        // Note that this is very naive, as it won't catch cases where
        // we have three cells with, for example, 123, 123, 12 as potentials.
        // These would form a closed group over 1,2,3.
        for cell_reference in &cell_group.cells {
            cell_reference.borrow_mut().potentially_valid_values.sort();
            let key = key_from_potential_values(cell_reference);
            
            if let Some(cells) = hashmap.get_mut(&key) {
                cells.push(cell_reference.clone());
            } else {
                hashmap.insert(key, vec![cell_reference.clone()]);
            }
        }

        for cells_grouped_by_potentials in hashmap {
            if cells_grouped_by_potentials.1.len() == cells_grouped_by_potentials.1[0].borrow().potentially_valid_values.len() {
                for cell in &cell_group.cells {
                    if cell.borrow().potentially_valid_values != cells_grouped_by_potentials.1[0].borrow().potentially_valid_values {
                        cell.borrow_mut().discount_values(&cells_grouped_by_potentials.1[0].borrow().potentially_valid_values)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sudoku::{core::{cell::Cell, cell_grid::CellReference, validatable_units::CellGroup}, draw::terminal_print::draw_all_rows, format::serializer::Serializer};

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
    fn eliminate_candidates_when_groups_of_cells_identified() {
        let cell_a = cell_reference_from_value(None);
        let cell_b = cell_reference_from_value(None);
        let cell_c = cell_reference_from_value(None);
        let cell_d = cell_reference_from_value(None);

        cell_a.borrow_mut().discount_range(1..=7);
        cell_b.borrow_mut().discount_range(1..=7);
        cell_c.borrow_mut().discount_range(1..=6);
        cell_d.borrow_mut().discount_value(1);
        
        let references = vec![
            cell_a.clone(),
            cell_b.clone(),
            cell_c.clone(),
            cell_d.clone()
        ];

        let group = CellGroup::new(references);
        naive_eliminate_options_from_groups(&mut vec![group]);

        assert_eq!(cell_c.clone().borrow().potentially_valid_values, vec![7])
    }
    
    #[test]
    fn try_solve_a_puzzle_that_could_not_be_immediately_implicitely_solved (){
        let test_case = "328975641..13..572.7....839..27....3..7..32.68..6.2..74.9..73687..8..124286134795";
        let mut game = Serializer::new().new_game(test_case).expect("test data is valid");
        
        let initial_cell_count = game.count_cells_with_value();
        draw_all_rows(&game.rows);

        eliminate_candidates_from_closed_groups(&mut game);
        
        draw_all_rows(&game.rows);
        println!("before: {}, after: {}", initial_cell_count, game.count_cells_with_value());
    }
}