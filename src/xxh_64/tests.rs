use super::digest;

#[test]
#[rustfmt::skip]
fn test_empty_data() {
    let data = b"";
    assert_eq!(data, &[]);
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x0000000000000000, digest: 0xEF46DB3751D8E999 },
        SeedDigest { seed: 0x0000000000000001, digest: 0xD5AFBA1336A3BE4B },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x8CD47B39D95BC70C },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_1_byte() {
    let data = b"@";
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x0000000000000000, digest: 0xD059EB88AA915C20 },
        SeedDigest { seed: 0x0000000000000001, digest: 0xCDAC3A47F942F298 },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x2E428E07434A72E5 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_2_bytes() {
    let data = &[0xF2, 0x81];
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x0000000000000000, digest: 0x2D94F0C9A5B900DD },
        SeedDigest { seed: 0x0000000000000001, digest: 0x4C7AFCA30F755472 },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x20383C7D64DC48DC },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_6_bytes() {
    let data = b"xxHash";
    let cases = new_cases_sharing_data(data, &[
        SeedDigest { seed: 0x0000000000000000, digest: 0x4ACCF427F5F217FB },
        SeedDigest { seed: 0x0000000000000001, digest: 0xD7CB4FE8CF0FF9DD },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x2104F5A58CB6CD99 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_32_bytes() {
    let bytes: [u8; 32] = [
        0xEB, 0xBB, 0x08, 0x79, 0x21, 0xA6, 0xC3, 0x1D, 
        0x17, 0x99, 0x5E, 0xEF, 0xED, 0x43, 0x73, 0x14, 
        0x3A, 0x0F, 0x36, 0xC7, 0x8F, 0x68, 0xBB, 0xD9, 
        0xBF, 0x0C, 0x05, 0x49, 0xC3, 0x60, 0xF8, 0xA8
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x0000000000000000, digest: 0xCD866E99CB636DC7 },
        SeedDigest { seed: 0x0000000000000001, digest: 0x790BA36B6C7CA675 },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x9095693F5B01D9F8 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_33_bytes() {
    let bytes: [u8; 33] = [
        0xEB, 0xBB, 0x08, 0x79, 0x21, 0xA6, 0xC3, 0x1D, 
        0x17, 0x99, 0x5E, 0xEF, 0xED, 0x43, 0x73, 0x14, 
        0x3A, 0x0F, 0x36, 0xC7, 0x8F, 0x68, 0xBB, 0xD9, 
        0xBF, 0x0C, 0x05, 0x49, 0xC3, 0x60, 0xF8, 0xA8,
        0x67
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x0000000000000000, digest: 0x6ECDF52281C45019 },
        SeedDigest { seed: 0x0000000000000001, digest: 0xCA487855D729B16B },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x8CD2F37EFDDB1AB9 },
    ]);
    test_these_cases(&cases);
}

#[test]
#[rustfmt::skip]
fn test_93_bytes() {
    let bytes: [u8; 93] = [
        0x2E, 0x53, 0x85, 0x42, 0xDE, 0x00, 0x3A, 0x8F, 
        0x1B, 0xCB, 0xE4, 0x5B, 0x46, 0x55, 0x72, 0x2A, 
        0xBF, 0x15, 0x62, 0x27, 0xAD, 0xF4, 0x9F, 0x78, 
        0x02, 0xFF, 0xF9, 0xDB, 0xBC, 0x08, 0x75, 0x37, 
        0x66, 0x46, 0xEB, 0x8B, 0x8B, 0xE7, 0xEC, 0x14, 
        0x37, 0x58, 0x17, 0xC7, 0xA6, 0x9C, 0xB3, 0x41, 
        0x94, 0xEB, 0xAB, 0x13, 0x7C, 0x26, 0x43, 0x3B, 
        0x64, 0xFA, 0x6C, 0xD8, 0x96, 0xF4, 0x35, 0x97, 
        0x84, 0x8D, 0x72, 0x50, 0x42, 0x64, 0x9A, 0x14, 
        0xB8, 0x1F, 0xCB, 0xF5, 0xF5, 0xF6, 0xCB, 0xB0, 
        0x52, 0xEE, 0x47, 0xE4, 0x55, 0x14, 0xA5, 0x3D, 
        0x65, 0xFA, 0x29, 0x8D, 0xC2
    ];
    let cases = new_cases_sharing_data(&bytes,&[
        SeedDigest { seed: 0x0000000000000000, digest: 0xFA76A15598999F2A },
        SeedDigest { seed: 0x0000000000000001, digest: 0x5B27C6DD2155BA76 },
        SeedDigest { seed: 0xE2220000F1110000, digest: 0x6CFA844CBAD15137 },
    ]);
    test_these_cases(&cases);
}

struct TestCase<'a> {
    seed: u64,
    data: &'a [u8],
    digest: u64,
}

struct SeedDigest {
    seed: u64,
    digest: u64,
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
