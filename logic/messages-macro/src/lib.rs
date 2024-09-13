extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, LitStr};

// Procedural macros that creates a custom serialization logic for the given message
// struct which automatically implement `Message` trait
#[proc_macro_attribute]
pub fn message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_type = parse_macro_input!(attr as LitStr);
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("command macro only supports structs"),
    };

    let serialize_fields = fields.iter().map(|field| {
        let name = &field.ident;
        quote! {
            state.serialize_field(stringify!(#name), &self.#name)?;
        }
    });

    let fields_len = fields.len();

    let expanded = quote! {
        #[derive(serde::Deserialize, std::cmp::PartialEq, std::fmt::Debug)]
        #input

        impl crate::messages::Message for #struct_name {
            fn message_type() -> &'static str {
                #message_type
            }

            fn serialize(&self) -> Result<Vec<u8>, crate::messages::SerializationError> {
                use serde::ser::{Serialize, SerializeStruct, Serializer};
                let mut data = Vec::new();
                let mut serializer = serde_json::Serializer::new(&mut data);
                let mut state = serializer.serialize_struct(stringify!(#struct_name), #fields_len)?;
                state.serialize_field("type", #message_type)?;
                #(#serialize_fields)*
                state.end()?;
                Ok(data)
            }

            fn deserialize(data: &[u8]) -> Result<Self, crate::messages::SerializationError> {
                 let parsed = serde_json::from_slice(data)?;
                 Ok(parsed)
            }
        }
    };

    TokenStream::from(expanded)
}
