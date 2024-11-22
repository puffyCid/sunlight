use crate::{error::SunlightError, tags::parser::parse_tag};
use log::error;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ProtoTag {
    pub tag: Tag,
    pub value: Value,
}

#[derive(Debug, Serialize)]
pub struct Tag {
    pub tag_byte: u8,
    pub wire_type: WireType,
    /**`HashMap` key */
    pub field: usize,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum WireType {
    VarInt,
    Fixed64,
    Len,
    /** Deprecated */
    StartGroup,
    /** Deprecated */
    EndGroup,
    Fixed32,
    Unknown,
}

/// Attempt to extract data from provided Protobuf bytes. Returns a `HashMap` representing the Protobuf data
///
/// # Example
/// ```rust
/// let proto_bytes = [
///           10, 45, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114,
///           101, 100, 46, 77, 105, 103, 114, 97, 116, 111, 114, 77, 105, 115, 99, 101, 108, 108,
///           97, 110, 101, 111, 117, 115, 84, 97, 115, 107, 10, 40, 99, 111, 109, 46, 97, 112, 112,
///           108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46, 77, 105, 103, 114, 97,
///           116, 111, 114, 65, 112, 112, 85, 115, 97, 103, 101, 84, 97, 115, 107, 10, 38, 99, 111,
///           109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46,
///           77, 105, 103, 114, 97, 116, 111, 114, 65, 114, 99, 97, 100, 101, 84, 97, 115, 107];
/// let proto_map = sunlight::light::extract_protobuf(&proto_bytes).unwrap();
/// assert_eq!(
///       proto_map.get(&1).unwrap().value,
///       serde_json::Value::Array(vec![
///       serde_json::Value::String(String::from("com.apple.appstored.MigratorMiscellaneousTask")),
///       serde_json::Value::String(String::from("com.apple.appstored.MigratorAppUsageTask")),
///       serde_json::Value::String(String::from("com.apple.appstored.MigratorArcadeTask"))]));
/// assert_eq!(proto_map.get(&1).unwrap().tag.wire_type, sunlight::light::WireType::Len);
/// ```
/** ```json

 JSON representation
  {
    "1": {
        "tag": {
            "field": 1,
            "tag_byte": 10,
            "wire_type": "Len"
        },
        "value": [
            "com.apple.appstored.MigratorMiscellaneousTask",
            "com.apple.appstored.MigratorAppUsageTask",
            "com.apple.appstored.MigratorArcadeTask"
        ]
    }
}
```
*/
pub fn extract_protobuf(data: &[u8]) -> Result<HashMap<usize, ProtoTag>, SunlightError> {
    let proto_result = parse_tag(data);
    let proto_map = match proto_result {
        Ok((_, results)) => results,
        Err(err) => {
            error!("[sunlight] could not parse provided protobuf bytes: {err:?}");
            return Err(SunlightError::Parser);
        }
    };

    Ok(proto_map)
}

#[cfg(test)]
mod tests {
    use super::extract_protobuf;

    #[test]
    #[should_panic(expected = "Parser")]
    fn test_extract_protobuf() {
        let bad_data = [0, 0, 1, 4, 5, 0, 0];
        let _ = extract_protobuf(&bad_data).unwrap();
    }
}
