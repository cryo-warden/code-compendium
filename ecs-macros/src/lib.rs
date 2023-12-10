use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro]
pub fn remove_duplicate_struct_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let vis = input.vis;
    let attrs = input.attrs;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Expected a named struct"),
    };

    let mut field_names = std::collections::HashSet::new();
    let filtered_fields = fields.into_iter().filter(|field| {
        let field_name = field.ident.as_ref().unwrap();
        field_names.insert(field_name.clone())
    });

    let output = quote! {
      #(#attrs)*
      #vis struct #struct_name {
        #(#filtered_fields,)*
      }
    };

    output.into()
}

#[proc_macro]
pub fn remove_duplicate_struct_initializer_fields(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::ExprStruct);

    let struct_name = input.path;
    let fields = input.fields;

    let mut field_names = std::collections::HashSet::new();
    let filtered_fields: Vec<_> = fields
        .into_iter()
        .filter(|field| field_names.insert(field.member.clone()))
        .collect();

    let output = quote! {
      #struct_name {
        #(#filtered_fields,)*
      }
    };

    output.into()
}
