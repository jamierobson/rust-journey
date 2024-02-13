mod hello_world;
mod sudoku;

fn main() {
    hello_world::greeter::say_hello();
    let mut cell = sudoku::domain::cell::Cell::new();

    write_all_elements_to_console("new cell all candidates", &cell.candidates);
    write_all_elements_to_console("new cell all discounted", &cell.discounted_values);
    write_cell_value("created", &cell.value);

    cell.add_candidate(3);
    cell.add_candidate(3);
    cell.discount_value(6);

    write_all_elements_to_console("add 3 discount 6 remaining candidates", &cell.candidates);
    write_all_elements_to_console("add 3 discount 6 remaining discounted", &cell.discounted_values);
    
    cell.set_value(4);
    write_all_elements_to_console("add 3 and 4, discount 6 remaining candidates", &cell.candidates);
    write_all_elements_to_console("add 3 and 4, discount 6 remaining discounted", &cell.discounted_values);
    write_cell_value("set value = 4", &cell.value);
}

fn write_cell_value(after_operation: &str, value: &Option<u8>) {
    let value_to_display = match value {
        None => "not set".into(),
        _ => value.as_ref().unwrap().to_string()
    };

    print!("Cell value after {} is {}\n", after_operation, value_to_display);
}

fn write_all_elements_to_console(vector_name: &str, vector: &Vec<u8>){
    print!("Contents of {} ", vector_name);
    print!("{:?} \n", vector);
}