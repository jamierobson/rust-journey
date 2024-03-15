use crate::{pretty::aliases::*, sudoku::core::{cell_grid::CellReference, consts::PUZZLE_TOTAL_CELL_COUNT}};
use std::{cell, collections::{hash_map, HashMap}, ops::Add};

use crate::sudoku::core::{cell::Cell, game::Game, validatable_units::CellGroup};

use super::implicitly_solved;

pub fn eliminate_candidates_from_closed_groups(game: &mut Game) {

    implicitly_solved::set_solved_cells(game);
    game.cell_grid.grid.iter().flat_map(|group| group).for_each(|cell| cell.borrow_mut().potentially_valid_values.sort());

    let mut i = 0;
    loop {
        i+=1;

        let any_eliminated = try_eliminate_closed_candidate_groups(game);

        if !any_eliminated {
            println!("Finished eliminating candidates from closed groups after {} iterations", i);
            break;
        }

        implicitly_solved::set_solved_cells(game);

        if i >= PUZZLE_TOTAL_CELL_COUNT * PUZZLE_TOTAL_CELL_COUNT {
            println!("Gave up eliminating candidates from closed groups after {} iterations", i);
            // Should never happen, but ensure we terminate
            break;
        }
    }
}

fn try_eliminate_closed_candidate_groups(game: &mut Game) -> bool {

    return 
    try_naive_eliminate_options_from_groups(&mut game.rows)
    || try_naive_eliminate_options_from_groups(&mut game.columns)
    || try_naive_eliminate_options_from_groups(&mut game.blocks);
}

fn try_naive_eliminate_options_from_groups(cell_group_vector: &mut Vector<CellGroup>) -> bool {

    // Group cells by their potential values, so that we can identify if, for example, 
    // two cells both have the exact same 2 candidates, 
    // meaning that none other of the cells could have those

    let mut any_eliminated = false;
    
    for cell_group in cell_group_vector {
        
        let mut dictionary : HashMap<String, usize>= HashMap::new();
        
        let all_keys: Vector<_> = cell_group.cells.iter().filter(|cell| cell.borrow().value.is_none()).map(|cell| key_from_potential_values(&cell)).collect();
        for key in all_keys.clone() {
            dictionary.insert(key, 0);
        }

        for cell_reference in &cell_group.cells {
            if cell_reference.borrow().value.is_some(){
                continue;
            }

            all_keys.iter()
            .filter(|key| cell_reference.borrow().potentially_valid_values.iter().all(|c| key.contains(&c.to_string())))
            .for_each(|key| {
                if let Some(count) = dictionary.get(key) {
                    dictionary.insert(key.clone(), *count + 1);
                }
            });
        }

        for dictionary_entry in dictionary {
           
            let values_to_discount = values_from_key(&dictionary_entry.0);
            
            if values_to_discount.len() != dictionary_entry.1
            {
                continue;
            }

            if cell_group.cells.len() == dictionary_entry.1 {
                continue;
            }

            println!("cycle identified for key {}", &dictionary_entry.0);
 
            for cell in cell_group.cells.iter() {

                // if cell is one of the cells in the group then we shouldn't alter its candidates
                // Only if a cell is actually mutated in some way do we want to report that something happened
                // and that it is work continuing another iteration
                if cell.borrow().potentially_valid_values.iter().all(|c| dictionary_entry.0.contains(&c.to_string())) {
                    continue;
                }

                any_eliminated = true;
                cell.borrow_mut().discount_values(&values_to_discount);
            }

            // Cycle identified
            // println!("cycle identified for key {}", dictionary_entry.0);
            // Work out: Which cells to try to eliminate from
            // Work out: Were any eliminated for any cells? If so, set any_eliminated to true

        }

    }

    return any_eliminated;
}

const SEPARATOR: &str = "-";

fn key_from_potential_values(potentials: &CellReference) -> String {
    return potentials.borrow().potentially_valid_values
    .iterate()
    .map(|value| value.to_string())
    .collect::<Vector<_>>()
    .join(SEPARATOR);
}

fn key_element_count(key: &String) -> usize {
    if key.len() == 0 {
        return 0;
    }

    return key.matches(SEPARATOR).count() + 1;
}

fn values_from_key(key: &String) -> Vec<u8> {
    return key.split(SEPARATOR).map(|s| s.parse::<u8>().expect("the key was constructed from values of this type")).collect();
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sudoku::{core::{cell::Cell, cell_grid::CellReference, validatable_units::CellGroup}, draw::terminal_print::draw_all_rows, format::serializer::Serializer, techniques::implicitly_solved::set_solved_cells};

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
        let some_eliminated = try_naive_eliminate_options_from_groups(&mut vec![group]);

        assert!(some_eliminated);
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
    
    #[test]
    fn try_solve_a_puzzle_with_closed_group (){
        let test_case = ".8..9..3..3.....699.2.63158.2.8.459.8519.7.463946.587.563.4.9872......15.1..5..2.";
        let mut game = Serializer::new().new_game(test_case).expect("test data is valid");
        
        let initial_cell_count = game.count_cells_with_value();
        draw_all_rows(&game.rows);

        eliminate_candidates_from_closed_groups(&mut game);
        
        draw_all_rows(&game.rows);
        println!("before: {}, after: {}", initial_cell_count, game.count_cells_with_value());
    }
}