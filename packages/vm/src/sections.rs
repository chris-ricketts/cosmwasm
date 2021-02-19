use crate::conversion::to_u32;
use crate::errors::VmResult;

/// Decodes sections of data into multiple slices.
///
/// Each encoded section is suffixed by a section length, encoded as big endian uint32.
///
/// See also: `encode_section`.
pub fn decode_sections(data: &[u8]) -> Vec<&[u8]> {
    let mut result: Vec<&[u8]> = vec![];
    let lengths = extract_lengths(data);
    let mut start = 0;
    for length in lengths.iter().rev() {
        result.push(&data[start..start + length]);
        start += length + 4;
    }
    result
}

/// Encodes multiple sections of data into one vector.
///
/// Each section is suffixed by a section length encoded as big endian uint32.
/// Using suffixes instead of prefixes allows reading sections in reverse order,
/// such that the first element does not need to be re-allocated if the contract's
/// data structure supports truncation (such as a Rust vector).
///
/// The resulting data looks like this:
///
/// ```ignore
/// section1 || section1_len || section2 || section2_len || section3 || section3_len || …
/// ```
#[allow(dead_code)]
pub fn encode_sections(sections: &[Vec<u8>]) -> VmResult<Vec<u8>> {
    let mut out_len: usize = sections.iter().map(|section| section.len()).sum();
    out_len += 4 * sections.len();
    let mut out_data = Vec::with_capacity(out_len);
    for section in sections {
        let section_len = to_u32(section.len())?.to_be_bytes();
        out_data.extend(section);
        out_data.extend_from_slice(&section_len);
    }
    debug_assert_eq!(out_data.len(), out_len);
    debug_assert_eq!(out_data.capacity(), out_len);
    Ok(out_data)
}

/// Extracts the lengths of encoded vectors.
///
/// Starts from the end of `data`, and computes a (reverse) list of lengths.
fn extract_lengths(data: &[u8]) -> Vec<usize> {
    let mut lengths = vec![];
    let mut remaining_len = data.len();
    while remaining_len >= 4 {
        let tail_len = u32::from_be_bytes([
            data[remaining_len - 4],
            data[remaining_len - 3],
            data[remaining_len - 2],
            data[remaining_len - 1],
        ]) as usize;
        lengths.push(tail_len);
        remaining_len -= 4 + tail_len;
    }
    lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_sections_works_for_empty_sections() {
        let dec = decode_sections(&[]);
        assert_eq!(dec.len(), 0);
        let dec = decode_sections(b"\0\0\0\0");
        assert_eq!(dec, &[&[0u8; 0]]);
        let dec = decode_sections(b"\0\0\0\0\0\0\0\0");
        assert_eq!(dec, &[&[0u8; 0]; 2]);
        let dec = decode_sections(b"\0\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(dec, &[&[0u8; 0]; 3]);
        // ignores "trailing" stuff
        let dec = decode_sections(b"\0\0\0\0\0\0\0\0\0\0\0");
        assert_eq!(dec, &[&[0u8; 0]; 2]);
    }

    #[test]
    fn decode_sections_works_for_one_element() {
        let dec = decode_sections(b"\xAA\0\0\0\x01");
        assert_eq!(dec, &[vec![0xAA]]);
        let dec = decode_sections(b"\xAA\xBB\0\0\0\x02");
        assert_eq!(dec, &[vec![0xAA, 0xBB]]);
        let dec = decode_sections(b"\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\0\0\x01\x15");
        assert_eq!(dec, &[vec![0x9D; 277]]);
    }

    #[test]
    fn decode_sections_works_for_two_elements() {
        let data = b"\xAA\0\0\0\x01\xBB\xCC\0\0\0\x02".to_vec();
        assert_eq!(decode_sections(&data), &[vec![0xAA], vec![0xBB, 0xCC]]);
        let data = b"\xDE\xEF\x62\0\0\0\x03\0\0\0\0".to_vec();
        assert_eq!(decode_sections(&data), &[vec![0xDE, 0xEF, 0x62], vec![]]);
        let data = b"\0\0\0\0\xDE\xEF\x62\0\0\0\x03".to_vec();
        assert_eq!(decode_sections(&data), &[vec![], vec![0xDE, 0xEF, 0x62]]);
        let data = b"\0\0\0\0\0\0\0\0".to_vec();
        assert_eq!(decode_sections(&data), &[vec![0u8; 0], vec![]]);
        let data = b"\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x13\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\0\0\x01\x15".to_vec();
        assert_eq!(decode_sections(&data), &[vec![0xFF; 19], vec![0x9D; 277]]);
    }

    #[test]
    fn decode_sections_works_for_multiple_elements() {
        let dec = decode_sections(b"\xAA\0\0\0\x01");
        assert_eq!(dec, &[vec![0xAA]]);
        let dec = decode_sections(b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02");
        assert_eq!(dec, &[vec![0xAA], vec![0xDE, 0xDE]]);
        let dec = decode_sections(b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02\0\0\0\0");
        assert_eq!(dec, &[vec![0xAA], vec![0xDE, 0xDE], vec![]]);
        let dec = decode_sections(b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x13");
        assert_eq!(dec, &[vec![0xAA], vec![0xDE, 0xDE], vec![], vec![0xFF; 19]]);
    }

    #[test]
    fn encode_sections_works_for_empty_sections() {
        let enc = encode_sections(&[]).unwrap();
        assert_eq!(enc, b"" as &[u8]);
        let enc = encode_sections(&[vec![]]).unwrap();
        assert_eq!(enc, b"\0\0\0\0" as &[u8]);
        let enc = encode_sections(&[vec![], vec![]]).unwrap();
        assert_eq!(enc, b"\0\0\0\0\0\0\0\0" as &[u8]);
        let enc = encode_sections(&[vec![], vec![], vec![]]).unwrap();
        assert_eq!(enc, b"\0\0\0\0\0\0\0\0\0\0\0\0" as &[u8]);
    }

    #[test]
    fn encode_sections_works_for_one_element() {
        let enc = encode_sections(&[]).unwrap();
        assert_eq!(enc, b"" as &[u8]);
        let enc = encode_sections(&[vec![0xAA]]).unwrap();
        assert_eq!(enc, b"\xAA\0\0\0\x01" as &[u8]);
        let enc = encode_sections(&[vec![0xAA, 0xBB]]).unwrap();
        assert_eq!(enc, b"\xAA\xBB\0\0\0\x02" as &[u8]);
        let enc = encode_sections(&[vec![0x9D; 277]]).unwrap();
        assert_eq!(enc, b"\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\x9D\0\0\x01\x15" as &[u8]);
    }

    #[test]
    fn encode_sections_works_for_multiple_elements() {
        let enc = encode_sections(&[vec![0xAA]]).unwrap();
        assert_eq!(enc, b"\xAA\0\0\0\x01" as &[u8]);
        let enc = encode_sections(&[vec![0xAA], vec![0xDE, 0xDE]]).unwrap();
        assert_eq!(enc, b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02" as &[u8]);
        let enc = encode_sections(&[vec![0xAA], vec![0xDE, 0xDE], vec![]]).unwrap();
        assert_eq!(enc, b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02\0\0\0\0" as &[u8]);
        let enc = encode_sections(&[vec![0xAA], vec![0xDE, 0xDE], vec![], vec![0xFF; 19]]).unwrap();
        assert_eq!(enc, b"\xAA\0\0\0\x01\xDE\xDE\0\0\0\x02\0\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\x13" as &[u8]);
    }
}
