use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use serde_json::{json, Value};
use syn::{
    parse_macro_input, DeriveInput, Error, Expr, Generics, ItemFn, Lit, Meta, PathArguments,
    Result, Type,
};

#[proc_macro_attribute]
pub fn tool(args: TokenStream, input: TokenStream) -> TokenStream {
    let registry_name = parse_macro_input!(args as syn::Ident);
    let input = parse_macro_input!(input as ItemFn);

    try_tool(registry_name, input).unwrap_or_else(|err| err.to_compile_error().into())
}

fn try_tool(registry_name: syn::Ident, input: ItemFn) -> Result<TokenStream> {
    let name = &input.sig.ident.to_string();

    let docs = &input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(expr) = &meta.value {
                        if let Lit::Str(lit_str) = &expr.lit {
                            let value = lit_str.value();
                            let trimed_value = value.trim();
                            if !trimed_value.is_empty() {
                                return Some(trimed_value.to_string());
                            }
                        }
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .join("\n");

    let parameters = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let syn::Pat::Ident(ref identity) = *pat_type.pat {
                    let param_name = identity.ident.to_string();
                    let param_type = match extract_type(&pat_type.ty) {
                        Ok(param_type) => param_type,
                        Err(err) => return Some(Err(err)),
                    };
                    let rust_type = pat_type.ty.clone();
                    return Some(Ok((param_name, param_type, rust_type)));
                }
            }
            None
        })
        .collect::<Result<Vec<_>>>()?;

    let param_names: Vec<_> = parameters
        .iter()
        .map(|(name, _, _)| format_ident!("{}", name))
        .collect();
    let rust_types: Vec<_> = parameters.iter().map(|(_, _, ty)| ty.clone()).collect();

    let properties = parameters
        .iter()
        .map(|(name, param_type, _)| (name.to_string(), param_type.clone()))
        .collect::<serde_json::Map<_, _>>();

    let required = param_names
        .iter()
        .map(|name| name.to_string())
        .collect::<Vec<_>>();

    let schema = json!({
        "name": name,
        "description": docs,
        "input_schema": {
            "type": "object",
            "properties": properties,
            "required": required
        }
    });

    let schema_string = serde_json::to_string(&schema).unwrap();

    let fn_name = input.sig.ident.clone();
    let struct_name = format_ident!("{}__AnchorChainTool", fn_name);
    let register_fn_name = format_ident!("register_{}__anchor_chain_tool", fn_name);

    // TODO: Move execute method to a stand alone function
    let expanded = quote! {
        #input

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct #struct_name;

        impl #struct_name {
            pub fn execute(params: Value) -> Value {
                #(
                    let #param_names: #rust_types = serde_json::from_value(params[stringify!(#param_names)].clone()).unwrap();
                )*
                let result = #fn_name(#(#param_names),*);
                serde_json::to_value(result).unwrap()
            }
        }

        #[anchor_chain::ctor::ctor]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        fn #register_fn_name() {
            let schema_value: Value = serde_json::from_str(#schema_string).unwrap();
            #registry_name.blocking_write().register_tool(stringify!(#fn_name), #struct_name::execute, schema_value);
        }
    };

    Ok(expanded.into())
}

fn extract_type(ty: &Type) -> Result<Value> {
    if let Type::Path(type_path) = ty {
        let type_segment = type_path.path.segments.last().unwrap();
        let type_name = type_segment.ident.to_string();
        match type_name.as_str() {
            "String" => Ok(json!({ "type": "string" })),
            "i32" | "i64" | "f32" | "f64" => Ok(json!({ "type": "number" })),
            "bool" => Ok(json!({ "type": "boolean" })),
            _ => {
                // Check if it is a reference to &str
                // TODO: Fix lifetime issue when deserializing &str
                if type_name == "str" {
                    if let PathArguments::AngleBracketed(args) = &type_segment.arguments {
                        if args.args.is_empty() {
                            return Ok(json!({ "type": "string" }));
                        }
                    }
                }
                Err(Error::new_spanned(
                    ty,
                    format!("Unsupported argument type: {}", type_name),
                ))
            }
        }
    } else if let Type::Reference(type_reference) = ty {
        if let Type::Path(type_path) = &*type_reference.elem {
            let type_name = type_path.path.segments.last().unwrap().ident.to_string();
            if type_name == "str" {
                return Ok(json!({ "type": "string" }));
            }
        }
        Err(Error::new_spanned(ty, "Unsupported reference type"))
    } else {
        Err(Error::new_spanned(ty, "Unsupported argument type"))
    }
}

#[proc_macro_derive(Stateless)]
pub fn stateless_node_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_stateless_node(&ast)
}

fn impl_stateless_node(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generics = get_generics(&ast.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let crate_path = if let Ok(crate_name) = crate_name("anchor-chain") {
        match crate_name {
            FoundCrate::Itself => quote!(crate),
            FoundCrate::Name(ref name) => {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                quote!(#ident)
            }
        }
    } else {
        quote!(anchor_chain)
    };

    let gen = quote! {
        impl #impl_generics #crate_path::node::Stateless for #name #ty_generics #where_clause {}
    };
    gen.into()
}

fn get_generics(generics: &Generics) -> Generics {
    let mut generics = generics.clone();
    for param in &mut generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param.bounds.push(syn::parse_quote!(Send));
            type_param.bounds.push(syn::parse_quote!(Sync));
        }
    }
    generics
}
