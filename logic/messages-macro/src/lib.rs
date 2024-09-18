extern crate proc_macro;

use binary_encoding::encode_message_tag;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitInt};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};

// Collection of all used message tags which ensures that there are no two messages with the same id
lazy_static! {
    static ref CLIENT_MESSAGE_TAGS: Mutex<HashMap<u16, String>> = Mutex::new(HashMap::new());
    static ref SERVER_MESSAGE_TAGS: Mutex<HashMap<u16, String>> = Mutex::new(HashMap::new());
}

/// Mutex may become poisoned during active development, probably because rust-analyzer
/// multi-threaded processing. This wrapper ignores the error as often it's not an issue
fn lock_mutex<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// Procedural macros that creates a custom serialization logic for the given client message
#[proc_macro_attribute]
pub fn client_message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_tag_lit = parse_macro_input!(attr as LitInt);
    let message_tag = message_tag_lit
        .base10_parse::<u16>()
        .expect("Message tag has to be a u16 number");
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name_ident = &input.ident;
    let struct_name = struct_name_ident.to_string();
    let mut message_tags = lock_mutex(&CLIENT_MESSAGE_TAGS);

    // Ensure that no two messages share the same tag
    if let Some(existing_name) = message_tags.get(&message_tag) {
        if existing_name != &struct_name {
            panic!(
                "Duplicate client message tag={} for struct {} and struct {}",
                message_tag, struct_name, existing_name
            );
        }
    } else {
        message_tags.insert(message_tag, struct_name.clone());
    }

    // We fully control JSON creation process and don't need to support any other clients,
    // so we can avoid the overhead of full JSON parsing. Instead, we define a fixed prefix and suffix
    // for the JSON structure and use simple string concatenation and substring operations as a shortcut
    let encoded_tag = encode_message_tag(message_tag);
    let json_prefix = format!(r#"{{"k":"{}","v":""#, encoded_tag);
    let json_suffix = r#""}"#;

    let message_serializer = syn::Ident::new(
        &format!("{}Serializer", struct_name_ident),
        struct_name_ident.span(),
    );

    let expanded = quote! {
        #[derive(std::cmp::PartialEq, std::fmt::Debug, bincode::Decode, bincode::Encode, uniffi::Record, Clone)]
        #input

        // Because of limitation of uniffi we can't add methods to uniffi::Record, so we create a second
        // struct will be responsible for data encoding
        #[cfg(feature="uniffi")]
        #[derive(uniffi::Object)]
        struct #message_serializer {
            data: #struct_name_ident
        }


        #[cfg(feature="uniffi")]
        #[uniffi::export]
        impl #message_serializer {
            #[uniffi::constructor]
            pub fn new(data: #struct_name_ident) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {data})
            }

            pub fn data(&self) -> #struct_name_ident {
                self.data.clone()
            }

            pub fn serialize(&self) -> Result<String, SerializationError> {
                let data = bincode::encode_to_vec(&self.data, bincode::config::standard())?;
                let mut output = #json_prefix.to_string();
                output.push_str(&binary_encoding::encode_base94(&data));
                output.push_str(#json_suffix);
                Ok(output)
            }

            #[uniffi::constructor]
            pub fn deserialize(data: String) -> Result<std::sync::Arc<Self>, SerializationError> {
                if data.starts_with(#json_prefix) && data.ends_with(#json_suffix) {
                    let base64_data = &data[#json_prefix.len()..data.len() - #json_suffix.len()];
                    let decoded_data = binary_encoding::decode_base94(base64_data)?;
                    let instance: #struct_name_ident = bincode::decode_from_slice(&decoded_data, bincode::config::standard())?.0;
                    Ok(std::sync::Arc::new(Self {data: instance}))
                } else {
                    Err(SerializationError::BadData { msg: "No json_prefix and json_suffix found".to_string() })
                }
            }

            // For client development it's handy to have a quick way to dump message to log
            pub fn debug_string(&self) -> String {
                format!("{:?}", self.data)
            }
        }
    };
    TokenStream::from(expanded)
}

// Procedural macros that creates a custom serialization logic for the given server message
#[proc_macro_attribute]
pub fn server_message(attr: TokenStream, item: TokenStream) -> TokenStream {
    let message_tag_lit = parse_macro_input!(attr as LitInt);
    let message_tag = message_tag_lit
        .base10_parse::<u16>()
        .expect("Message tag has to be a u16 number");
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name_ident = &input.ident;
    let struct_name = struct_name_ident.to_string();
    let mut message_tags = lock_mutex(&SERVER_MESSAGE_TAGS);

    // Same logic for ensuring no server message tag got duplicated, although tags for
    // client messages and server messages may repeat as they are independent
    if let Some(existing_name) = message_tags.get(&message_tag) {
        if existing_name != &struct_name {
            panic!(
                "Duplicate server message tag={} for struct {} and struct {}",
                message_tag, struct_name, existing_name
            );
        }
    } else {
        message_tags.insert(message_tag, struct_name.clone());
    }

    let struct_message_tag_fn = syn::Ident::new(
        &format!("{}_message_tag", struct_name_ident),
        struct_name_ident.span(),
    );

    let message_serializer = syn::Ident::new(
        &format!("{}Serializer", struct_name_ident),
        struct_name_ident.span(),
    );

    let message_tag = binary_encoding::encode_message_tag(message_tag);

    let expanded = quote! {
        #[derive(std::cmp::PartialEq, std::fmt::Debug, bincode::Decode, bincode::Encode, uniffi::Record, Clone)]
        #input

        // For server we can have serializing logic on a struct itself, it makes things easier to work with
        #[cfg(feature = "server")]
        impl #struct_name_ident {
            /// Serialize message to Base94 encoded string and add 2 bytes message tag at the beginning
            pub fn serialize(&self) -> Result<String, SerializationError> {
                let data = bincode::encode_to_vec(&self, bincode::config::standard())?;
                let serialized = binary_encoding::encode_base94(&data);
                Ok(format!("{}{}", #message_tag, serialized))
            }
            /// Deserialize Base94 encoded string with 2 bytes message tag at the beginning back to message
            pub fn deserialize(input: &str) -> Result<Self, SerializationError> {
                if !input.starts_with(#message_tag) {
                    return Err(SerializationError::BadData { msg: "Bad message tag".to_string() })
                }
                let input = &input[#message_tag.len()..];
                let decoded = binary_encoding::decode_base94(input)?;
                let instance: Self = bincode::decode_from_slice(&decoded, bincode::config::standard())?.0;
                Ok(instance)
            }
        }

        // Because of limitation of uniffi we can't add methods to uniffi::Record, so we create a second
        // struct will be responsible for data encoding
        #[cfg(feature="uniffi")]
        #[derive(uniffi::Object)]
        struct #message_serializer {
            data: #struct_name_ident
        }

        // To avoid conflict with duplicated "serialize" function enable it only when server is turned off
        // otherwise `cargo build --all-features` fails
        #[cfg(all(feature = "uniffi", not(feature = "server")))]
        #[uniffi::export]
        impl #message_serializer {
            #[uniffi::constructor]
            pub fn new(data: #struct_name_ident) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {data})
            }

            pub fn data(&self) -> #struct_name_ident {
                self.data.clone()
            }

            pub fn serialize(&self) -> Result<String, SerializationError> {
                let data = bincode::encode_to_vec(&self.data, bincode::config::standard())?;
                let serialized = binary_encoding::encode_base94(&data);
                Ok(format!("{}{}", #message_tag, serialized))
            }

            #[uniffi::constructor]
            pub fn deserialize(input: String) -> Result<std::sync::Arc<Self>, SerializationError> {
                if !input.starts_with(#message_tag) {
                    return Err(SerializationError::BadData { msg: "Bad message tag".to_string() })
                }
                let input = &input[#message_tag.len()..];
                let decoded = binary_encoding::decode_base94(input)?;
                let instance: #struct_name_ident = bincode::decode_from_slice(&decoded, bincode::config::standard())?.0;
                Ok(std::sync::Arc::new(Self {data: instance}))
            }

            // For client development it's handy to have a quick way to dump message to log
            pub fn debug_string(&self) -> String {
                format!("{:?}", self.data)
            }
        }

        #[cfg(feature="uniffi")]
        #[uniffi::export]
        #[allow(non_snake_case)]
        fn #struct_message_tag_fn() -> String {
            #message_tag.to_string()
        }
    };
    TokenStream::from(expanded)
}

/// Returns current maximum registered client message type. Useful during development when
/// you don't want to search for the whole project to figure out which message type was last
#[proc_macro]
pub fn max_client_message_type(_: TokenStream) -> TokenStream {
    let max_id = {
        let ids = CLIENT_MESSAGE_TAGS.lock().unwrap();
        ids.keys().max().copied()
    };
    let output = match max_id {
        Some(id) => quote! { #id },
        None => quote! { 0 },
    };
    output.into()
}

/// Returns current maximum registered server message type
#[proc_macro]
pub fn max_server_message_type(_: TokenStream) -> TokenStream {
    let max_id = {
        let ids = SERVER_MESSAGE_TAGS.lock().unwrap();
        ids.keys().max().copied()
    };
    let output = match max_id {
        Some(id) => quote! { #id },
        None => quote! { 0 },
    };
    output.into()
}
