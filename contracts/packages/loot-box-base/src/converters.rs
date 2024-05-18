use std::str::FromStr;

use cosmwasm_std::{Addr, Decimal, Decimal256, StdError, StdResult, Uint128, Uint256};
use hashing_helper::base::ENC_KEY_LEN;

pub fn str_to_dec(s: &str) -> Decimal {
    Decimal::from_str(s).unwrap()
}

pub fn str_to_dec256(s: &str) -> Decimal256 {
    Decimal256::from_str(s).unwrap()
}

pub fn u128_to_dec<T>(num: T) -> Decimal
where
    Uint128: From<T>,
{
    Decimal::from_ratio(Uint128::from(num), Uint128::one())
}

pub fn u128_to_dec256<T>(num: T) -> Decimal256
where
    Uint128: From<T>,
{
    Decimal256::from_ratio(Uint128::from(num), Uint128::one())
}

pub fn u256_to_uint128(u256: impl Into<Uint256>) -> Uint128 {
    str_to_dec(
        &Into::<Uint256>::into(u256)
            .to_string()
            .chars()
            .take(Decimal::DECIMAL_PLACES as usize)
            .collect::<String>(),
    )
    .to_uint_ceil()
}

pub fn dec_to_dec256(dec: Decimal) -> Decimal256 {
    Decimal256::from_str(&dec.to_string()).unwrap()
}

pub fn dec256_to_dec(dec256: Decimal256) -> Decimal {
    str_to_dec(
        &dec256
            .to_string()
            .chars()
            .take(Decimal::DECIMAL_PLACES as usize)
            .collect::<String>(),
    )
}

pub fn dec256_to_uint128(dec256: Decimal256) -> Uint128 {
    Uint128::try_from(dec256.to_uint_floor()).unwrap()
}

pub fn str_vec_to_dec_vec(str_vec: &[&str]) -> Vec<Decimal> {
    str_vec.iter().map(|&x| str_to_dec(x)).collect()
}

pub fn u128_vec_to_uint128_vec(u128_vec: &[u128]) -> Vec<Uint128> {
    u128_vec
        .iter()
        .map(|&x| Uint128::from(x))
        .collect::<Vec<Uint128>>()
}

/// Converts u8 vector to [u8; ENC_KEY_LEN]
pub fn u8_vec_to_hash_bytes(v: &Vec<u8>) -> StdResult<[u8; ENC_KEY_LEN]> {
    TryInto::try_into(v.to_owned()).map_err(|_| StdError::GenericErr {
        msg: format!("Vector length is {} but expected {}", v.len(), ENC_KEY_LEN),
    })
}

/// Converts any String to u8 vector
pub fn str_to_u8_vec(s: &str) -> Vec<u8> {
    s.chars().map(|c| c as u8).collect()
}

/// Converts any u8 vector to String
pub fn u8_vec_to_str(v: &[u8]) -> String {
    String::from_iter(v.iter().map(|x| *x as char))
}

/// Converts u8 vector to String if all elements are valid UTF-8
pub fn utf8_vec_to_str(v: &[u8]) -> StdResult<String> {
    match std::str::from_utf8(v).map(|x| x.to_string()) {
        Ok(x) => Ok(x),
        Err(e) => Err(StdError::GenericErr { msg: e.to_string() }),
    }
}

/// Converts [u8; ENC_KEY_LEN] to Decimal in range 0..1
pub fn hash_bytes_to_norm_dec(hash: &[u8; 32]) -> Decimal {
    let mut hash_value: u128 = 0;

    for &byte in hash.iter().rev() {
        hash_value = (hash_value << 8) | (byte as u128);
    }

    dec256_to_dec(Decimal256::from_ratio(
        Uint128::from(hash_value),
        Uint128::from(u128::MAX),
    ))
}

pub fn address_to_salt(address: &Addr) -> String {
    // Salt length must be >= 12
    address.to_string().repeat(2)
}
