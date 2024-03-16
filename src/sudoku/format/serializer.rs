// It's hard to find formatting information, so we'll create one for now and hope for the best.
// The standard format is in the form [1-9.]{81}
// I'm imagining that we eventually support other sizes, in which case we will cover that when we get there.

use crate::{pretty::aliases::*, sudoku::core::consts::PUZZLE_TOTAL_CELL_COUNT};
use regex::Regex;

use crate::sudoku::core::{consts::PUZZLE_DIMENTION, game::{Game, SeedGrid}};

const NINE_X_NINE_CELL_REGEX: StringSlice = "^[1-9.]{81}$";

pub struct Serializer {
    regex: Regex
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            regex: Regex::new(NINE_X_NINE_CELL_REGEX).expect("expecting a valid regex here")
        }
    }

    pub fn can_parse(&self, input: StringSlice) -> bool {
        return self.regex.is_match(input);
    }

    pub fn new_game(&self, input: StringSlice) -> Result<Game, String> {
        
        if !self.can_parse(input) {
            return Err(format!("The input '{input}' wasn't understood as notation for a sudoku game. Expected a string matching {NINE_X_NINE_CELL_REGEX}"));
        }

        let values = values_from_input(input);
        return Ok(Game::new(&values));
    }

    pub fn serialize_to_string(&self, game: &Game) -> String {
        let mut serialized = String::with_capacity(PUZZLE_TOTAL_CELL_COUNT);
        for row in 0..PUZZLE_DIMENTION {
        for column in 0..PUZZLE_DIMENTION {
            // let value = game.cell_grid[row][column].borrow().value;
    
            let push = match game.cell_grid[row][column].borrow().value {
                Some(cell_value) => cell_value.to_string(),
                None => ".".to_string(),
            };
            
            serialized.push_str(push.as_str());
        }}
    
        return serialized;
    }
}

fn values_from_input(input: StringSlice) -> SeedGrid {
    let mut grid = [[None; PUZZLE_DIMENTION]; PUZZLE_DIMENTION];
    
    for (index, c) in input.chars().enumerate() {
        let row = index / PUZZLE_DIMENTION;
        let column = index % PUZZLE_DIMENTION;
        
        match c {
            '.' => grid[row][column] = None,
            _ => {
                if let Some(value) = c.to_digit(10).map(|digit| digit as u8) {
                    grid[row][column] = Some(value);
                }
            }
        }
    }
    
    return grid;
} 

#[cfg(test)]
mod tests {
    use std::{clone};

    use crate::sudoku::{core::consts::{self, PUZZLE_MAXIMUM_VALUE, PUZZLE_TOTAL_CELL_COUNT}, format::serializer};
    use super::*;

    fn repeat_value_times(repeat_this: StringSlice, times: usize) -> String {
        let repeated_values:Vector<String> = std::iter::repeat(repeat_this.to_string()).take(times).collect();
        return repeated_values.join("");
    }

    #[test]
    fn can_parse_return_true_when_81_periods() {
        let serializer = Serializer::new();
        let can_parse = serializer.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT));
        assert!(can_parse);
    }

    #[test]
    fn can_parse_return_false_when_not_exactly_81_elements() {
        let serializer = Serializer::new();
        let can_parse_80 = serializer.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT - 1));
        assert_eq!(can_parse_80, false);
        let can_parse_82 = serializer.can_parse(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT + 1));
        assert_eq!(can_parse_82, false);
    }

    #[test]
    fn can_parse_return_true_when_81_digits() {
        let serializer = Serializer::new();

        for value in 1..=consts::PUZZLE_MAXIMUM_VALUE {
            let can_parse = serializer.can_parse(&repeat_value_times(&value.to_string(), PUZZLE_TOTAL_CELL_COUNT));
            assert!(can_parse);
        }
    }

    #[test]
    fn can_parse_return_true_when_any_mix_of_digits_and_periods() {
        let serializer = Serializer::new();
        let can_parse = serializer.can_parse("123..............456..................789.................147..........258....369");
        assert!(can_parse);
    }

    #[test]
    fn deserialze_then_serialize_returns_same_string() {
        let serializer = Serializer::new();

        let test_cases = vec![
            "123..............456..................789.................147..........258....369".to_string(),
            repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT),
            repeat_value_times("9", PUZZLE_TOTAL_CELL_COUNT),
            repeat_value_times("123456789", PUZZLE_DIMENTION)
        ];

        for expected in test_cases {
            let game = serializer.new_game(&expected).expect("test data is confirmed correct");
            let actual = serializer.serialize_to_string(&game);

            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn can_parse_return_false_when_any_invalid_character() {
        let serializer = Serializer::new();
        let can_parse = serializer.can_parse(&repeat_value_times("a", PUZZLE_TOTAL_CELL_COUNT));
        assert_eq!(can_parse, false);
    }

    #[test]
    fn to_grid_returns_err_when_invalid_input() {
        let serializer = Serializer::new();
        let cell_grid_result = serializer.new_game("invalid");
        assert!(cell_grid_result.is_err());
    }

    #[test]
    fn to_grid_returns_empty_grid_when_all_periods() {
        let serializer = Serializer::new();
        let empty = repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT);
        let result = serializer.new_game(&empty);
        assert!(result.is_ok());
        result.unwrap().cell_grid.grid.iterate().flat_map(|x| x.iterate()).for_each(|cell_ref| assert_eq!(cell_ref.borrow().value, None))
    }

    #[test]
    fn to_grid_returns_value_from_string() {
        let serializer = Serializer::new();

        let expected: u8 = 1;
        let mut string_representation = expected.to_string().as_str().to_owned();
        string_representation.push_str(&repeat_value_times(".", PUZZLE_TOTAL_CELL_COUNT-1));

        let result = serializer.new_game(&string_representation);
        
        assert!(result.is_ok());
        let found_value = result.unwrap().cell_grid.grid[0][0].borrow().value;
        assert!(found_value.is_some());
        assert_eq!(found_value.unwrap(), expected)
    }

    #[test]
    fn to_grid_returns_filled_in_grid() {
        let serializer = Serializer::new();

        let string_representation = repeat_value_times("123456789", PUZZLE_DIMENTION);
        
        let row: [Option<u8>; PUZZLE_DIMENTION] = (1..=PUZZLE_MAXIMUM_VALUE).map(|value| Some(value)).collect::<Vector<_>>().try_into().unwrap();
        let expected_values: [[Option<u8>; PUZZLE_DIMENTION]; PUZZLE_DIMENTION] = core::array::from_fn(|_i| row.clone());

        let result = serializer.new_game(&string_representation);
        assert!(result.is_ok());

        let game = result.unwrap();

        for row in 0..PUZZLE_DIMENTION {
        for column in 0..PUZZLE_DIMENTION {
            let actual = game.cell_grid[row][column].borrow().value;
            assert!(actual.is_some());
            let expected = expected_values[row][column].expect("there should be a value here according to the test setup");
            
            assert_eq!(expected, actual.unwrap());
        }}
    }

}