use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Expr, ExprArray};

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

#[proc_macro]
pub fn generate_push_binds(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExprArray);

    let query_builder = &input.elems[0]; // query_builder
    let domain = &input.elems[1]; // domain

    let fields: Vec<proc_macro2::Ident> = input
        .elems
        .iter()
        .skip(2)
        .filter_map(|elem| {
            if let Expr::Path(path) = elem {
                path.path.get_ident().cloned()
            } else {
                None
            }
        })
        .collect();

    let generated = quote! {
        #(
            #query_builder.push_bind(#domain.#fields);
        )*
    };

    TokenStream::from(generated)
}

// #[macro_export]
// macro_rules! generate_push_binds {
//     ($query_builder:expr, $domain:expr, [$($field:ident),*]) => {
//         $(
//             $query_builder.push_bind($domain.$field);
//         )*
//     };
// }
