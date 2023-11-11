impl From<syn::Type> for super::MermaidType {
    fn from(value: syn::Type) -> Self {
        match value {
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
                    let mut generics = None;
                    if let syn::PathArguments::AngleBracketed(angle_args) = &segment.arguments {
                        if let syn::GenericArgument::Type(syn::Type::Path(generic_type_path)) =
                            angle_args.args.first().unwrap()
                        {
                            let generic_type = generic_type_path.path.get_ident().unwrap();
                            generics = Some(vec![generic_type.to_string()]); // TODO support more generics
                        }
                    }
                    return Self {
                        reference: false,
                        identifier,
                        generics,
                    };
                }
            }
            syn::Type::Ptr(_) => todo!(),
            syn::Type::Reference(reference) => {
                if let syn::Type::Path(type_path) = *reference.elem {
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
