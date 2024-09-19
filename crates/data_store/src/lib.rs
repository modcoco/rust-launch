use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GetFieldNames)]
pub fn get_field_names(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(fields_named),
        ..
    }) = input.data
    {
        fields_named.named
    } else {
        return TokenStream::new(); // 只处理命名字段或结构体
    };

    let field_names = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            stringify!(#name)
        }
    });

    let expanded = quote! {
        impl #struct_name {
            pub fn field_names() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }
        }
    };

    TokenStream::from(expanded)
}
