use crate::base::calc_hash_bytes;

use speculoos::assert_that;

#[test]
fn default_hashing() {
    const PASSWORD: &str = "noria18tnvnwkklyv4dyuj8x357n7vray4v4zugmp3du";
    const SALT: &str = "16898739935670952395686488112";

    const HASH_BYTES: &[u8; 32] = &[
        91, 152, 14, 8, 228, 11, 6, 114, 224, 66, 251, 165, 22, 227, 208, 185, 188, 1, 12, 116,
        150, 6, 93, 132, 111, 47, 46, 115, 114, 114, 224, 238,
    ];

    let hash = calc_hash_bytes(PASSWORD, SALT).unwrap();

    assert_that(&hash).is_equal_to(HASH_BYTES);
}
