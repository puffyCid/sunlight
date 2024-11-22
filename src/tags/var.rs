use crate::utils::nom_helper::{nom_unsigned_one_byte, Endian};
use serde_json::Value;

/// Parse var based tags. Will be a number representing one of: int32, int64, uint32, uint64, sint32, sint64, bool, or enum
pub(crate) fn parse_var(data: &[u8]) -> nom::IResult<&[u8], Value> {
    let mut proto_data = data;
    let mut var_value: isize = 0;

    let mut shift = 0;
    let adjust = 0x7f;
    let wire = 7;
    let done = 0x80;
    while !proto_data.is_empty() {
        let (input, value) = nom_unsigned_one_byte(proto_data, Endian::Le)?;
        var_value += (value as isize & adjust) << (shift * wire);
        shift += 1;

        proto_data = input;
        if (value & done) == 0 {
            break;
        }
    }
    Ok((proto_data, Value::Number(var_value.into())))
}

#[cfg(test)]
mod tests {
    use super::parse_var;

    #[test]
    fn test_parse_var() {
        let test = [
            240, 249, 7, 24, 61, 32, 1, 42, 10, 66, 105, 111, 109, 101, 65, 103, 101, 110, 116, 0,
            0, 0,
        ];
        let (remaining, result) = parse_var(&test).unwrap();
        assert_eq!(remaining.len(), 19);
        assert_eq!(result, 130288);
    }
}
