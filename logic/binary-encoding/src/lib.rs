//! Binary encoding for messages and message tags.
//!
//! AWS API Gateway WebSockets do not support binary data and lack compression functionality. Messages
//! sent from the client to the API must be JSON documents, as the gateway routes them to the correct AWS Lambda
//! using one of the JSON fields as a key. For messages sent back from API to the client, any string is acceptable
//! as long as it is valid UTF-8.
//!
//! To support client message routing, we provide a way to encode numerical `u16` client message tags into a JSON-safe
//! string with char set which allowed by API Gateway. In this encoding we are using the most efficient encoding possible
//! while remaining valid UTF-8 and safe to use inside JSON field that is acceptable as route. Message tag are encoded as
//! two bytes for any tag up to 4_356 which should be enough for our cases.
//!
//! Additionally, there is functionality to encode an arbitrary array of bytes using Base94 encoding for entire server
//! messages. Using the more common Base64 adds 33% space overhead, while this Base94 adds only 22%.

/// All printable characters, excluding quote and slash so that the encoded string can be safely embedded in a JSON string
const CHAR_SET_JSON_STRING: [u8; 94] = [
    32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    123, 124, 125, 126, 127,
];

/// Message tags are used for routing in AWS API Gateway which allows only numbers, letters and
/// 4 symbols: - , / _
const CHAR_SET_API_GATEWAY_ROUTE: [u8; 66] = [
    45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75,
    76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 95, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
];

/// Maximum possible message tag that we can encode
pub const MAX_VALID_TAG: u16 = (CHAR_SET_API_GATEWAY_ROUTE.len() as u16).pow(2) - 1;

/// How much request id takes in a serialized string
pub const REQUEST_ID_LEN: usize = 2;

#[derive(Debug)]
pub enum EncodingError {
    BadData(&'static str),
}

/// Encode binary data using Base94 encoding
pub fn encode_base94(data: &[u8]) -> String {
    base_x::encode(CHAR_SET_JSON_STRING.as_ref(), data)
}

/// Decode data from a Base94 string back into bytes
pub fn decode_base94(encoded: &str) -> Result<Vec<u8>, EncodingError> {
    let data = base_x::decode(CHAR_SET_JSON_STRING.as_ref(), encoded)
        .map_err(|_| EncodingError::BadData("Bad data"))?;
    Ok(data)
}

/// Encodes a message tag (u16) to a string of exactly 2 characters which can be used as a routing key in messages
/// sent to API Gateway. Panics if the tag exceeds MAX_VALID_TAG.
pub fn encode_message_tag(mut tag: u16) -> String {
    if tag > MAX_VALID_TAG {
        panic!("Message tag too big to fit into 2 bytes");
    }
    let base = CHAR_SET_API_GATEWAY_ROUTE.len() as u16;
    let mut result = Vec::new();

    // Push the least significant character
    result.push(CHAR_SET_API_GATEWAY_ROUTE[(tag % base) as usize]);
    tag /= base;

    // If tag is greater than 0, push the next character. Otherwise, push a padding character.
    if tag > 0 {
        result.push(CHAR_SET_API_GATEWAY_ROUTE[(tag % base) as usize]);
    } else {
        // Add padding character (using the first element of CHAR_SET_API_GATEWAY_ROUTE)
        result.push(CHAR_SET_API_GATEWAY_ROUTE[0]);
    }

    // The order is reversed, so reverse back to ensure correct order
    result.reverse();
    String::from_utf8(result).expect("Our custom charset should always be convertible to String")
}

/// Encode request id to the string of 2 bytes
pub fn encode_request_id(id: u8) -> String {
    // We can't encode u8 as a valid UTF8 string if we want to represent it as a 1 byte length string
    // So we simply encode it in the same way as tag which is guaranteed to fit into 2 bytes
    encode_message_tag(id as u16)
}

/// Decode request id back to the original value
pub fn decode_request_id(data: &[u8]) -> Result<u8, EncodingError> {
    let v = decode_message_tag(data)?;
    u8::try_from(v).map_err(|_| EncodingError::BadData("Not valid request id"))
}

/// Decodes encoded tag string back into its original tag value (u16). Expects a string of exactly 2
/// characters from the `CHAR_SET_API_GATEWAY_ROUTE` and returns an error if otherwise.
pub fn decode_message_tag(data: &[u8]) -> Result<u16, EncodingError> {
    if data.len() != 2 {
        return Err(EncodingError::BadData(
            "Encoded message tag string must be exactly 2 bytes long",
        ));
    }
    let base = CHAR_SET_API_GATEWAY_ROUTE.len() as u16;
    let mut tag = 0u16;

    for &c in data {
        let pos = CHAR_SET_API_GATEWAY_ROUTE
            .iter()
            .position(|&v| v == c)
            .ok_or(EncodingError::BadData("Invalid byte in encoded string"))?;
        tag = tag * base + pos as u16;
    }

    Ok(tag)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use serde_json::Value;

    #[test]
    fn check_valid_gateway_route_chars() {
        let valid_symbols = b"-./_";
        for c in CHAR_SET_API_GATEWAY_ROUTE {
            let c = c as char;
            if !c.is_ascii_digit() && !c.is_ascii_alphabetic() {
                assert!(valid_symbols.contains(&(c as u8)));
            }
        }
    }

    #[test]
    fn test_valid_char_set() {
        let mut good_chars = vec![];
        for i in 0..=u8::MAX {
            let character = i as char;
            let mut json = vec![];
            json.extend_from_slice(r#"{"k":""#.as_bytes());
            json.push(character as u8);
            json.extend_from_slice(r#""}"#.as_bytes());
            if let Ok(val) = serde_json::from_slice::<Value>(&json) {
                let value = val["k"].as_str().unwrap_or("");
                if value == character.to_string() {
                    good_chars.push(i);
                }
            }
        }
        assert_eq!(good_chars, Vec::from(CHAR_SET_JSON_STRING));
    }

    #[test]
    fn test_tag_encoding() {
        // Check that values are in the expected most efficient form
        let vals = vec![
            (0, vec![45, 45]),
            (1, vec![45, 46]),
            (66, vec![46, 45]),
            ((CHAR_SET_API_GATEWAY_ROUTE.len() - 1) as u16, vec![45, 122]),
            ((CHAR_SET_API_GATEWAY_ROUTE.len()) as u16, vec![46, 45]),
            ((CHAR_SET_API_GATEWAY_ROUTE.len() + 1) as u16, vec![46, 46]),
            (MAX_VALID_TAG, vec![122, 122]),
        ];
        for (id, want) in vals {
            let encoded = encode_message_tag(id);
            let want = String::from_utf8(want).unwrap();
            assert_eq!(
                encoded, want,
                "Bad encoded value for id={id}, got={encoded:?}, want={want:?}"
            );
            let decoded = decode_message_tag(encoded.as_bytes()).unwrap();
            assert_eq!(
                decoded, id,
                "Bad decoded value for id={id}, decoded={decoded}, encoded={encoded:?}"
            )
        }
    }

    #[test]
    fn test_tag_encoding_decoding() {
        for tag in 0..MAX_VALID_TAG {
            // Check that conversion works both ways
            let encoded = encode_message_tag(tag);
            let decoded = decode_message_tag(encoded.as_bytes()).unwrap();
            assert_eq!(
                decoded, tag,
                "Failed test for id='{tag}', encoded='{encoded:#X?}', decoded='{decoded:#X?}'"
            );

            // Check that conversion through JSON works as well
            let mut json = vec![];
            json.extend_from_slice(r#"{"k":""#.as_bytes());
            json.extend_from_slice(encoded.as_bytes());
            json.extend_from_slice(r#""}"#.as_bytes());
            let doc = serde_json::from_slice::<Value>(&json).unwrap();
            let got = decode_message_tag(doc["k"].as_str().unwrap().as_bytes()).unwrap();
            assert_eq!(got, tag);
        }
    }

    #[test]
    fn test_encoding_overhead() {
        let mut size_base64 = 0;
        let mut size_base94 = 0;
        let mut size_data = 0;
        for len in 0..=100 {
            let mut data = vec![1u8; len];
            for i in 0..len {
                for byte_value in 0..=255 {
                    data[i] = byte_value;
                    let encoded = encode_base94(&data);
                    size_data += data.len();
                    size_base94 += encoded.as_bytes().len();
                    size_base64 += base64::prelude::BASE64_STANDARD_NO_PAD
                        .encode(&data)
                        .as_bytes()
                        .len();
                    let decoded = decode_base94(&encoded).unwrap();
                    assert_eq!(
                        data, decoded,
                        "Mismatch found in data {:?}, encoded={:?}, decoded={:?}",
                        data, encoded, decoded
                    );
                }
            }
        }

        // Just to validate our assumption about base encoding overhead and ensure it still holds, for
        // base94 we should get an 11% improvement compared to the base64 overhead of 33%
        let overhead_base64 = ((size_base64 as f32 / size_data as f32) * 100f32) as usize - 100;
        let overhead_base94 = ((size_base94 as f32 / size_data as f32) * 100f32) as usize - 100;
        assert_eq!(overhead_base64, 33);
        assert_eq!(overhead_base94, 22);
    }

    #[test]
    fn request_id_encode_decode() {
        for i in 0..=u8::MAX {
            let encoded = encode_request_id(i);
            assert_eq!(encoded.len(), REQUEST_ID_LEN);
            let decoded = decode_request_id(encoded.as_bytes()).unwrap();
            assert_eq!(decoded, i);
        }
    }
}
