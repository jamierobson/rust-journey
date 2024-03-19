use crate::sudoku::core::puzzle::Puzzle;

use super::conjugate_groups::solve_conjugate_groups;

pub fn solve_by_brute_force(sudoku: &mut Puzzle) {
    
    solve_conjugate_groups(sudoku);

    let cloned = sudoku.clone();
}