//! Encoding of message types into JSON strings for client messages.
//!
//! AWS API Gateway expects JSON documents where one specific field is a string that is used for routing.
//! Initially, we included a `message_type` attribute, which was typically equal to the message's name.
//! However, this added 10-20 bytes of overhead to every message.
//!
//! We then decided to assign a numerical (u16) ID to each message, but since it had to be converted to a
//! string, an ID like 101 would become "101", which is already 3 bytes.
//!
//! This encoding implementation converts a u16 to a string where values up to 94 take only 1 byte, and values
//! up to MAX_VALID_ID=8835 will take 2 bytes.

/// Array of character codes that can be safely embedded in JSON string, meaning inside "[HERE]"
/// Excluding control characters (codes from 0 to 31), then no quote (34) and no slash (92)
/// and nothing bigger than 127 as it creates not valid UTF8. 
/// 
/// Actually for the ID which takes two bytes we can use an extended set for the second byte that may include 
/// values in range [0x80; 0xBF] and it still have valid UTF8.
const VALID_CHAR_SET: [u8; 94] = [
    32, 33, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
    57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
    81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
    104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122,
    123, 124, 125, 126, 127,
];

/// Maximum possible ID that we can encode
pub const MAX_VALID_ID: u16 = (VALID_CHAR_SET.len() as u16).pow(2) - 1;

/// Encodes a message ID (u16) into a JSON-safe string using 1 or 2 characters.
/// IDs that fit into 1 character will return a single-byte string, while larger
/// IDs will use 2 bytes. Panics if the ID exceeds MAX_VALID_ID
pub fn encode_as_json_bytes(mut id: u16) -> Vec<u8> {
    if id > MAX_VALID_ID {
        panic!("Cannot encode ID as JSON-safe bytes");
    }
    let base = VALID_CHAR_SET.len() as u16;
    let mut result = Vec::new();
    result.push(VALID_CHAR_SET[(id % base) as usize]);
    id /= base;
    if id > 0 {
        result.push(VALID_CHAR_SET[(id % base) as usize]);
    }
    result.reverse();
    result
}

/// Decodes a JSON-safe string back into its original message ID (u16).
/// Expects a string of 1 or 2 characters from the `VALID_CHAR_SET`.
pub fn decode_from_json_bytes(encoded: &[u8]) -> u16 {
    if encoded.len() > 2 || encoded.is_empty() {
        panic!("Encoded string must have 1 or 2 bytes");
    }
    let base = VALID_CHAR_SET.len() as u16;
    let mut id = 0u16;
    for &c in encoded {
        let pos = VALID_CHAR_SET
            .iter()
            .position(|&v| v == c)
            .expect("Invalid byte in encoded string");
        id = id * base + pos as u16;
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

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
        assert_eq!(good_chars, Vec::from(VALID_CHAR_SET));
    }

    #[test]
    fn test_encoding_decoding() {
        // Check that values are in expected form
        let vals = vec![
            (0, vec![32]),
            (1, vec![33]),
            (94, vec![33, 32]),
            ((VALID_CHAR_SET.len() - 1) as u16, vec![127]),
            ((VALID_CHAR_SET.len()) as u16, vec![33, 32]),
            ((VALID_CHAR_SET.len() + 1) as u16, vec![33, 33]),
            (MAX_VALID_ID, vec![127, 127]),
        ];
        for (id, want) in vals {
            let encoded = encode_as_json_bytes(id);
            assert_eq!(
                encoded, want,
                "Bad encoded value for id={id}, got={encoded:?}, want={want:?}"
            );
            let decoded = decode_from_json_bytes(&encoded);
            assert_eq!(
                decoded, id,
                "Bad decoded value for id={id}, decoded={decoded}, encoded={encoded:?}"
            )
        }
    }

    #[test]
    fn test_encoding_decoding_all() {
        for id in 0..MAX_VALID_ID {
            // Check that conversion works both way
            let encoded = encode_as_json_bytes(id);
            let decoded = decode_from_json_bytes(&encoded);
            assert_eq!(
                decoded, id,
                "Failed test for id='{id}', encoded='{encoded:#X?}', decoded='{decoded:#X?}'"
            );

            // Check that conversion through JSON works as well
            let mut json = vec![];
            json.extend_from_slice(r#"{"k":""#.as_bytes());
            json.extend_from_slice(&encoded);
            json.extend_from_slice(r#""}"#.as_bytes());
            let doc = serde_json::from_slice::<Value>(&json).unwrap();
            let got = decode_from_json_bytes(doc["k"].as_str().unwrap().as_bytes());
            assert_eq!(got, id);
        }
    }
}
