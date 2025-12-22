use std::{fs::read, path::PathBuf};
use serde_json::json;
use sunlight::light::extract_protobuf;

#[test]
fn test_protobuf_parser() {
    let mut test_location = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_location.push("tests/test_data/stats/proto.raw");
    let bytes = read(test_location.to_str().unwrap()).unwrap();

    let value = extract_protobuf(&bytes).unwrap();

    assert_eq!(value.len(), 7);
    assert_eq!(value.get(&23).unwrap().value.as_array().unwrap().len(), 1736);
    assert_eq!(value.get(&23).unwrap().value.as_array().unwrap()[23].as_object().unwrap().get("10").unwrap().as_object().unwrap().get("value").unwrap().as_number().unwrap(), json!(4099380458 as u32).as_number().unwrap())
}
