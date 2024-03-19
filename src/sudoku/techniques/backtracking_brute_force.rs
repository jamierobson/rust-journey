use crate::sudoku::core::cell_grid::CellReference;
use crate::sudoku::core::consts::{PUZZLE_DIMENTION, PUZZLE_MAXIMUM_VALUE, PUZZLE_TOTAL_CELL_COUNT};
use crate::sudoku::core::puzzle::Puzzle;
use crate::pretty::aliases::{Iteratable, Vector};
use crate::sudoku::core::validatable_units::PuzzleValidator;

use super::conjugate_groups::solve_conjugate_groups;

pub fn solve_by_brute_force(sudoku: &mut Puzzle) {

    assert!(sudoku.is_valid());

    let mut i = 0;
    loop {
        i+=1;

        solve_conjugate_groups(sudoku);
        if sudoku.is_complete() {
            return;
        }

        // Somehow we got into a bad state. This should be an error not a panic. Will change later
        assert!(sudoku.is_valid());

        let mut cloned = sudoku.clone();
        let trial_cell = sudoku.cell_grid.grid.iterate().flat_map(|grid| grid.iter()).filter(|cell| cell.borrow().value.is_none()).nth(0);
        
        // If we are not complete, but can't identify a cell to try, that's a problem.
        // Will turn into error later
        assert!(trial_cell.is_some());

        let trial_cell = trial_cell.unwrap().clone();
        let binding = trial_cell.borrow();
        let trial_value = binding.potentially_valid_values.iterate().nth(0);

        // Again, if there's no valid tries, we're in trouble. Will make an error return type
        assert!(trial_value.is_some());
        let trial_value = trial_value.unwrap();

        if !try_solve(&mut sudoku.clone(), &mut trial_cell.clone(), *trial_value, &mut i) {
            trial_cell.borrow_mut().discount_value(*trial_value);
        }

        if i >= PUZZLE_TOTAL_CELL_COUNT * PUZZLE_DIMENTION {
            println!("Gave up brute force attempt after {} iterations", i);
            // Should never happen, but ensure we terminate
            break;
        };
    }
}

// Take a first pass.
// Clone the grid, try a value. If we run into a logical inconsistency, then we can wipe that from the potential values
// If we can't solve, try place another. Continue until solved or paradox. Then the original guess can be struck off the caller

// fn try_solve(sudoku: &Puzzle) -> bool {
fn try_solve(sudoku: &mut Puzzle, trial_cell: &mut CellReference, trial_value: u8, i: &mut usize) -> bool {
        let first_empty_cell = sudoku.cell_grid.grid.iterate().flat_map(|grid| grid.iter()).filter(|cell| cell.borrow().value.is_none()).nth(0);

        if first_empty_cell.is_none(){
            return sudoku.is_valid();
        }

        let trial_cell = first_empty_cell.unwrap().clone();

        // only here because otherwise "temporary value is freed at the end of this statement". 
        // I don't get it - it looks like trial_cell and trial_value would still be in scope, to me
        let binding = trial_cell.borrow(); 
        let trial_value = binding.potentially_valid_values.iter().nth(0);
        if trial_value.is_none() {
            return false;
        }

        trial_cell.borrow_mut().set_value(*trial_value.unwrap());

        if sudoku.is_complete() {
            return true;
        }

        if !sudoku.is_valid(){
            return false;
        }

        return try_solve(sudoku);
}


// Idea is:
// Clone the grid, try something. See how much we can solve automatically. 
// If the puzzle gets to an invalid state, then the caller knows its guess was bad.