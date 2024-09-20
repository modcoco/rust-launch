use chrono::NaiveDateTime;
use data_store::GetFieldNames;
use quote::quote;
use serde::{Deserialize, Serialize};
use syn::{DeriveInput, Fields};

#[derive(Debug, Default, Serialize, Deserialize, GetFieldNames)]
pub struct User {
    pub username: String,
    pub created_at: NaiveDateTime,
    pub email: String,
    pub id: i32,
}

fn get_user_struct() -> DeriveInput {
    let expanded = quote! {
        #[derive(Debug, Default, Serialize, Deserialize)]
        pub struct User {
            pub id: i32,
            pub username: String,
            pub email: String,
            pub created_at: NaiveDateTime,
        }
    };
    syn::parse2(expanded).unwrap()
}

#[test]
fn test_get_data_struct() {
    let input = get_user_struct();

    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = input.data {
        match fields {
            Fields::Named(fields_named) => {
                for field in fields_named.named {
                    if let Some(ident) = field.ident {
                        println!("{}", ident);
                    }
                }
            }
            _ => panic!("Unsupported fields"),
        }
    } else {
        panic!("Not a struct");
    }
}

#[test]
fn test_get_field_names() {
    let field_names = User::field_names();
    println!("{:?}", field_names);
}
