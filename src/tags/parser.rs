use super::{length::parse_length_tag, tag::get_tag_type};
use crate::{
    light::{ProtoTag, WireType},
    tags::{
        fixed::{parse_fixed32, parse_fixed64},
        var::parse_var,
    },
    utils::encoding::base64_encode_standard,
};
use log::warn;
use serde_json::Value;
use std::collections::HashMap;

/// Extract the Protobuf values from the provided data
pub(crate) fn parse_tag(data: &[u8]) -> nom::IResult<&[u8], HashMap<usize, ProtoTag>> {
    let mut proto_data = data;
    let mut proto_map: HashMap<usize, ProtoTag> = HashMap::new();

    while !proto_data.is_empty() {
        let (input, tag) = get_tag_type(proto_data)?;

        let (input, value) = match tag.wire_type {
            WireType::VarInt => parse_var(input)?,
            WireType::Fixed64 => parse_fixed64(input)?,
            WireType::Len => parse_length_tag(input)?,
            WireType::StartGroup => {
                warn!("[sunlight] got start group wiretype. This is deprecated, ending parsing now. Returning base64 as final result");
                ([].as_slice(), Value::String(base64_encode_standard(input)))
            }
            WireType::EndGroup => {
                warn!("[sunlight] got end group wiretype. This is deprecated, ending parsing now. Returning base64 as final result");
                ([].as_slice(), Value::String(base64_encode_standard(input)))
            }
            WireType::Fixed32 => parse_fixed32(input)?,
            WireType::Unknown => {
                warn!("[sunlight] got unknown wire type. Protobuf data may be corrupted or this is not protobuf data, ending parsing now. Returning base64 as final result");
                ([].as_slice(), Value::String(base64_encode_standard(input)))
            }
        };

        // Existing field found. Value is should be an array
        if let Some(existing_field) = proto_map.get_mut(&tag.field) {
            if existing_field.value.is_array() {
                existing_field.value.as_array_mut().unwrap().push(value);
            } else {
                // Convert data to array of values
                existing_field.value = Value::Array(vec![existing_field.value.clone(), value]);
            }
        } else {
            let proto_tag = ProtoTag { tag, value };
            proto_map.insert(proto_tag.tag.field, proto_tag);
        }

        proto_data = input;
    }

    Ok((proto_data, proto_map))
}

#[cfg(test)]
mod tests {
    use super::parse_tag;
    use crate::light::WireType;
    use serde_json::Value;
    use std::{fs::read, path::PathBuf};

    #[test]
    fn test_parse_tag() {
        let test = [
            10, 45, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114,
            101, 100, 46, 77, 105, 103, 114, 97, 116, 111, 114, 77, 105, 115, 99, 101, 108, 108,
            97, 110, 101, 111, 117, 115, 84, 97, 115, 107, 10, 40, 99, 111, 109, 46, 97, 112, 112,
            108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46, 77, 105, 103, 114, 97,
            116, 111, 114, 65, 112, 112, 85, 115, 97, 103, 101, 84, 97, 115, 107, 10, 38, 99, 111,
            109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46,
            77, 105, 103, 114, 97, 116, 111, 114, 65, 114, 99, 97, 100, 101, 84, 97, 115, 107,
        ];

        let (_, result) = parse_tag(&test).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(
            result.get(&1).unwrap().value,
            Value::Array(vec![
                Value::String(String::from(
                    "com.apple.appstored.MigratorMiscellaneousTask"
                )),
                Value::String(String::from("com.apple.appstored.MigratorAppUsageTask")),
                Value::String(String::from("com.apple.appstored.MigratorArcadeTask"))
            ])
        );
        assert_eq!(result.get(&1).unwrap().tag.wire_type, WireType::Len);
    }

    #[test]
    fn test_parse_tag_fields() {
        let test = [
            10, 10, 112, 114, 111, 100, 117, 99, 116, 105, 111, 110, 18, 32, 99, 52, 52, 101, 49,
            48, 50, 57, 57, 57, 57, 51, 101, 101, 53, 100, 97, 56, 48, 56, 48, 98, 51, 57, 53, 51,
            57, 57, 101, 56, 50, 54,
        ];

        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&1).unwrap().value, "production");
        assert_eq!(
            result.get(&2).unwrap().value,
            "c44e10299993ee5da8080b395399e826"
        );
    }

    #[test]
    fn test_parse_tag_biome() {
        let test = [
            10, 15, 55, 53, 48, 48, 53, 54, 55, 57, 57, 54, 48, 56, 53, 57, 56, 16, 240, 249, 7,
            24, 61, 32, 1, 42, 10, 66, 105, 111, 109, 101, 65, 103, 101, 110, 116, 0, 0, 0,
        ];
        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 6);
        assert_eq!(result.get(&4).unwrap().value, 1);
        assert_eq!(
            result.get(&0).unwrap().value.as_array().unwrap(),
            &vec![0, 0]
        );
        assert_eq!(result.get(&5).unwrap().value, "BiomeAgent");
        assert_eq!(result.get(&1).unwrap().value, "750056799608598");
    }

    #[test]
    fn test_parse_tag_biome_app() {
        let test = [
            16, 1, 24, 1, 33, 217, 236, 52, 46, 208, 118, 198, 65, 50, 28, 99, 111, 109, 46, 100,
            117, 99, 107, 100, 117, 99, 107, 103, 111, 46, 109, 97, 99, 111, 115, 46, 98, 114, 111,
            119, 115, 101, 114, 74, 7, 49, 46, 49, 49, 52, 46, 48, 82, 3, 51, 48, 56, 88, 1, 96, 1,
            0, 0, 0,
        ];
        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 9);
        assert_eq!(result.get(&4).unwrap().value.to_string(), "{\"double\":753770588.413478,\"signed\":4739606294354521305,\"unsigned\":4739606294354521305}");
        assert_eq!(
            result.get(&0).unwrap().value.as_array().unwrap(),
            &vec![0, 0]
        );
        assert_eq!(
            result.get(&6).unwrap().value,
            "com.duckduckgo.macos.browser"
        );
        assert_eq!(result.get(&9).unwrap().value, "1.114.0");
    }

    #[test]
    fn test_parse_tag_biome_microsoft() {
        let test = [
            16, 1, 24, 0, 33, 19, 41, 57, 157, 203, 118, 198, 65, 50, 25, 99, 111, 109, 46, 109,
            105, 99, 114, 111, 115, 111, 102, 116, 46, 97, 117, 116, 111, 117, 112, 100, 97, 116,
            101, 50, 74, 4, 52, 46, 55, 54, 82, 13, 52, 46, 55, 54, 46, 50, 52, 49, 48, 49, 51, 56,
            55, 88, 1, 96, 1, 0, 0, 0,
        ];
        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 9);
        assert_eq!(result.get(&4).unwrap().value.to_string(), "{\"double\":753768250.446566,\"signed\":4739606274742233363,\"unsigned\":4739606274742233363}");
        assert_eq!(
            result.get(&0).unwrap().value.as_array().unwrap(),
            &vec![0, 0]
        );
        assert_eq!(result.get(&6).unwrap().value, "com.microsoft.autoupdate2");
        assert_eq!(result.get(&10).unwrap().value, "4.76.24101387");
        assert_eq!(result.get(&9).unwrap().value, "4.76");
    }

    #[test]
    fn test_parse_tag_biome_siri() {
        let test = [
            8, 1, 18, 55, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 115, 105, 114, 105, 46,
            109, 101, 116, 114, 105, 99, 115, 46, 77, 101, 116, 114, 105, 99, 115, 69, 120, 116,
            101, 110, 115, 105, 111, 110, 46, 115, 99, 111, 114, 101, 99, 97, 114, 100, 46, 100,
            97, 105, 108, 121, 26, 11, 78, 111, 116, 32, 83, 116, 97, 114, 116, 101, 100,
        ];
        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(
            result.get(&2).unwrap().value,
            "com.apple.siri.metrics.MetricsExtension.scorecard.daily"
        );
        assert_eq!(result.get(&3).unwrap().value, "Not Started");
        assert_eq!(result.get(&1).unwrap().value, 1);
    }

    #[test]
    fn test_parse_tag_biome_siri_metrics() {
        let test = [
            8, 1, 17, 0, 0, 0, 128, 76, 206, 217, 65, 25, 0, 0, 0, 32, 155, 208, 217, 65, 34, 55,
            99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 115, 105, 114, 105, 46, 109, 101, 116,
            114, 105, 99, 115, 46, 77, 101, 116, 114, 105, 99, 115, 69, 120, 116, 101, 110, 115,
            105, 111, 110, 46, 115, 99, 111, 114, 101, 99, 97, 114, 100, 46, 100, 97, 105, 108,
            121, 42, 11, 78, 111, 116, 32, 83, 116, 97, 114, 116, 101, 100, 49, 134, 227, 69, 236,
            1, 207, 217, 65, 56, 1, 64, 0, 72, 0, 81, 0, 0, 0, 192, 204, 255, 42, 64, 89, 0, 0, 0,
            0, 0, 0, 240, 191, 97, 0, 0, 0, 192, 204, 255, 42, 64, 105, 0, 0, 0, 0, 0, 0, 240, 191,
            113, 0, 0, 0, 0, 0, 0, 240, 191, 0, 0,
        ];
        let (_, result) = parse_tag(&test).unwrap();
        assert_eq!(result.len(), 15);
        assert_eq!(
            result.get(&4).unwrap().value,
            "com.apple.siri.metrics.MetricsExtension.scorecard.daily"
        );
        assert_eq!(result.get(&3).unwrap().value.to_string(), "{\"double\":1732406400.0,\"signed\":4745053047086907392,\"unsigned\":4745053047086907392}");
        assert_eq!(result.get(&12).unwrap().value.to_string(), "{\"double\":13.499608993530273,\"signed\":4623789222308872192,\"unsigned\":4623789222308872192}");
    }

    #[test]
    fn test_parse_blackboxprotobuf_test() {
        let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_path.push("tests/test_data/blackboxprotobuf/test_message.out");
        let data = read(test_path.to_str().unwrap()).unwrap();

        let (_, result) = parse_tag(&data).unwrap();
        assert_eq!(serde_json::to_string(&result).unwrap().len(), 1856);
        assert_eq!(result.get(&128).unwrap().value, 1);
        assert_eq!(
            result.get(&1024).unwrap().value.to_string(),
            "{\"float\":null,\"signed\":-20,\"unsigned\":4294967276}"
        );
        assert_eq!(result.get(&32768).unwrap().value.to_string(), "{\"2\":{\"tag\":{\"field\":2,\"tag_byte\":18,\"wire_type\":\"Len\"},\"value\":\"Test1234\"},\"3\":{\"tag\":{\"field\":3,\"tag_byte\":25,\"wire_type\":\"Fixed64\"},\"value\":{\"double\":2.1,\"signed\":4611911198408756429,\"unsigned\":4611911198408756429}}}");
    }
}
