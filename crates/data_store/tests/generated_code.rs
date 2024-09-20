use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

#[test]
fn test_generate_push_binds_code() {
    let field_names = vec!["username", "email", "created_at"];
    let generated_code = generate_push_binds_code(field_names);
    println!("{}", generated_code);
}

fn generate_push_binds_code(field_names: Vec<&str>) -> TokenStream {
    let fields: Vec<Ident> = field_names
        .into_iter()
        .map(|name| Ident::new(name, proc_macro2::Span::call_site()))
        .collect();

    // 使用 quote! 生成代码
    let bindings = fields.into_iter().map(|field| {
        quote! {
            b.push_bind($user.#field);
        }
    });

    let bindings_stream: TokenStream = bindings.collect();

    // 使用 quote! 宏构建最终的代码
    let generated_code = quote! {
        $(
            #bindings_stream
        )*
    };

    generated_code
}
