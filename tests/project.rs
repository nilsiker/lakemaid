use lakemaid::create_class_diagram_string;

#[test]
fn project() {
    let mut file_readings = vec![];
    let paths = std::fs::read_dir("src").unwrap();
    for path in paths {
        if path.as_ref().unwrap().path().is_dir() {
            continue;
        }
        let input = std::fs::read_to_string(path.unwrap().path()).expect("can find readable file");
        file_readings.push(input);
    }
    let diagram_string = file_readings
        .into_iter()
        .map(create_class_diagram_string)
        .collect::<Vec<String>>()
        .join("\n");

    assert_eq!("", "", ""); // TODO create actual assert
    std::fs::write("out/project.md", diagram_string).expect("can write to out/project.md");
}
