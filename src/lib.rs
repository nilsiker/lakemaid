use std::collections::{hash_set::IntoIter, HashSet};

use mappings::{MermaidClass, MermaidEnum, Relationship};
use syn::Item;

mod mappings;

pub struct MermaidResult {
    pub classes: Vec<MermaidClass>,
    pub enums: Vec<MermaidEnum>,
    pub relationships: HashSet<Relationship>,
}

pub fn create_class_diagram_string(input: String) -> String {
    let MermaidResult {
        classes,
        enums,
        relationships: associations,
    } = parse_src_into_mermaid(&input);

    let classes_str: String = classes
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>()
        .join("\n");
    let associations_str: String = associations
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>()
        .join("\n");
    let enums_str = enums
        .into_iter()
        .map(String::from)
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "```mermaid\nclassDiagram\ndirection LR\n{}\n{}\n{}\n```",
        classes_str, enums_str, associations_str
    )
}

fn parse_src_into_mermaid(src: &str) -> MermaidResult {
    let mut classes = Vec::new();
    let mut enums = Vec::new();
    let mut relationships = HashSet::new();
    syn::parse_file(src)
        .expect("can parse file")
        .items
        .into_iter()
        .for_each(|item| match item {
            Item::Const(_) => (),
            Item::Enum(e) => enums.push(e.into()),
            Item::ExternCrate(_) => todo!(),
            Item::Fn(_) => todo!(),
            Item::ForeignMod(_) => todo!(),
            Item::Impl(_) => (),
            Item::Macro(_) => (),
            Item::Mod(_) => (),
            Item::Static(_) => todo!(),
            Item::Struct(item_struct) => {
                let mermaid_class = Into::<MermaidClass>::into(item_struct.clone());
                classes.push(mermaid_class.clone());

                get_relationships_from_class(mermaid_class).for_each(|r| {
                    relationships.insert(r);
                });
            }
            Item::Trait(_) => todo!(),
            Item::TraitAlias(_) => todo!(),
            Item::Type(_) => todo!(),
            Item::Union(_) => todo!(),
            Item::Use(_) => todo!(),
            Item::Verbatim(_) => todo!(),
            _ => todo!(),
        });

    MermaidResult {
        classes,
        enums,
        relationships,
    }
}

fn get_relationships_from_class(class: MermaidClass) -> IntoIter<Relationship> {
    let mut relationships = HashSet::new();
    class.fields.iter().for_each(|field| {
        let to = match &field.ty.generics {
            Some(generics) => generics.first().unwrap(),
            None => &field.ty.identifier,
        };

        if !is_primitive(to) && field.ty.reference && field.name.is_some() {
            relationships.insert(Relationship::Association(class.name.clone(), to.clone()));
        } else if !is_primitive(to) && field.name.is_some() {
            relationships.insert(Relationship::Composition(class.name.clone(), to.clone()));
        }
    });

    // if let Fields::Named(named_fields) = fields {
    //     for field in named_fields.named {
    //         match field.ty {
    //             Type::Path(type_path) => {
    //                 if is_primitive(&type_path.path) {
    //                     continue;
    //                 }
    //                 let composited_class_name = type_path
    //                     .path
    //                     .get_ident()
    //                     .map(|i| i.to_string())
    //                     .unwrap_or_default();

    //                 relationships.insert(Relationship::Composition(
    //                     class_name.clone(),
    //                     composited_class_name,
    //                 ));
    //             }
    //             Type::Reference(reference) => {
    //                 if let Type::Path(type_path) = *reference.elem {
    //                     let associated_class_name = type_path
    //                         .path
    //                         .get_ident()
    //                         .map(|i| i.to_string())
    //                         .unwrap_or_default();

    //                     relationships.insert(Relationship::Association(
    //                         class_name.clone(),
    //                         associated_class_name,
    //                     ));
    //                 }
    //             }
    //             Type::Array(_) => todo!(),
    //             Type::BareFn(_) => todo!(),
    //             Type::Group(_) => todo!(),
    //             Type::ImplTrait(_) => todo!(),
    //             Type::Infer(_) => todo!(),
    //             Type::Macro(_) => todo!(),
    //             Type::Never(_) => todo!(),
    //             Type::Paren(_) => todo!(),
    //             Type::Ptr(_) => todo!(),
    //             Type::Slice(_) => todo!(),
    //             Type::TraitObject(_) => todo!(),
    //             Type::Tuple(_) => todo!(),
    //             Type::Verbatim(_) => todo!(),
    //             _ => todo!(),
    //         }
    // }
    // };
    relationships.into_iter()
}

fn is_primitive(ty: &str) -> bool {
    let primitive_identifiers = [
        "String", "&str", "i32", "u32", "i64", "u64", "i16", "u16", "i8", "u8", "usize", "isize",
    ]; // TODO can this be dynamic?
    primitive_identifiers.contains(&ty)
}
