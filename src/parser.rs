use syn::{parse_file, Item, ItemFn, ItemStruct, ItemTrait, ItemImpl};
use syn::{Fields, FieldsNamed, Type, Pat, FnArg, ReturnType, Attribute};
use std::fs;
use std::path::Path;
use quote::ToTokens;

#[derive(Debug)]
pub struct NoirFile {
    pub name: String,
    pub structs: Vec<NoirStruct>,
    pub traits: Vec<NoirTrait>,
    pub functions: Vec<NoirFunction>,
    pub impls: Vec<NoirImpl>,
}

#[derive(Debug)]
pub struct NoirStruct {
    pub name: String,
    pub fields: Vec<NoirField>,
}

#[derive(Debug)]
pub struct NoirField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct NoirTrait {
    pub name: String,
    pub methods: Vec<NoirFunction>,
}

#[derive(Debug)]
pub struct NoirFunction {
    pub name: String,
    pub params: Vec<NoirParam>,
    pub return_type: Option<String>,
    pub doc_comment: Option<String>,
    pub attributes: Vec<String>,
    pub generic_params: Vec<String>,
    pub is_unconstrained: bool,
}

#[derive(Debug)]
pub struct NoirParam {
    pub name: String,
    pub ty: String,
}

#[derive(Debug)]
pub struct NoirImpl {
    pub target: String,
    pub methods: Vec<NoirFunction>,
}

pub fn parse_noir_file(file_path: &str) -> Result<NoirFile, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let ast = parse_file(&content)?;

    let file_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap().to_string();

    let mut noir_file = NoirFile {
        name: file_name,
        structs: Vec::new(),
        traits: Vec::new(),
        functions: Vec::new(),
        impls: Vec::new(),
    };

    for item in ast.items {
        match item {
            Item::Struct(s) => noir_file.structs.push(parse_struct(s)),
            Item::Trait(t) => noir_file.traits.push(parse_trait(t)),
            Item::Fn(f) => noir_file.functions.push(parse_function(f)),
            Item::Impl(i) => noir_file.impls.push(parse_impl(i)),
            _ => {}
        }
    }

    Ok(noir_file)
}

fn parse_struct(s: ItemStruct) -> NoirStruct {
    let name = s.ident.to_string();
    let fields = match s.fields {
        Fields::Named(FieldsNamed { named, .. }) => named
            .into_iter()
            .map(|f| NoirField {
                name: f.ident.unwrap().to_string(),
                ty: type_to_string(&f.ty),
            })
            .collect(),
        _ => Vec::new(), // Handle unnamed fields if needed
    };
    NoirStruct { name, fields }
}

fn parse_trait(t: ItemTrait) -> NoirTrait {
    let name = t.ident.to_string();
    let methods = t.items
        .into_iter()
        .filter_map(|item| {
            if let syn::TraitItem::Method(method) = item {
                Some(parse_trait_method(method))
            } else {
                None
            }
        })
        .collect();
    NoirTrait { name, methods }
}

fn parse_function(f: ItemFn) -> NoirFunction {
    let name = f.sig.ident.to_string();
    let params = f.sig.inputs.iter().filter_map(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            Some(NoirParam {
                name: pat_to_string(&pat_type.pat),
                ty: type_to_string(&pat_type.ty),
            })
        } else {
            None
        }
    }).collect();

    let return_type = match f.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ref ty) => Some(type_to_string(ty)),
    };

    let doc_comment = extract_doc_comment(&f.attrs);
    let attributes = extract_attributes(&f.attrs);
    let generic_params = f.sig.generics.params.iter().map(|param| param.to_token_stream().to_string()).collect();
    let is_unconstrained = f.sig.constness.is_some() || attributes.iter().any(|attr| attr.contains("unconstrained"));

    NoirFunction {
        name,
        params,
        return_type,
        doc_comment,
        attributes,
        generic_params,
        is_unconstrained,
    }
}

fn parse_impl(i: ItemImpl) -> NoirImpl {
    let target = type_to_string(&i.self_ty);
    let methods = i.items
        .into_iter()
        .filter_map(|item| {
            if let syn::ImplItem::Method(method) = item {
                Some(parse_impl_method(method))
            } else {
                None
            }
        })
        .collect();
    NoirImpl { target, methods }
}

fn parse_trait_method(method: syn::TraitItemMethod) -> NoirFunction {
    let name = method.sig.ident.to_string();
    let params = method.sig.inputs
        .into_iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some(NoirParam {
                    name: pat_to_string(&pat_type.pat),
                    ty: type_to_string(&pat_type.ty),
                })
            } else {
                None
            }
        })
        .collect();
    let return_type = match method.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(type_to_string(&ty)),
    };
    let doc_comment = extract_doc_comment(&method.attrs);
    let attributes = extract_attributes(&method.attrs);
    let generic_params = method.sig.generics.params.iter().map(|param| param.to_token_stream().to_string()).collect();
    let is_unconstrained = method.sig.constness.is_some() || attributes.iter().any(|attr| attr.contains("unconstrained"));

    NoirFunction {
        name,
        params,
        return_type,
        doc_comment,
        attributes,
        generic_params,
        is_unconstrained,
    }
}

fn parse_impl_method(method: syn::ImplItemMethod) -> NoirFunction {
    let name = method.sig.ident.to_string();
    let params = method.sig.inputs
        .into_iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                Some(NoirParam {
                    name: pat_to_string(&pat_type.pat),
                    ty: type_to_string(&pat_type.ty),
                })
            } else {
                None
            }
        })
        .collect();
    let return_type = match method.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => Some(type_to_string(&ty)),
    };
    let doc_comment = extract_doc_comment(&method.attrs);
    let attributes = extract_attributes(&method.attrs);
    let generic_params = method.sig.generics.params.iter().map(|param| param.to_token_stream().to_string()).collect();
    let is_unconstrained = method.sig.constness.is_some() || attributes.iter().any(|attr| attr.contains("unconstrained"));

    NoirFunction {
        name,
        params,
        return_type,
        doc_comment,
        attributes,
        generic_params,
        is_unconstrained,
    }
}

fn type_to_string(ty: &Type) -> String {
    ty.to_token_stream().to_string()
}

fn pat_to_string(pat: &Pat) -> String {
    pat.to_token_stream().to_string()
}

fn extract_doc_comment(attrs: &[Attribute]) -> Option<String> {
    attrs.iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .filter_map(|attr| attr.parse_meta().ok())
        .filter_map(|meta| {
            if let syn::Meta::NameValue(nv) = meta {
                if let syn::Lit::Str(lit) = nv.lit {
                    Some(lit.value().trim().to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
        .into()
}

fn extract_attributes(attrs: &[Attribute]) -> Vec<String> {
    attrs.iter()
        .map(|attr| attr.to_token_stream().to_string())
        .collect()
}