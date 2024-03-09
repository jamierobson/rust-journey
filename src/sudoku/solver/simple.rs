use crate::sudoku::core::{game::Game, validatable_units::CellGroup};

pub fn set_solved_cells(game: &mut Game) {
    let pre_iteration_completed_cell_count = game.count_cells_with_value();
    
    try_complete_all_cells(game);

    if game.count_cells_with_value() > pre_iteration_completed_cell_count {
        set_solved_cells(game);
    }
}

pub fn try_complete_all_cells(game: &mut Game){
    eliminate_options_from_groups(&mut game.rows);
    eliminate_options_from_groups(&mut game.columns);
    eliminate_options_from_groups(&mut game.blocks);
    game.cell_grid.grid.iter().flatten().for_each(|rc| rc.borrow_mut().try_complete())
}


fn eliminate_options_from_groups(collection: &mut Vec<CellGroup>){
    for group in collection {
        let living_cell_reference_iterator = group.cells.iter().filter_map(|weak| weak.upgrade());
        let used_values: Vec<u8> = living_cell_reference_iterator.clone().filter_map(|rc| rc.borrow().value).collect();
        living_cell_reference_iterator.for_each(|rc| rc.borrow_mut().discount_values(&used_values));
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::{core::validatable_units::GameStateValidator, draw::terminal_print::{draw_all_cells, draw_all_rows}, format::parser::Parser};

    use super::*;

    #[test]
    fn solves_single_missing_cell(){

        let trivial_solution_representation = "534678912672195348198342567859761423426853791713924856961537284287419635345.86179";
        let mut game = Parser::new().new_game(trivial_solution_representation).expect("the test data should be correct");

        assert!(!game.is_complete());
        assert!(game.cell_grid[8][3].borrow().value.is_none());

        set_solved_cells(&mut game);
        
        assert!(game.is_complete());
        assert_eq!(game.cell_grid[8][3].borrow().value.unwrap(), 2);

    }

    #[test]
    fn solves_very_easy() {

        let very_easy = "5834.7.2...7...453.61.258...94.....5.5..63.14..68....7........8.3564..9..792385..";
        let mut game: Game = Parser::new().new_game(very_easy).expect("the test data should be correct");
        draw_all_rows(&game.rows);

        assert!(game.is_valid());
        set_solved_cells(&mut game);
        draw_all_rows(&game.rows);

        assert!(game.is_complete());
    }
}