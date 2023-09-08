use super::digest;

#[test]
#[rustfmt::skip]
fn test_empty_data() {
    let data = b"";
    assert_eq!(data, &[]);
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x00000000, digest: 0x02CC5D05 },
        SeedDigest { seed: 0x00000001, digest: 0x0B2CB792 },
        SeedDigest { seed: 0x76543210, digest: 0xA7EEC56E },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_6_bytes() {
    let data = b"xxHash";
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x00000000, digest: 0xAEA3647D },
        SeedDigest { seed: 0x00000001, digest: 0x6F3D609E },
        SeedDigest { seed: 0x76543210, digest: 0x7ED7FF35 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_16_bytes() {
    let bytes: [u8; 16] = [
        0x65, 0x99, 0xC3, 0xBF, 0x60, 0x8F, 0x92, 0xE9, 
        0x94, 0xE5, 0x91, 0x1C, 0xED, 0x47, 0xB1, 0x9B
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x00000000, digest: 0x73F47DDB },
        SeedDigest { seed: 0x00000001, digest: 0x606A9284 },
        SeedDigest { seed: 0x76543210, digest: 0x4DB07EB7 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_17_bytes() {
    let bytes: [u8; 17] = [
        0x65, 0x99, 0xC3, 0xBF, 0x60, 0x8F, 0x92, 0xE9, 
        0x94, 0xE5, 0x91, 0x1C, 0xED, 0x47, 0xB1, 0x9B,
        0x4E
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x00000000, digest: 0x3F7F8C1E },
        SeedDigest { seed: 0x00000001, digest: 0x8079F6C2 },
        SeedDigest { seed: 0x76543210, digest: 0xA5DB502E },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_89_bytes() {
    let bytes: [u8; 87] = [
        0x8D, 0xD0, 0xA5, 0xD9, 0x62, 0x3F, 0x1D, 0x5F, 
        0xB5, 0xD9, 0xA7, 0x9B, 0x95, 0x43, 0x2F, 0xC9, 
        0x53, 0x75, 0xA9, 0xB4, 0x4B, 0x02, 0x8C, 0xBC, 
        0x4F, 0xC2, 0x63, 0x18, 0x2A, 0x4D, 0x29, 0x3F, 
        0x85, 0xB3, 0xDC, 0x86, 0xBD, 0x62, 0xAC, 0xE4, 
        0x55, 0x80, 0xB2, 0xE3, 0x7C, 0xB6, 0x54, 0x06, 
        0x81, 0xDA, 0x20, 0x71, 0xDD, 0x4C, 0x28, 0x16, 
        0x66, 0x27, 0x4E, 0x4F, 0x3D, 0xDE, 0xA9, 0x28, 
        0x85, 0xA1, 0x14, 0x80, 0x7D, 0xBA, 0x01, 0x77, 
        0xA3, 0xC8, 0xE7, 0x9F, 0xAC, 0x50, 0x79, 0x9D, 
        0x3E, 0xE0, 0xB4, 0x8C, 0xDA, 0x39, 0x4E
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x00000000, digest: 0x2322D5E4 },
        SeedDigest { seed: 0x00000001, digest: 0x19910A57 },
        SeedDigest { seed: 0x76543210, digest: 0x328F95FD },
    ]);
    test_these_cases(&cases);
}

struct TestCase<'a> {
    seed: u32,
    data: &'a [u8],
    digest: u32,
}

struct SeedDigest {
    seed: u32,
    digest: u32,
}

fn new_cases_sharing_data<'a>(data: &'a [u8], items: &[SeedDigest]) -> Vec<TestCase<'a>> {
    items
        .iter()
        .map(|item| TestCase {
            seed: item.seed,
            data,
            digest: item.digest,
        })
        .collect()
}

/// Example output: `"[0x01, 0x02, 0x03]"`.
///
/// Comments about the string length:
/// * the final string length will be `6n` where `n = bytes.len()`
/// * ... illustrated by `1 + 6n - 2 + 1`
/// * the maximum string length during the algorithm is `1 + 6n`
fn format_as_hex_list(bytes: &[u8]) -> String {
    let mut string = String::with_capacity(1 + 6 * bytes.len());
    string.push('[');
    for &byte in bytes.iter() {
        string.push_str(&format!("0x{:02X}, ", byte));
    }
    if !bytes.is_empty() {
        string.truncate(string.len() - 2); // Remove last ", "
    }
    string.push(']');
    string
}

fn test_these_cases(cases: &[TestCase]) {
    for c in cases {
        let actual = digest(c.data, c.seed);
        if actual != c.digest {
            let data_hex = format_as_hex_list(c.data);
            let data_str = c.data.iter().map(|b| *b as char).collect::<String>();
            #[rustfmt::skip]
            panic!(r#"assertion failed:
     seed: 0x{:08X}
     data: {}
     data: b"{}"
   actual: 0x{:08X}
 expected: 0x{:08X}"#,
                c.seed, data_hex, data_str, actual, c.digest
            );
        }
    }
}
