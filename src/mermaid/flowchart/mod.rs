pub struct Flowchart {
    nodes: Vec<String>,
}

impl From<Flowchart> for String {
    fn from(value: Flowchart) -> Self {
        let nodes = value.nodes.join("\n");

        format!(
            r"
```mermaid
flowchart
{nodes}
```",
        )
    }
}
