mod rs;

#[derive(Debug, Clone)]
pub struct MermaidType {
    pub reference: bool,
    pub identifier: String,
    pub generics: Option<Vec<String>>,
}
impl From<MermaidType> for String {
    fn from(value: MermaidType) -> Self {
        match value.generics {
            Some(generics) => format!(
                "{}{}~{}~",
                if value.reference { "&" } else { "" },
                value.identifier,
                generics.join(",")
            ),
            None => value.identifier,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MermaidClass {
    pub name: String,
    pub fields: Vec<MermaidField>,
}
impl From<MermaidClass> for String {
    fn from(value: MermaidClass) -> Self {
        let mut s = format!("class {} {{\n", value.name);
        value
            .fields
            .into_iter()
            .map(String::from)
            .for_each(|field_string| {
                s += &field_string;
            });
        s + "}"
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

#[derive(Debug, Clone)]
pub struct MermaidField {
    visibility: Visibility,
    pub name: Option<String>,
    pub ty: MermaidType,
}
impl From<MermaidField> for String {
    fn from(value: MermaidField) -> Self {
        match value.name {
            Some(name) => format!(
                "    {} {}: {}\n",
                Into::<&str>::into(value.visibility),
                name,
                String::from(value.ty)
            ),
            None => format!(
                "    {} {}\n",
                Into::<&str>::into(value.visibility),
                String::from(value.ty)
            ),
        }
    }
}

#[derive(Debug, Clone)]
enum Visibility {
    Public,
    Private,
    // TODO add protected? Is there a use case for rust?
}
impl From<Visibility> for &str {
    fn from(value: Visibility) -> Self {
        match value {
            Visibility::Public => "+",
            Visibility::Private => "-",
        }
    }
}

pub struct MermaidEnum {
    pub name: String,
    pub variants: Vec<String>,
}
impl From<MermaidEnum> for String {
    fn from(value: MermaidEnum) -> Self {
        let mut string = format!("class {} {{\n<<enumeration>>\n", value.name);
        value
            .variants
            .into_iter()
            .for_each(|variant| string += &format!("    {variant}\n"));

        string + "}"
    }
}
