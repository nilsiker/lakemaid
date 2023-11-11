mod mappings;

use std::{fs::File, io::Write};

use lakemaid::create_class_diagram_string;

fn main() {
    let input = std::fs::read_to_string("in/general.rs").expect("can find readable file");

    let class_diagram_string = create_class_diagram_string(input);
    let mut file = File::create("out/general.md").unwrap();
    file.write_all(class_diagram_string.as_bytes()).unwrap();
}
