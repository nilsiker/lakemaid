use super::MermaidType;

impl From<syn::Type> for super::MermaidType {
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

impl From<syn::ItemStruct> for super::MermaidClass {
    fn from(value: syn::ItemStruct) -> Self {
        Self {
            name: value.ident.to_string(),
            fields: value.fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl From<syn::Field> for super::MermaidField {
    fn from(value: syn::Field) -> Self {
        let name = value.ident.map(|i| i.to_string());
        Self {
            visibility: value.vis.into(),
            name,
            ty: value.ty.into(),
        }
    }
}

impl From<syn::Visibility> for super::Visibility {
    fn from(value: syn::Visibility) -> Self {
        match value {
            syn::Visibility::Public(_) => super::Visibility::Public,
            _ => super::Visibility::Private,
        }
    }
}

impl From<syn::ItemEnum> for super::MermaidEnum {
    fn from(value: syn::ItemEnum) -> Self {
        let name = value.ident.to_string();
        let variants = value
            .variants
            .into_iter()
            .map(|v| v.ident.to_string())
            .collect();
        super::MermaidEnum { name, variants }
    }
}

fn parse_generics(ty: &syn::Type) -> Option<Vec<MermaidType>> {
    match ty {
        syn::Type::Path(path) => {
            if let Some(segment) = path.path.segments.last() {
                let mut generics = Vec::new();

                if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                    for arg in &angle_args.args {
                        if let syn::GenericArgument::Type(inner_type) = arg {
                            match inner_type {
                                syn::Type::Path(path) => {
                                    let identifier = path
                                        .path
                                        .segments
                                        .last()
                                        .unwrap()
                                        .ident
                                        .to_string();
                                    // path.path.get_ident().unwrap_or(&default_ident).to_string();
                                    let inner_mermaid_type = parse_generics(inner_type);
                                    generics.push(MermaidType {
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
    use crate::mappings::{MermaidField, MermaidType};

    mod class_fields {
        use super::*;

        #[test]
        fn private_u32_field() {
            let field = MermaidField {
                name: Some("field_name".to_string()),
                ty: MermaidType {
                    reference: false,
                    identifier: "u32".to_string(),
                    generics: None,
                },
                visibility: crate::mappings::Visibility::Private,
            };

            let string: String = field.into();
            assert_eq!(
                string, "- field_name: u32",
                "testing field 'field_name: u32'"
            );
        }

        #[test]
        fn public_u32_field() {
            let field = MermaidField {
                name: Some("field_name".to_string()),
                ty: MermaidType {
                    generics: None,
                    reference: false,
                    identifier: "u32".to_string(),
                },
                visibility: crate::mappings::Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: u32",
                "testing field 'pub field_name: u32'"
            );
        }

        #[test]
        fn private_vec_u32_field() {
            let field = MermaidField {
                name: Some("field_name".to_string()),
                ty: MermaidType {
                    generics: Some(vec![MermaidType {
                        generics: None,
                        identifier: "u32".to_string(),
                        reference: false,
                    }]),
                    identifier: "Vec".to_string(),
                    reference: false,
                },
                visibility: crate::mappings::Visibility::Private,
            };

            let string: String = field.into();
            assert_eq!(
                string, "- field_name: Vec~u32~",
                "testing field 'field_name: Vec<u32>'"
            );
        }

        #[test]
        fn public_vec_u32_field() {
            let field = MermaidField {
                name: Some("field_name".to_string()),
                ty: MermaidType {
                    generics: Some(vec![MermaidType {
                        generics: None,
                        identifier: "u32".to_string(),
                        reference: false,
                    }]),
                    identifier: "Vec".to_string(),
                    reference: false,
                },
                visibility: crate::mappings::Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: Vec~u32~",
                "testing field 'pub field_name: Vec<u32>'"
            );
        }

        #[test]
        fn private_option_vec_u32_field() {
            let field = MermaidField {
                name: Some("field_name".to_string()),
                ty: MermaidType {
                    reference: false,
                    identifier: "Option".to_string(),
                    generics: Some(vec![MermaidType {
                        reference: false,
                        identifier: "Vec".to_string(),
                        generics: Some(vec![MermaidType {
                            reference: false,
                            identifier: "u32".to_string(),
                            generics: None,
                        }]),
                    }]),
                },
                visibility: crate::mappings::Visibility::Public,
            };

            let string: String = field.into();
            assert_eq!(
                string, "+ field_name: Option~Vec~u32~~",
                "testing field 'pub field_name: Option≲Vec<u32>>'"
            );
        }
    }
}
