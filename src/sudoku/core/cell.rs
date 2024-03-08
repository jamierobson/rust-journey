use super::consts;

#[derive(Default, Debug)]
pub struct Cell {
    pub value: Option<u8>,
    pub candidates: Vec<u8>,
    pub discounted_values: Vec<u8>,
    potentially_valid_values: Vec<u8>
}

impl Cell {
    pub fn new() -> Self {
        Self::default()
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
        self.candidates.clear();
        self.discounted_values.clear();
        self.potentially_valid_values.clear();
    }

    pub fn discount_value(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }

        add_to_collection(&mut self.discounted_values, value);
        remove_from_collection(&mut self.candidates, value);
        remove_from_collection(&mut self.potentially_valid_values, value);
    }

    pub fn add_candidate(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }
        add_to_collection(&mut self.candidates, value);
        add_to_collection(&mut self.potentially_valid_values, value);
        remove_from_collection(&mut self.discounted_values, value);
    }

    pub fn remove_candidate(&mut self, value: u8) {
        if !is_valid_cell_value(value){
            return;
        }
        remove_from_collection(&mut self.candidates, value);
        remove_from_collection(&mut self.potentially_valid_values, value);
    }
}


fn remove_from_collection<T>(collection: &mut Vec<T>, value: T) where T: PartialEq {
    collection.retain(|x| *x != value);
}

fn add_to_collection<T>(collection : &mut Vec<T>, value: T) where T: PartialEq {
    if!collection.contains(&value) {
        collection.push(value);
    }
}

fn is_valid_cell_value(value: u8) -> bool{
    return 1 <= value && value <= consts::PUZZLE_MAXIMUM_VALUE
}

#[cfg(test)]
mod tests {
    use self::consts::PUZZLE_MAXIMUM_VALUE;
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
}