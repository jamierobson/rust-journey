use crate::pretty::aliases::*;
use crate::sudoku::core::{consts::PUZZLE_TOTAL_CELL_COUNT, game::Game, validatable_units::CellGroup};

pub fn set_solved_cells(game: &mut Game) {
    
    let mut i = 0;
    loop {
        i+=1;

        let pre_iteration_completed_cell_count = game.count_cells_with_value();
        try_complete_all_cells(game);

        if game.count_cells_with_value() == pre_iteration_completed_cell_count {
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

fn try_complete_all_cells(game: &mut Game){
    eliminate_options_from_groups(&mut game.rows);
    eliminate_options_from_groups(&mut game.columns);
    eliminate_options_from_groups(&mut game.blocks);
    game.cell_grid.grid.iterate().flatten().for_each(|rc| rc.borrow_mut().try_complete())
}


fn eliminate_options_from_groups(collection: &mut Vector<CellGroup>){
    for group in collection {
        
        let used_values: Vector<u8> = group.cells.iterate().filter_map(|rc| rc.borrow().value).collect();
        group.cells.iterate().for_each(|rc| rc.borrow_mut().discount_values(&used_values));
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{core::validatable_units::GameStateValidator, draw::terminal_print::{draw_all_cells, draw_all_rows}, format::serializer::Serializer};

    use super::*;

    #[test]
    fn solves_single_missing_cell(){

        let trivial_puzzle = "534678912672195348198342567859761423426853791713924856961537284287419635345.86179";
        let mut game = Serializer::new().new_game(trivial_puzzle).expect("the test data should be correct");

        assert!(!game.is_complete());
        assert!(game.cell_grid[8][3].borrow().value.is_none());

        set_solved_cells(&mut game);
        
        assert!(game.is_complete());
        assert_eq!(game.cell_grid[8][3].borrow().value.unwrap(), 2);

    }

    #[test]
    fn solves_extremely_easy_with_40_spaces() {

        let puzzle = "5834.7.2...7...453.61.258...94.....5.5..63.14..68....7........8.3564..9..792385..";
        let mut game: Game = Serializer::new().new_game(puzzle).expect("the test data should be correct");
        draw_all_rows(&game.rows);
        set_solved_cells(&mut game);
        draw_all_rows(&game.rows);

        assert!(game.is_complete());
    }

    #[test]
    fn solves_very_easy_with_51_spaces() {

        let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
        let mut game: Game = Serializer::new().new_game(puzzle).expect("the test data should be correct");
        draw_all_rows(&game.rows);
        set_solved_cells(&mut game);
        draw_all_rows(&game.rows);

        assert!(game.is_complete());
    }

    #[test]
    fn fills_in_many_blanks_in_medium_puzzle_with_47_spaces() {
        let puzzle = "...97564...13..572.7....8....27....3..7..32..8..6.2..74.9....6.7..8..1.4286.34...";
        let mut game: Game = Serializer::new().new_game(puzzle).expect("the test data should be correct");
        draw_all_rows(&game.rows);

        let initial_cell_count = game.count_cells_with_value();
        set_solved_cells(&mut game);
        draw_all_cells(&game.cell_grid);
        assert!(game.count_cells_with_value() > initial_cell_count);
        println!("before: {}, after: {}", initial_cell_count, game.count_cells_with_value());
    }
}