```mermaid
classDiagram
direction LR
class Data {
    - info: Info
    - notes: &Note
}
class Info {
    - inner: String
}
class Note {
    - contents: String
}

Data*--Info
Data-->Note
```