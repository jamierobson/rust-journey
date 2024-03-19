use crate::pretty::aliases::*;
use crate::sudoku::core::{consts::PUZZLE_TOTAL_CELL_COUNT, puzzle::Puzzle, validatable_units::CellGroup};

pub fn solve_single_candidate_cells(sudoku: &mut Puzzle) {
    
    let mut i = 0;
    loop {
        i+=1;

        let pre_iteration_completed_cell_count = sudoku.count_cells_with_value();
        try_complete_all_cells(sudoku);

        if sudoku.count_cells_with_value() == pre_iteration_completed_cell_count {
            println!("Finished filling in implicitely solved cells after {} iterations", i);
            break;
        }

        if i >= PUZZLE_TOTAL_CELL_COUNT * PUZZLE_TOTAL_CELL_COUNT {
            println!("Gave up filling in implicitely solved cells after {} iterations", i);
            // Should never happen, but ensure we terminate
            break;
        }
    }
}

fn try_complete_all_cells(sudoku: &mut Puzzle){
    eliminate_options_from_groups(&mut sudoku.rows);
    eliminate_options_from_groups(&mut sudoku.columns);
    eliminate_options_from_groups(&mut sudoku.blocks);
    sudoku.cell_grid.grid.iterate().flatten().for_each(|rc| rc.borrow_mut().try_complete())
}


fn eliminate_options_from_groups(collection: &mut Vector<CellGroup>){
    for group in collection {
        
        let used_values: Vector<u8> = group.cells.iterate().filter_map(|rc| rc.borrow().value).collect();
        group.cells.iterate().for_each(|rc| {rc.borrow_mut().discount_values(&used_values);});
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{core::validatable_units::PuzzleValidator, draw::terminal_print::draw_all_rows, format::serializer::Serializer};

    use super::*;

    #[test]
    fn solves_single_missing_cell(){

        let trivial_puzzle = "534678912672195348198342567859761423426853791713924856961537284287419635345.86179";
        let mut sudoku = Serializer::new().new_puzzle(trivial_puzzle).expect("the test data should be correct");

        assert!(!sudoku.is_complete());
        assert!(sudoku.cell_grid[8][3].borrow().value.is_none());

        solve_single_candidate_cells(&mut sudoku);
        
        assert!(sudoku.is_complete());
        assert_eq!(sudoku.cell_grid[8][3].borrow().value.unwrap(), 2);

    }

    #[test]
    fn solves_extremely_easy_with_40_spaces() {

        let puzzle = "5834.7.2...7...453.61.258...94.....5.5..63.14..68....7........8.3564..9..792385..";
        let mut sudoku: Puzzle = Serializer::new().new_puzzle(puzzle).expect("the test data should be correct");
        draw_all_rows(&sudoku.rows);
        solve_single_candidate_cells(&mut sudoku);
        draw_all_rows(&sudoku.rows);

        assert!(sudoku.is_complete());
    }

    #[test]
    fn solves_very_easy_with_51_spaces() {

        let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
        let mut sudoku: Puzzle = Serializer::new().new_puzzle(puzzle).expect("the test data should be correct");
        draw_all_rows(&sudoku.rows);
        solve_single_candidate_cells(&mut sudoku);
        draw_all_rows(&sudoku.rows);

        assert!(sudoku.is_complete());
    }

    #[test]
    fn fills_in_many_blanks_in_medium_puzzle_with_47_spaces() {
        let puzzle = "...97564...13..572.7....8....27....3..7..32..8..6.2..74.9....6.7..8..1.4286.34...";
        let mut sudoku: Puzzle = Serializer::new().new_puzzle(puzzle).expect("the test data should be correct");
        draw_all_rows(&sudoku.rows);

        let initial_cell_count = sudoku.count_cells_with_value();
        solve_single_candidate_cells(&mut sudoku);
        draw_all_rows(&sudoku.rows);
        assert!(sudoku.count_cells_with_value() > initial_cell_count);
        println!("before: {}, after: {}", initial_cell_count, sudoku.count_cells_with_value());
    }
}