#[cfg(test)]
use lakemaid::create_class_diagram_string;
use std::path::PathBuf;

#[test]
fn simple_class() {
    let (output, key) = get_output_and_key("simple_class");
    assert_eq!(output, key);
}

#[test]
fn simple_relationships() {
    let (output, key) = get_output_and_key("simple_relationships");

    assert_eq!(output, key);
}

#[test]
fn class_with_generics() {
    let (output, key) = get_output_and_key("class_with_generics");
    assert_eq!(output, key);
}

fn get_output_and_key(path: &str) -> (String, String) {
    let input = read(&format!("{path}.rs"));
    let output = create_class_diagram_string(input);
    let key = read_key(&format!("{path}_key.md"));
    std::fs::write("out/".to_owned() + path + ".md", &output).expect("can write to output");
    (output, key)
}

fn read(path: &str) -> String {
    let dir: PathBuf = "tests/in/".into();
    let full_path = dir.join(path);
    std::fs::read_to_string(full_path)
        .expect(&format!("existing file '{}' in directory 'in'", path))
}

fn read_key(path: &str) -> String {
    let dir: PathBuf = "tests/keys/".into();
    let full_path = dir.join(path);
    std::fs::read_to_string(full_path)
        .expect(&format!("existing file '{}' in directory 'in'", path))
}
