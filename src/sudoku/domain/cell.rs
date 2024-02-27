use super::consts;

#[derive(Default)]
pub struct Cell {
    pub value: Option<u8>,
    pub candidates: Vec<u8>,
    pub discounted_values: Vec<u8>,
    potentially_valid_values: Vec<u8> //todo: just exploring if you can have private members
}

impl Cell {
    pub fn new() -> Self {
        return Self {
            value: None, 
            candidates: Vec::new(),
            discounted_values: Vec::new(),
            potentially_valid_values: (0..=consts::MAXIMUM_VALUE).collect()
        }
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
    return 1 <= value && value <= consts::MAXIMUM_VALUE
}