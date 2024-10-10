extern crate proc_macro;

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

/// Procedural macros that creates a custom serialization logic for the given public client message
#[proc_macro_attribute]
pub fn client_public_message(attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let message_tag_encoded = binary_encoding::encode_message_tag(message_tag);

    let expanded = quote! {
        #[doc = concat!("Message tag = ", #message_tag_encoded)]
        #[derive(std::cmp::PartialEq, std::fmt::Debug, bincode::Decode, bincode::Encode, uniffi::Object, Clone)]
        #input

        #[cfg(feature = "server")]
        impl crate::messages::ClientPublicMessage for #struct_name_ident {
            #[doc = "Serialize underlying message to string"]
            fn serialize(&self, request_id: u8) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ClientPublicMessage::serialize(&self, #message_tag, request_id)
            }

            #[doc = "Deserialize string to the underlying message type"]
            #[uniffi::constructor]
            fn deserialize(data: String) -> Result<(Self, u8), crate::messages::serializers::SerializationError> {
                let data: (#struct_name_ident, u8) = crate::messages::serializers::ClientPublicMessage::deserialize(&data, #message_tag)?;
                Ok(data)
            }

            #[doc = "Return message tag"]
            fn tag() -> u16 {
                #message_tag
            }
        }

        #[cfg(all(feature = "uniffi", not(feature = "server")))]
        #[uniffi::export]
        impl #struct_name_ident {
            #[doc = "Serialize underlying message to string"]
            pub fn serialize(&self, request_id: u8) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ClientPublicMessage::serialize(&self, #message_tag, request_id)
            }

            #[doc = "Easy way to quickly output underlying message to the string for debugging purposes"]
            pub fn debug_string(&self) -> String {
                format!("{:?}", self)
            }
        }
    };
    TokenStream::from(expanded)
}

/// Procedural macros that creates a custom serialization logic for the given player client message
#[proc_macro_attribute]
pub fn client_player_message(attr: TokenStream, item: TokenStream) -> TokenStream {
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

    let message_tag_encoded = binary_encoding::encode_message_tag(message_tag);

    let expanded = quote! {
        #[doc = concat!("Message tag = ", #message_tag_encoded)]
        #[derive(std::cmp::PartialEq, std::fmt::Debug, bincode::Decode, bincode::Encode, uniffi::Object, Clone)]
        #input

        #[cfg(feature = "server")]
        impl crate::messages::ClientPlayerMessage for #struct_name_ident {
            #[doc = "Serialize underlying message to string which will include player public key and will be signed to proof it's validity"]
            fn serialize(&self, request_id: u8, public_key: crate::encryption::PublicKey, private_key: crate::encryption::PrivateKey) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ClientPlayerMessage::serialize(&self, #message_tag, request_id, &public_key, &private_key)
            }

            #[doc = "Deserialize string to the underlying message type and included public_key as a string. Returns error if signature is not valid"]
            fn deserialize(data: String) -> Result<(Self, std::sync::Arc<crate::encryption::PublicKey>, u8), crate::messages::serializers::SerializationError> {
                let data: (#struct_name_ident, std::sync::Arc<crate::encryption::PublicKey>, u8) = crate::messages::serializers::ClientPlayerMessage::deserialize(&data, #message_tag)?;
                Ok(data)
            }

            #[doc = "Return message tag"]
            fn tag() -> u16 {
                #message_tag
            }
        }

        #[cfg(all(feature = "uniffi", not(feature = "server")))]
        #[uniffi::export]
        impl #struct_name_ident {
            #[doc = "Serialize underlying message to string which will include player public key and will be signed to proof it's validity"]
            pub fn serialize(&self, request_id: u8, keys: crate::encryption::Keys) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ClientPlayerMessage::serialize(&self, #message_tag, request_id, keys.public_key.as_ref(), keys.private_key.as_ref())
            }

            #[doc = "Easy way to quickly output underlying message to the string for debugging purposes"]
            pub fn debug_string(&self) -> String {
                format!("{:?}", self)
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

    let message_tag_encoded = binary_encoding::encode_message_tag(message_tag);

    let expanded = quote! {
        #[doc = concat!("Message tag = ", #message_tag_encoded)]
        #[derive(std::cmp::PartialEq, std::fmt::Debug, bincode::Decode, bincode::Encode, uniffi::Record, Clone)]
        #input

        // For server we can have serializing logic on a struct itself, it makes things easier to work with
        #[cfg(feature = "server")]
        impl #struct_name_ident {
            #[doc = "Serialize message to string"]
            pub fn serialize(&self, request_id: u8) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ServerMessage::serialize(self, #message_tag, request_id)
            }
            #[doc = "Deserialize string back to the message type"]
            pub fn deserialize(data: &str) -> Result<(Self, u8), crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ServerMessage::deserialize(data, #message_tag)
            }
        }

        // Because of limitation of uniffi we can't add methods to uniffi::Record, so we create a second
        // struct will be responsible for data encoding
        #[cfg(feature="uniffi")]
        #[derive(uniffi::Object)]
        struct #message_serializer {
            data: #struct_name_ident,
            request_id: u8,
        }

        // To avoid conflict with duplicated "serialize" function enable it only when server is turned off
        // otherwise `cargo build --all-features` fails
        #[cfg(all(feature = "uniffi", not(feature = "server")))]
        #[uniffi::export]
        impl #message_serializer {
            #[doc = "Creates new message serializer"]
            #[uniffi::constructor]
            pub fn new(data: #struct_name_ident) -> std::sync::Arc<Self> {
                std::sync::Arc::new(Self {
                    data,
                    request_id: 0,
                })
            }

            #[doc = "Returns underlying message"]
            pub fn data(&self) -> #struct_name_ident {
                // TODO Can't we get rid of clone? It's because of uniffi, but it also used
                //      in ::serialize, so it's extra clone
                self.data.clone()
            }

            #[doc = "Returns client message request identifier for which this message was created"]
            pub fn request_id(&self) -> u8 {
                self.request_id
            }

            #[doc = "Serialize underlying message to string"]
            pub fn serialize(&self, request_id: u8) -> Result<String, crate::messages::serializers::SerializationError> {
                crate::messages::serializers::ServerMessage::serialize(&self.data, #message_tag, request_id)
            }

            #[doc = "Deserialize string to the underlying message type"]
            #[uniffi::constructor]
            pub fn deserialize(data: String) -> Result<std::sync::Arc<Self>, crate::messages::serializers::SerializationError> {
                let data: (#struct_name_ident, u8) = crate::messages::serializers::ServerMessage::deserialize(&data, #message_tag)?;
                Ok(std::sync::Arc::new(Self{data: data.0, request_id: data.1}))
            }

            #[doc = "Easy way to quickly output underlying message to the string for debugging purposes"]
            pub fn debug_string(&self) -> String {
                format!("{:?}", self.data)
            }
        }

        #[doc = "Return message tag string"]
        #[cfg(feature="uniffi")]
        #[uniffi::export]
        #[allow(non_snake_case)]
        fn #struct_message_tag_fn() -> String {
            #message_tag_encoded.to_string()
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
