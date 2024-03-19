use std::rc::Rc;

use crate::sudoku::core::cell_grid::CellReference;
use crate::sudoku::core::consts::{PUZZLE_DIMENTION, PUZZLE_MAXIMUM_VALUE, PUZZLE_TOTAL_CELL_COUNT};
use crate::sudoku::core::puzzle::Puzzle;
use crate::pretty::aliases::{Iteratable, Vector};
use crate::sudoku::core::validatable_units::PuzzleValidator;

use super::conjugate_groups::solve_conjugate_groups;

struct Trial {
    original_cell: CellReference,
    trial_cell: CellReference,
    trial_value: u8
}

impl Trial {
    pub fn reject(&self) {
        self.original_cell.borrow_mut().discount_value(self.trial_value);
    }

    pub fn apply_hypothesis(&self) {
        self.trial_cell.borrow_mut().set_value(self.trial_value);
    }

    pub fn accept(&self) {
        self.original_cell.borrow_mut().set_value(self.trial_value);
    }
}

/// Note to self: 
/// Take our most complex technique that does not rely on trial-and-error. See if we have solved.
/// At the top level, we need to be reasoning about a single cell at a time - 
/// Either a cell value is discounted because it leads to inconsistency, or it is correct, and then test the next cell
/// 
/// Take a clone of the puzzle. Make a guess. 
/// If after trying the other techniques no candidates would be left for some cell, it's invalid. Discount it and try again with another guess
/// If, when 
pub fn solve_by_trial_and_error(sudoku: &mut Puzzle) {

    assert!(sudoku.is_valid());

    let mut i = 0;
    loop {

        solve_conjugate_groups(sudoku);
        if sudoku.is_complete() {
            println!("Completed puzzle by trial and error after {} attempts", i);
            return;
        }

        if try_recursively(sudoku, &mut i){
            return;
        }

        if i >= PUZZLE_TOTAL_CELL_COUNT * PUZZLE_DIMENTION {
            println!("Gave up trial and error attempt after {} attempts", i);
            // Should never happen, but ensure we terminate
            break;
        };
    }
}

fn try_get_next_trial(sudoku: &Puzzle, cloned_sudoku: &Puzzle) -> Option<Trial> {
    let original_cell = sudoku.cell_grid.grid.iterate().flatten().filter(|cell| cell.borrow().value.is_none()).nth(0);
    let trial_cell = cloned_sudoku.cell_grid.grid.iterate().flatten().filter(|cell| cell.borrow().value.is_none()).nth(0);

    // we assume that original_cell and trial_cell are the same - todo: verify
    if original_cell.is_none() {
        return None;
    }

    // I want to understand the error that appears without this binding line
    // "temporary value is freed at the end of this statement". 
    let binding = trial_cell.unwrap().borrow();
    let trial_value = *binding.potentially_valid_values.iterate().nth(0).unwrap();

    return Some(Trial {
        original_cell: Rc::clone(original_cell.unwrap()),
        trial_cell: Rc::clone(trial_cell.unwrap()),
        trial_value: trial_value
    });
}

fn try_recursively(sudoku: &mut Puzzle, iterations: &mut usize) -> bool {
    *iterations += 1;
    let mut clone = sudoku.clone();

    let trial = try_get_next_trial(&sudoku, &clone);

    if trial.is_none() {
        return sudoku.is_valid();
    }

    let trial = trial.unwrap();
    trial.apply_hypothesis();
    solve_conjugate_groups(&mut clone);

    if clone.is_complete() {
        trial.accept();
        solve_conjugate_groups(sudoku);
        return true;
    }

    if !clone.is_valid() {
        trial.reject();
        return false;
    }

    let success =  try_recursively(&mut clone, iterations);
    
    match success {
        true => trial.accept(),
        false => trial.reject()
    }

    return success;
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{draw::terminal_print::draw_all_rows};

    use super::*;

    #[test]
    fn solve_by_trial_and_error_solve_empty_puzzle() {
        let mut sudoku = Puzzle::default();

        solve_by_trial_and_error(&mut sudoku);

        draw_all_rows(&sudoku.rows);
        assert!(sudoku.is_complete());
    }

    #[test]
    fn solve_by_trial_and_error_solve_puzzle_not_solved_by_other_means() {

    }
}