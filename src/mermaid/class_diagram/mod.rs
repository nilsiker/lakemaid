pub mod nodes;
pub mod rs;

use std::collections::HashSet;

use nodes::{Class, Enum};

pub struct ClassDiagram {
    pub classes: Vec<Class>,
    pub enums: Vec<Enum>,
    pub relationships: HashSet<Relationship>,
}

impl From<ClassDiagram> for String {
    fn from(
        ClassDiagram {
            classes,
            relationships,
            enums,
        }: ClassDiagram,
    ) -> Self {
        let classes_str: String = classes
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("\n");
        let mut relationships_vec: Vec<String> = relationships
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>();
        relationships_vec.sort();
        let relationships_str = relationships_vec.join("\n");
        let enums_str = enums
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r"```
mermaid
classDiagram
{classes_str}
{enums_str}
{relationships_str}
```"
        )
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Relationship {
    Association(String, String),
    Composition(String, String),
}
impl From<Relationship> for String {
    fn from(value: Relationship) -> Self {
        match value {
            Relationship::Association(from, to) => format!("{from}-->{to}"),
            Relationship::Composition(from, to) => format!("{from}*--{to}"),
        }
    }
}
