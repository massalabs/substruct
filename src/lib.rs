use proc_macro::{TokenStream};
// use std::mem::uninitialized;
use quote::{
    // format_ident,
    quote};
use syn::{parse_macro_input, DeriveInput, Data, Meta, NestedMeta, Lit, Attribute, Ident};

fn get_parent_attr(attr: &Attribute) -> syn::Result<Ident> {

    // Parse attribute like:
    // #[parent(type = "XXX")]

    // parse attribute
    let meta = attr.parse_meta()?;

    // println!("meta: {:?}", meta);

    let meta_list = match meta {
        Meta::List(list) => list,
        _ => return Err(syn::Error::new_spanned(meta, "Expected a list-style attribute")),
    };

    let nested = match meta_list.nested.len() {
        1 => &meta_list.nested[0],
        _ => {
            return Err(syn::Error::new_spanned(
                meta_list.nested,
                "Currently only a single getter attribute is supported",
            ));
        }
    };

    let name_value = match nested {
        NestedMeta::Meta(Meta::NameValue(nv)) => nv,
        _ => return Err(syn::Error::new_spanned(nested, "Expected `parent = \"<value>\"`")),
    };

    if !name_value.path.is_ident("type") {
        return Err(syn::Error::new_spanned(
            &name_value.path,
            "unsupported getter attribute, expected `type`",
        ));
    }

    match &name_value.lit {
        Lit::Str(s) => {
            // String -> Ident, report an error if parsing fails
            syn::parse_str::<Ident>(&s.value())
                .map_err(|e|
                    syn::Error::new_spanned(s, e)
                )
        }
        lit => Err(syn::Error::new_spanned(lit, "expected string literal")),
    }
}

fn is_doc_attr(attr: &Attribute) -> syn::Result<bool> {

    // Check if attr is a docstring (aka '///')

    // parse attribute
    let meta = attr.parse_meta()?;

    // println!("[is_doc_attr] meta: {:?}", meta);

    let meta_name_value = match meta {
        Meta::NameValue(name_value) => name_value,
        _ => return Err(syn::Error::new_spanned(meta, "Expected a name-value like attribute")),
    };

    let path_segment = match meta_name_value.path.segments.len() {
        1 => &meta_name_value.path.segments[0],
        _ => {
            return Err(syn::Error::new_spanned(
                meta_name_value.path,
                "Expect only 1 segment",
            ));
        }
    };

    let is_doc = path_segment.ident == "doc";
    return Ok(is_doc);

}

#[proc_macro_derive(SubStruct, attributes(parent))]
pub fn derive(input: TokenStream) -> TokenStream {

    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let data_struct = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Enum && Union are not supported here!"),
    };

    let name = input.ident;
    let field_names = data_struct
        .fields
        .iter()
        .map(|field| &field.ident);

    // println!("input attrs len: {}", input.attrs.len());

    let attr = input
        .attrs
        .iter()
        .filter(|a| {
            !is_doc_attr(a).unwrap_or(false)
        })
        .next();

    // let parent_name = get_parent_attr(&input.attrs[1]).unwrap();
    let parent_name = get_parent_attr(attr.expect("Expect one attribute")).unwrap();
    let expanded = quote! {
        impl From<&#parent_name> for #name {
            fn from(value: &#parent_name) -> Self {
                Self {
                    #(
                      #field_names: value.#field_names,
                    )*
                }
            }
        }
    };

    // Debug
    // eprintln!("TOKENS: {}", expanded);
    proc_macro::TokenStream::from(expanded)
}
