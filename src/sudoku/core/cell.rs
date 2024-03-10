use std::{collections::btree_map::Range, ops::{RangeBounds, RangeInclusive}};

use crate::pretty::aliases::*;
use super::consts::PUZZLE_MAXIMUM_VALUE;

#[derive(Debug)]
pub struct Cell {
    pub value: Option<u8>,
    pub discounted_values: Vector<u8>,
    pub potentially_valid_values: Vector<u8>
}

impl Cell {
    pub fn new() -> Self {
        Self {
            value: None,
            discounted_values: Vector::new(),
            potentially_valid_values: (1..=PUZZLE_MAXIMUM_VALUE).collect()
        }
    }

    pub fn from_value(value: Option<u8>) -> Self {
        let mut cell = Cell::new();

        if let Some(value) = value {
            cell.set_value(value);
        }

        return cell;
    }

    pub fn set_value(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }
        self.value = Some(value);
        self.discounted_values.clear();
        self.potentially_valid_values.clear();
    }

    pub fn discount_value(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }

        add_to_collection(&mut self.discounted_values, value);
        remove_from_collection(&mut self.potentially_valid_values, value);
    }

    pub fn discount_values(&mut self, values: impl AsRef<[u8]>) {
        for &value in values.as_ref() {
            self.discount_value(value)
        }
    }

    pub fn discount_range(&mut self, range: impl Iterator<Item = u8>) {
        self.discount_values(range.collect::<Vector<u8>>());
    }

    pub fn add_candidate(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }
        add_to_collection(&mut self.potentially_valid_values, value);
        remove_from_collection(&mut self.discounted_values, value);
    }

    pub fn try_complete(&mut self) {
        if self.potentially_valid_values.len() == 1 {
            self.set_value(self.potentially_valid_values[0]);
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}

fn remove_from_collection<T>(collection: &mut Vector<T>, value: T) where T: PartialEq {
    collection.retain(|x| *x != value);
}

fn add_to_collection<T>(collection : &mut Vector<T>, value: T) where T: PartialEq {
    if!collection.contains(&value) {
        collection.push(value);
    }
}

fn is_valid_cell_value(value: u8) -> bool{
    return (1..=PUZZLE_MAXIMUM_VALUE).contains(&value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_value_none_by_default() {

        let cell = Cell::new();
        assert_eq!(cell.value, None);
    }

    #[test]
    fn cell_set_provided_value_when_in_range() {

        let mut cell = Cell::new();
        for value in 1..=PUZZLE_MAXIMUM_VALUE{
            cell.set_value(value);
            assert_eq!(cell.value.unwrap(), value);
        }
    }

    #[test]
    fn cell_not_set_value_when_out_of_range() {

        const OUT_OF_RANGE: u8 = PUZZLE_MAXIMUM_VALUE + 1;
        let mut cell = Cell::new();
        cell.set_value(OUT_OF_RANGE);
        assert_eq!(cell.value, None);

        const EXPECTED: u8 = PUZZLE_MAXIMUM_VALUE - 1;

        cell.set_value(EXPECTED);
        cell.set_value(OUT_OF_RANGE);
        assert_eq!(cell.value.unwrap(), EXPECTED);
    }

    #[test]
    fn try_complete_value_sets_correct_value() {

        for expected in 1..=PUZZLE_MAXIMUM_VALUE {
            let mut cell = Cell::new();
            let discounted_values: Vector<u8> = (1..=PUZZLE_MAXIMUM_VALUE).filter(|&x| x != expected).collect();
            cell.discount_values(discounted_values);
            cell.try_complete();

            assert!(cell.value.is_some());
            assert_eq!(cell.value.unwrap(), expected);
        }
    }

    #[test]
    fn try_complete_value_does_not_set_value_when_more_than_one_missing_value() {

        let mut cell = Cell::new();
        cell.discount_range(3..=PUZZLE_MAXIMUM_VALUE);
        assert!(cell.value.is_none());
    }

    #[test]
    fn from_value_give_cell_with_set_value() {
        let cell = Cell::from_value(Some(5));
        assert_eq!(cell.value.unwrap_or_default(), 5);
    }

    #[test]
    fn from_value_give_cell_with_none_when_out_of_range() {
        let cell = Cell::from_value(Some(PUZZLE_MAXIMUM_VALUE + 1));
        assert!(cell.value.is_none());
    }

    #[test]
    fn from_value_give_cell_with_none_when_none_given() {
        let cell = Cell::from_value(None);
        assert!(cell.value.is_none());
    }
}