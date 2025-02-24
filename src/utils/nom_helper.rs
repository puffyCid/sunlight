use nom::{
    bytes::complete::take,
    number::complete::{le_i32, le_i64, le_u8, le_u32, le_u64},
};
use std::mem::size_of;

pub(crate) enum Endian {
    /**Little Endian */
    Le,
}

/**
 * Nom four (4) bytes to u32
 * Need to specify Endianess
 */
pub(crate) fn nom_unsigned_four_bytes(data: &[u8], endian: Endian) -> nom::IResult<&[u8], u32> {
    let (input, value_data) = take(size_of::<u32>())(data)?;

    let (_, value) = match endian {
        Endian::Le => le_u32(value_data)?,
    };

    Ok((input, value))
}

/**
 * Nom eight (8) bytes to u64
 * Need to specify Endianess
 */
pub(crate) fn nom_unsigned_eight_bytes(data: &[u8], endian: Endian) -> nom::IResult<&[u8], u64> {
    let (input, value_data) = take(size_of::<u64>())(data)?;

    let (_, value) = match endian {
        Endian::Le => le_u64(value_data)?,
    };
    Ok((input, value))
}

/**
 * Nom one (1) bytes to u8
 * Need to specify Endianess
 */
pub(crate) fn nom_unsigned_one_byte(data: &[u8], endian: Endian) -> nom::IResult<&[u8], u8> {
    let (input, value_data) = take(size_of::<u8>())(data)?;

    let (_, value) = match endian {
        Endian::Le => le_u8(value_data)?,
    };
    Ok((input, value))
}

/**
 * Nom four (4) bytes to i32
 * Need to specify Endianess
 */
pub(crate) fn nom_signed_four_bytes(data: &[u8], endian: Endian) -> nom::IResult<&[u8], i32> {
    let (input, value_data) = take(size_of::<u32>())(data)?;

    let (_, value) = match endian {
        Endian::Le => le_i32(value_data)?,
    };

    Ok((input, value))
}

/**
 * Nom eight (8) bytes to i64
 * Need to specify Endianess
 */
pub(crate) fn nom_signed_eight_bytes(data: &[u8], endian: Endian) -> nom::IResult<&[u8], i64> {
    let (input, value_data) = take(size_of::<u64>())(data)?;

    let (_, value) = match endian {
        Endian::Le => le_i64(value_data)?,
    };
    Ok((input, value))
}

#[cfg(test)]
mod tests {
    use crate::utils::nom_helper::{
        Endian, nom_signed_eight_bytes, nom_signed_four_bytes, nom_unsigned_eight_bytes,
        nom_unsigned_four_bytes, nom_unsigned_one_byte,
    };

    #[test]
    fn test_nom_signed_eight_bytes() {
        let test = [2, 0, 0, 0, 0, 0, 0, 0];
        let (_, results) = nom_signed_eight_bytes(&test, Endian::Le).unwrap();
        assert_eq!(results, 2);
    }

    #[test]
    fn test_nom_signed_four_bytes() {
        let test = [2, 0, 0, 0];
        let (_, results) = nom_signed_four_bytes(&test, Endian::Le).unwrap();
        assert_eq!(results, 2);
    }

    #[test]
    fn test_nom_unsigned_four_bytes() {
        let test = [0, 0, 0, 2];
        let (_, results) = nom_unsigned_four_bytes(&test, Endian::Le).unwrap();
        assert_eq!(results, 33554432);
    }

    #[test]
    fn test_nom_unsigned_eight_bytes() {
        let test = [0, 0, 0, 0, 0, 0, 0, 2];
        let (_, results) = nom_unsigned_eight_bytes(&test, Endian::Le).unwrap();
        assert_eq!(results, 144115188075855872);
    }

    #[test]
    fn test_nom_unsigned_one_byte() {
        let test = [2];
        let (_, results) = nom_unsigned_one_byte(&test, Endian::Le).unwrap();
        assert_eq!(results, 2);
    }
}
