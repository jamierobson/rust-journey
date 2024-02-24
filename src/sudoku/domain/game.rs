use super::cell_grid::CellGrid;

pub struct Game {
    pub cell_grid: CellGrid,
}

impl Game {
    pub fn new() -> Self {
        Self {
            cell_grid: CellGrid::new()
        }
    }
}