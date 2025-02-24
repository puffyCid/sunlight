use crate::{
    light::{Tag, WireType},
    utils::nom_helper::{Endian, nom_unsigned_one_byte},
};

/// Determine Protobuf Tag type
pub(crate) fn get_tag_type(data: &[u8]) -> nom::IResult<&[u8], Tag> {
    let (mut input, tag_byte) = nom_unsigned_one_byte(data, Endian::Le)?;
    let field_number = 3;

    let field = (tag_byte >> field_number) as usize;

    let mut tag = Tag {
        tag_byte,
        wire_type: get_wire_type(&tag_byte),
        field,
    };

    let mut check_msb = tag_byte;
    // If Most significant bit is set. The next byte is part of the tag
    while (check_msb >> 7) & 1 != 0 {
        let (remaining, check) = nom_unsigned_one_byte(input, Endian::Le)?;
        tag.field *= check as usize;
        check_msb = check;
        input = remaining;
    }

    Ok((input, tag))
}

/// Determine the Tag `WireType`
fn get_wire_type(value: &u8) -> WireType {
    let wire = 7;
    match value & wire {
        0 => WireType::VarInt,
        1 => WireType::Fixed64,
        2 => WireType::Len,
        3 => WireType::StartGroup,
        4 => WireType::EndGroup,
        5 => WireType::Fixed32,
        _ => WireType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::{get_tag_type, get_wire_type};
    use crate::tags::tag::WireType;

    #[test]
    fn test_get_wire_type() {
        let test = [0, 1, 2, 3, 4, 5];
        for entry in test {
            let result = get_wire_type(&entry);
            assert_ne!(result, WireType::Unknown);
        }
    }

    #[test]
    fn test_get_tag_type() {
        let test = [
            10, 45, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114,
            101, 100, 46, 77, 105, 103, 114, 97, 116, 111, 114, 77, 105, 115, 99, 101, 108, 108,
            97, 110, 101, 111, 117, 115, 84, 97, 115, 107, 10, 40, 99, 111, 109, 46, 97, 112, 112,
            108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46, 77, 105, 103, 114, 97,
            116, 111, 114, 65, 112, 112, 85, 115, 97, 103, 101, 84, 97, 115, 107, 10, 38, 99, 111,
            109, 46, 97, 112, 112, 108, 101, 46, 97, 112, 112, 115, 116, 111, 114, 101, 100, 46,
            77, 105, 103, 114, 97, 116, 111, 114, 65, 114, 99, 97, 100, 101, 84, 97, 115, 107,
        ];

        let (_, result) = get_tag_type(&test).unwrap();
        assert_eq!(result.field, 1);
        assert_eq!(result.wire_type, WireType::Len);
        assert_eq!(result.tag_byte, 10);
    }
}
