use crate::{
    tags::parser::parse_tag,
    utils::{
        encoding::base64_encode_standard,
        nom_helper::{Endian, nom_unsigned_one_byte},
        strings::extract_utf8_string,
    },
};
use nom::bytes::complete::take;
use serde_json::Value;

/// Parse length based tags. The value can be either a string or nested object (sub-message)
pub(crate) fn parse_length_tag(data: &[u8]) -> nom::IResult<&[u8], Value> {
    let (input, value_length) = nom_unsigned_one_byte(data, Endian::Le)?;
    let (input, value) = take(value_length)(input)?;

    // Try string parsing first
    let message = extract_utf8_string(value);

    // If we fail, fallback to sub-message parsing
    if message.starts_with("Failed to get UTF8 string") {
        let result = parse_tag(value);
        let sub = match result {
            Ok((_, result)) => result,
            Err(_err) => {
                // If not string or submessage might be raw bytes?
                return Ok((
                    input,
                    serde_json::to_value(base64_encode_standard(value)).unwrap_or(Value::Null),
                ));
            }
        };
        return Ok((input, serde_json::to_value(sub).unwrap_or(Value::Null)));
    }

    Ok((input, Value::String(message)))
}

#[cfg(test)]
mod tests {
    use super::parse_length_tag;

    #[test]
    fn test_parse_length_tag() {
        let test = [
            45, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114,
            101, 100, 46, 77, 105, 103, 114, 97, 116, 111, 114, 77, 105, 115, 99, 101, 108, 108,
            97, 110, 101, 111, 117, 115, 84, 97, 115, 107, 10, 40, 99, 111, 109, 46, 97, 112, 112,
            108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46, 77, 105, 103, 114, 97,
            116, 111, 114, 65, 112, 112, 85, 115, 97, 103, 101, 84, 97, 115, 107, 10, 38, 99, 111,
            109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46,
            77, 105, 103, 114, 97, 116, 111, 114, 65, 114, 99, 97, 100, 101, 84, 97, 115, 107,
        ];

        let (remaining, result) = parse_length_tag(&test).unwrap();
        assert_eq!(result, "com.apple.appstored.MigratorMiscellaneousTask");
        assert_eq!(remaining.len(), 82);
    }
}
