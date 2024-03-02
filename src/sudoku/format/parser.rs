// It's hard to find formatting information, so we'll create one for now and hope for the best.
// The standard format is in the form [1-9.]{81}
// I'm imagining that we eventually support other sizes, in which case we will cover that when we get there.

use regex::Regex;

use crate::sudoku::domain::cell_grid::CellGrid;

const NINE_X_NINE_CELL_REGEX: &str = "^[1-9.]{81}$";

pub struct Parser {
    regex: Regex
}

impl Parser {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(NINE_X_NINE_CELL_REGEX).expect("expecting a valid regex here")
        }
    }

    pub fn can_parse(&self, input: &str) -> bool {
        return self.regex.is_match(input);
    }

    pub fn to_grid(&self, input: &str) -> Result<CellGrid, &str> {
        
        if !self.can_parse(input) {
            return Err("The input {input} wasn't understood as notation for a sudoku game. Expected a string matching {NINE_X_NINE_CELL_REGEX}");
        }

        let mut cell_grid = CellGrid::new();

        return Ok(cell_grid);
    }
}

#[cfg(test)]
mod tests {
    use crate::sudoku::domain::consts::{self, PUZZLE_TOTAL_CELL_COUNT};
    use super::*;

    fn repeat_value_times(repeat_this: &str, times: usize) -> String {
        let repeated_values:Vec<String> = std::iter::repeat(repeat_this.to_string()).take(times).collect();
        return repeated_values.join("");
    }

    #[test]
    fn can_parse_return_true_when_81_periods() {
        let parser = Parser::new();
        let can_parse = parser.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT));
        assert!(can_parse);
    }

    #[test]
    fn can_parse_return_false_when_not_exactly_81_elements() {
        let parser = Parser::new();
        let can_parse_80 = parser.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT - 1));
        assert_eq!(can_parse_80, false);
        let can_parse_82 = parser.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT + 1));
        assert_eq!(can_parse_82, false);
    }

    #[test]
    fn can_parse_return_true_when_81_digits() {
        let parser = Parser::new();

        for value in 1..=consts::PUZZLE_MAXIMUM_VALUE {
            let can_parse = parser.can_parse(&repeat_value_times(&value.to_string(), PUZZLE_TOTAL_CELL_COUNT));
            assert!(can_parse);
        }
    }

    #[test]
    fn can_parse_return_true_when_any_mix_of_digits_and_periods() {
        let parser = Parser::new();
        let can_parse = parser.can_parse("123..............456..................789.................147..........258....369");
        assert!(can_parse);
    }

    #[test]
    fn can_parse_return_false_when_any_invalid_character() {
        let parser = Parser::new();
        let can_parse = parser.can_parse(&repeat_value_times("a", PUZZLE_TOTAL_CELL_COUNT));
        assert_eq!(can_parse, false);
    }

    #[test]
    fn to_grid_returns_err_when_invalid_input() {
        let parser = Parser::new();
        let cell_grid_result = parser.to_grid("invalid");
        assert!(cell_grid_result.is_err());
    }

    #[test]
    fn to_grid_returns_empty_grid_when_all_periods() {
        let parser = Parser::new();
        let empty = repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT);
        let cell_grid_result = parser.to_grid(&empty);
        assert!(cell_grid_result.is_ok());
        cell_grid_result.unwrap().grid.iter().flat_map(|x| x.iter()).for_each(|cell_ref| assert_eq!(cell_ref.borrow().value, None))
    }
}