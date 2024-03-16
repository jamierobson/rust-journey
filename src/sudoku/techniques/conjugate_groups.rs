use crate::{pretty::{aliases::*, iterable::*},  sudoku::core::{cell_grid::CellReference, consts::PUZZLE_TOTAL_CELL_COUNT}};
use std::{cell, collections::{hash_map, HashMap}, ops::Add};

use crate::sudoku::core::{cell::Cell, game::Game, validatable_units::CellGroup};

use super::implicitly_solved;

pub fn eliminate_candidates_from_closed_groups(game: &mut Game) {

    implicitly_solved::set_solved_cells(game);
    game.cell_grid.grid.iterate().flat_map(|group| group).for_each(|cell| cell.borrow_mut().potentially_valid_values.sort());

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
    try_eliminate_options_from_groups(&mut game.rows)
    || try_eliminate_options_from_groups(&mut game.columns)
    || try_eliminate_options_from_groups(&mut game.blocks);
}

fn try_eliminate_options_from_groups(cell_group_vector: &mut Vector<CellGroup>) -> bool {

    let mut any_eliminated = false;
    
    for cell_group in cell_group_vector {
        
        let mut dictionary : HashMap<&Vec<u8>, usize>= HashMap::new();
        
        let cells_to_calculate_over: Vector<_> = cell_group.cells.iterate().filter(|cell| cell.borrow().value.is_none()).collect();

        let mut all_keys: Vector<_> = cells_to_calculate_over.iterate().map(|cell| cell.borrow().potentially_valid_values.clone()).collect();
        all_keys.dedup();
        for key in &all_keys {
            dictionary.insert(key, 0);
        }

        for cell_reference in &cells_to_calculate_over {

           all_keys
           .iterate()
           .filter(|key| key.is_superset_of(&cell_reference.borrow().potentially_valid_values))
           .for_each(|matched_key| {
                let count = dictionary.get(matched_key).expect("We are iterating over the keys that seeded the HashMap");
                dictionary.insert(&matched_key, count + 1);
           });

        }

        for dictionary_entry in dictionary {
           
            if dictionary_entry.0.len() != dictionary_entry.1
            {
                continue;
            }

            if cells_to_calculate_over.len() == dictionary_entry.1 {
                continue;
            }

            println!("cycle identified for key [{:?}]", &dictionary_entry.0);
 
            for cell in cells_to_calculate_over.iterate() {

                // if cell is one of the cells in the group then we shouldn't alter its candidates
                // Only if a cell is actually mutated in some way do we want to report that something happened
                // and that it is work continuing another iteration
                if cell.borrow().potentially_valid_values.is_subset_of(&dictionary_entry.0){
                    continue;
                }

                any_eliminated = cell.borrow_mut().discount_values(dictionary_entry.0) || any_eliminated;
            }
        }
    }

    return any_eliminated;
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::sudoku::{core::{cell::Cell, cell_grid::CellReference, validatable_units::{CellGroup, GameStateValidator}}, draw::terminal_print::draw_all_rows, format::serializer::Serializer, techniques::implicitly_solved::set_solved_cells};

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
            Rc::clone(&cell_a),
            Rc::clone(&cell_b),
            Rc::clone(&cell_c),
            Rc::clone(&cell_d)
        ];

        let group = CellGroup::new(references);
        let some_eliminated = try_eliminate_options_from_groups(&mut vec![group]);

        assert!(some_eliminated);
        assert_eq!(Rc::clone(&cell_c).borrow().potentially_valid_values, vec![7]);
        assert_eq!(Rc::clone(&cell_d).borrow().potentially_valid_values, vec![2,3,4,5,6])
    }
    
    #[test]
    fn try_solve_a_puzzle_with_only_single_iteration_of_closed_group_strategy (){
        let test_case = ".8..9..3..3.....699.2.63158.2.8.459.8519.7.463946.587.563.4.9872......15.1..5..2.";
        let mut game = Serializer::new().new_game(test_case).expect("test data is valid");
        
        let initial_cell_count = game.count_cells_with_value();
        draw_all_rows(&game.rows);

        eliminate_candidates_from_closed_groups(&mut game);
        
        draw_all_rows(&game.rows);
        println!("before: {}, after: {}", initial_cell_count, game.count_cells_with_value());

        assert!(game.is_complete())
    }
    
    #[test]
    fn try_solve_puzzle_with_multiple_iterations_of_closed_group_strategy (){
        let test_case = "251348796...9172.4..7256.......6.832.......7...8...9.....62...88..7.......25.164.";
        let mut game = Serializer::new().new_game(test_case).expect("test data is valid");
        
        let initial_cell_count = game.count_cells_with_value();
        draw_all_rows(&game.rows);

        eliminate_candidates_from_closed_groups(&mut game);
        
        draw_all_rows(&game.rows);
        println!("before: {}, after: {}", initial_cell_count, game.count_cells_with_value());

        assert!(game.is_complete())
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
        println!("{}", Serializer::new().serialize_to_string(&game));
        assert!(game.count_cells_with_value() > initial_cell_count);
    }
}