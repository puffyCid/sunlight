use crate::utils::encoding::base64_encode_standard;
use log::warn;

/// Get a UTF8 string from provided bytes data. Invalid UTF8 is base64 encoded. Use `extract_uf8_string_lossy` if replacing bytes is acceptable
pub(crate) fn extract_utf8_string(data: &[u8]) -> String {
    let utf8_result = String::from_utf8(data.to_vec());
    match utf8_result {
        Ok(result) => result.trim_end_matches('\0').to_string(),
        Err(err) => {
            warn!("Failed to get UTF8 string for Protobuf: {err:?}");
            let max_size = 2097152;
            let issue = if data.len() < max_size {
                base64_encode_standard(data)
            } else {
                format!("Binary data size larger than 2MB, size: {}", data.len())
            };
            format!("Failed to get UTF8 string: {}", issue)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::strings::extract_utf8_string;

    #[test]
    fn test_extract_utf8_string() {
        let test_data = vec![
            112, 112, 115, 116, 111, 114, 101, 100, 46, 77, 105, 103, 114, 97, 116, 111, 114, 77,
            105, 115, 99, 101, 108, 108,
        ];
        assert_eq!(extract_utf8_string(&test_data), "ppstored.MigratorMiscell")
    }
}
