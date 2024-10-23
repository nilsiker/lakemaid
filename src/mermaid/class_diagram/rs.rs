use std::collections::{hash_set::IntoIter, HashSet};

use syn::Item;

use super::{nodes::{Class, Enum, Field, Type, Visibility}, ClassDiagram, Relationship};

impl From<syn::Type> for Type {
    fn from(value: syn::Type) -> Self {
        match &value {
            syn::Type::Array(_) => todo!(),
            syn::Type::BareFn(_) => todo!(),
            syn::Type::Group(_) => todo!(),
            syn::Type::ImplTrait(_) => todo!(),
            syn::Type::Infer(_) => todo!(),
            syn::Type::Macro(_) => todo!(),
            syn::Type::Never(_) => todo!(),
            syn::Type::Paren(_) => todo!(),
            syn::Type::Path(path) => {
                if let Some(segment) = path.path.segments.last() {
                    let identifier = segment.ident.to_string();
                    let generics = parse_generics(&value);
                    return Self {
                        reference: false,
                        identifier,
                        generics: generics,
                    };
                }
            }
            syn::Type::Ptr(_) => todo!(),
            syn::Type::Reference(reference) => {
                if let syn::Type::Path(type_path) = *reference.clone().elem {
                    let association_class = type_path
                        .path
                        .get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_default();
                    return Self {
                        reference: true,
                        identifier: association_class,
                        generics: None,
                    };
                }
            }
            syn::Type::Slice(_) => todo!(),
            syn::Type::TraitObject(_) => todo!(),
            syn::Type::Tuple(_) => todo!(),
            syn::Type::Verbatim(_) => todo!(),
            _ => todo!(),
        }
        Self {
            reference: false,
            identifier: "".to_string(),
            generics: None,
        }
    }
}

impl From<syn::ItemStruct> for Class {
    fn from(value: syn::ItemStruct) -> Self {
        Self {
            name: value.ident.to_string(),
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl From<syn::Field> for Field {
    fn from(value: syn::Field) -> Self {
        let name = value.ident.map(|i| i.to_string());
        Self {
            visibility: value.vis.into(),
            name,
            ty: value.ty.into(),
        }
    }
}

impl From<syn::Visibility> for Visibility {
    fn from(value: syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => Visibility::Public,
            _ => Visibility::Private,
        }
    }
}

impl From<syn::ItemEnum> for Enum {
    fn from(value: syn::ItemEnum) -> Self {
        let name = value.ident.to_string();
        let variants = value
            .variants
            .into_iter()
            .map(|v| v.ident.to_string())
            .collect();
        Enum { name, variants }
    }
}

pub fn parse(src: &str) -> ClassDiagram {
    let mut classes = Vec::new();
    let mut enums = Vec::new();
    let mut relationships = HashSet::new();
    syn::parse_file(src)
        .expect("can parse file")
        .items
        .into_iter()
        .for_each(|item| match item {
            Item::Enum(e) => enums.push(e.into()),
            Item::Impl(_) => (),
            Item::Mod(_) => (),
            Item::Struct(item_struct) => {
                let mermaid_class = Class::from(item_struct.clone());
                classes.push(mermaid_class.clone());

                get_relationships_from_class(mermaid_class).for_each(|r| {
                    relationships.insert(r);
                });
            }
            Item::Trait(_) => todo!(),
            _ => (),
        });

    ClassDiagram {
        classes,
        enums,
        relationships,
    }
}

fn get_identifier_from_generic_type(ty: &Type) -> String {
    match &ty.generics {
        Some(generics) => get_identifier_from_generic_type(generics.first().unwrap()),
        None => ty.identifier.clone(),
    }
}

fn get_relationships_from_class(
    class: Class,
) -> IntoIter<Relationship> {
    let mut relationships = HashSet::new();
    class.fields.iter().for_each(|field| {
        let to = match &field.ty.generics {
            Some(_) => get_identifier_from_generic_type(&field.ty),
            None => field.ty.identifier.clone(),
        };

        if !is_primitive(&to) && field.ty.reference && field.name.is_some() {
            relationships.insert(Relationship::Association(
                class.name.clone(),
                to.clone(),
            ));
        } else if !is_primitive(&to) && field.name.is_some() {
            relationships.insert(Relationship::Composition(
                class.name.clone(),
                to.clone(),
            ));
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
        "String", "&str", "f32", "f64", "i32", "u32", "i64", "u64", "i16", "u16", "i8", "u8",
        "usize", "isize", "bool",
    ]; // TODO can this be dynamic?
    primitive_identifiers.contains(&ty)
}

fn parse_generics(ty: &syn::Type) -> Option<Vec<Type>> {
    match ty {
        syn::Type::Path(path) => {
            if let Some(segment) = path.path.segments.last() {
                let mut generics = Vec::new();

                if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                    for arg in &angle_args.args {
                        if let syn::GenericArgument::Type(inner_type) = arg {
                            match inner_type {
                                syn::Type::Path(path) => {
                                    let identifier =
                                        path.path.segments.last().unwrap().ident.to_string();
                                    // path.path.get_ident().unwrap_or(&default_ident).to_string();
                                    let inner_mermaid_type = parse_generics(inner_type);
                                    generics.push(Type {
                                        reference: false, // You may need to adjust this based on your requirements
                                        identifier,
                                        generics: inner_mermaid_type,
                                    });
                                }
                                syn::Type::Reference(_) => todo!(),
                                _ => return None,
                            }
                        }
                    }
                }
                return if generics.is_empty() {
                    None
                } else {
                    Some(generics)
                };
            }
            return None;
        }
        // Handle other types if necessary
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod class_fields {
        use super::*;

        #[test]
        fn private_u32_field() {
            let field = Field {
                name: Some("field_name".to_string()),
                ty: Type {
                    reference: false,
                    identifier: "u32".to_string(),
                    generics: None,
                },
                visibility: Visibility::Private,
            };

            let string: String = field.into();
            assert_eq!(
                string, "- field_name: u32",
                "testing field 'field_name: u32'"
            );
        }

        #[test]
        fn public_u32_field() {
            let field = Field {
                name: Some("field_name".to_string()),
                ty: Type {
                    generics: None,
                    reference: false,
                    identifier: "u32".to_string(),
                },
                visibility: Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: u32",
                "testing field 'pub field_name: u32'"
            );
        }

        #[test]
        fn private_vec_u32_field() {
            let field = Field {
                name: Some("field_name".to_string()),
                ty: Type {
                    generics: Some(vec![Type {
                        generics: None,
                        identifier: "u32".to_string(),
                        reference: false,
                    }]),
                    identifier: "Vec".to_string(),
                    reference: false,
                },
                visibility: Visibility::Private,
            };

            let string: String = field.into();
            assert_eq!(
                string, "- field_name: Vec~u32~",
                "testing field 'field_name: Vec<u32>'"
            );
        }

        #[test]
        fn public_vec_u32_field() {
            let field = Field {
                name: Some("field_name".to_string()),
                ty: Type {
                    generics: Some(vec![Type {
                        generics: None,
                        identifier: "u32".to_string(),
                        reference: false,
                    }]),
                    identifier: "Vec".to_string(),
                    reference: false,
                },
                visibility: Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: Vec~u32~",
                "testing field 'pub field_name: Vec<u32>'"
            );
        }

        #[test]
        fn private_option_vec_u32_field() {
            let field = Field {
                name: Some("field_name".to_string()),
                ty: Type {
                    reference: false,
                    identifier: "Option".to_string(),
                    generics: Some(vec![Type {
                        reference: false,
                        identifier: "Vec".to_string(),
                        generics: Some(vec![Type {
                            reference: false,
                            identifier: "u32".to_string(),
                            generics: None,
                        }]),
                    }]),
                },
                visibility: Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: Option~Vec~u32~~",
                "testing field 'pub field_name: Option≲Vec<u32>>'"
            );
        }
    }
}
