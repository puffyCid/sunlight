use crate::utils::nom_helper::{
    Endian, nom_signed_eight_bytes, nom_signed_four_bytes, nom_unsigned_eight_bytes,
    nom_unsigned_four_bytes,
};
use nom::{
    bytes::complete::take,
    number::{complete::le_f32, streaming::le_f64},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub(crate) struct Fixed64 {
    signed: i64,
    unsigned: u64,
    double: f64,
}

#[derive(Serialize)]
pub(crate) struct Fixed32 {
    signed: i32,
    unsigned: u32,
    float: f32,
}

/// Parsed a fixed 8 byte value. This can be signed, unsiged or float64 (double). So we return all 3 options. Its likely a float64
pub(crate) fn parse_fixed64(data: &[u8]) -> nom::IResult<&[u8], Value> {
    let (_, signed) = nom_signed_eight_bytes(data, Endian::Le)?;
    let (_, unsigned) = nom_unsigned_eight_bytes(data, Endian::Le)?;
    let (input, float_bytes) = take(size_of::<f64>())(data)?;

    let (_, double) = le_f64(float_bytes)?;
    let fixed = Fixed64 {
        signed,
        unsigned,
        double,
    };

    Ok((input, serde_json::to_value(fixed).unwrap_or(Value::Null)))
}

/// Parsed a fixed 4 byte value. This can be signed, unsiged or float. So we return all 3 options. Its likely a float
pub(crate) fn parse_fixed32(data: &[u8]) -> nom::IResult<&[u8], Value> {
    let (_, signed) = nom_signed_four_bytes(data, Endian::Le)?;
    let (_, unsigned) = nom_unsigned_four_bytes(data, Endian::Le)?;
    let (input, float_bytes) = take(size_of::<f32>())(data)?;

    let (_, float) = le_f32(float_bytes)?;
    let fixed = Fixed32 {
        signed,
        unsigned,
        float,
    };

    Ok((input, serde_json::to_value(fixed).unwrap_or(Value::Null)))
}

#[cfg(test)]
mod tests {
    use super::{parse_fixed32, parse_fixed64};

    #[test]
    fn test_parse_fixed64() {
        let test = [
            217, 236, 52, 46, 208, 118, 198, 65, 50, 28, 99, 111, 109, 46, 100, 117, 99, 107, 100,
            117, 99, 107, 103, 111, 46, 109, 97, 99, 111, 115, 46, 98, 114, 111, 119, 115, 101,
            114, 74, 7, 49, 46, 49, 49, 52, 46, 48, 82, 3, 51, 48, 56, 88, 1, 96, 1, 0, 0, 0,
        ];
        let (remaining, result) = parse_fixed64(&test).unwrap();
        assert_eq!(remaining.len(), 51);
        assert_eq!(
            result.to_string(),
            "{\"double\":753770588.413478,\"signed\":4739606294354521305,\"unsigned\":4739606294354521305}"
        );
    }

    #[test]
    fn test_parse_fixed32() {
        let test = [217, 236, 52, 46];
        let (remaining, result) = parse_fixed32(&test).unwrap();
        assert_eq!(remaining.len(), 0);
        assert_eq!(
            result.to_string(),
            "{\"float\":4.1137624556819574e-11,\"signed\":775220441,\"unsigned\":775220441}"
        );
    }
}
