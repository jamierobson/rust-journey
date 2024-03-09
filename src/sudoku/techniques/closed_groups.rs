use crate::sudoku::core::{game::Game, validatable_units::CellGroup};

pub fn eliminate_closed_candidate_groups(game: &mut Game) {
    eliminate_options_from_groups(&mut game.rows);
    eliminate_options_from_groups(&mut game.columns);
    eliminate_options_from_groups(&mut game.blocks);
}

fn eliminate_options_from_groups(collection: &mut Vec<CellGroup>) {
    for group in collection {
        
    }
}