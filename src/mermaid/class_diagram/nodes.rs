#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
}
impl From<Class> for String {
    fn from(value: Class) -> Self {
        let mut s = format!("class {} {{\n", value.name);
        value
            .fields
            .into_iter()
            .map(String::from)
            .for_each(|field_string| {
                s += "    ";
                s += &field_string;
                s += "\n";
            });
        s + "}"
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub visibility: Visibility,
    pub name: Option<String>,
    pub ty: Type,
}
impl From<Field> for String {
    fn from(value: Field) -> Self {
        match value.name {
            Some(name) => format!(
                "{} {}: {}",
                Into::<&str>::into(value.visibility),
                name,
                String::from(value.ty)
            ),
            None => format!(
                "{} {}",
                Into::<&str>::into(value.visibility),
                String::from(value.ty)
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Visibility {
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

#[derive(Debug, Clone)]
pub struct Type {
    pub reference: bool,
    pub identifier: String,
    pub generics: Option<Vec<Type>>,
}

impl From<Type> for String {
    fn from(value: Type) -> Self {
        match value.generics {
            Some(generics) => format!(
                "{}{}~{}~",
                if value.reference { "&" } else { "" },
                value.identifier,
                generics
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            None => format!(
                "{}{}",
                if value.reference { "&" } else { "" },
                value.identifier
            ),
        }
    }
}

pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}
impl From<Enum> for String {
    fn from(value: Enum) -> Self {
        let mut string = format!("class {} {{\n<<enumeration>>\n", value.name);
        value
            .variants
            .into_iter()
            .for_each(|variant| string += &format!("    {variant}\n"));

        string + "}"
    }
}
