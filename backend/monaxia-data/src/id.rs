use std::cmp::Ordering;

use thiserror::Error as ThisError;
use uuid::Uuid;

const ORDER58_CHARS: [char; 58] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K',
    'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
    'z',
];

#[derive(Debug, Clone, Copy, ThisError)]
pub enum Order58Error {
    #[error("invalid digit detected")]
    InvalidDigit,
}

pub fn to_order58(uuid: Uuid) -> String {
    let mut rest = uuid.as_u128();
    if rest == 0 {
        return ORDER58_CHARS[0].into();
    }

    // log58(2^128) ~= 21.8...
    let mut buffer = [ORDER58_CHARS[0]; 32];
    let mut cursor = 31;
    while rest > 0 {
        buffer[cursor] = ORDER58_CHARS[(rest % 58) as usize];
        rest /= 58;
        cursor -= 1;
    }

    buffer[(cursor + 1)..].iter().collect()
}

pub fn from_order58(o58: &str) -> Result<Uuid, Order58Error> {
    let mut value = 0u128;
    for c in o58.chars() {
        value *= 58;
        value += ORDER58_CHARS
            .binary_search(&c)
            .map_err(|_| Order58Error::InvalidDigit)? as u128;
    }
    Ok(Uuid::from_u128(value))
}

pub fn now_order58() -> String {
    to_order58(Uuid::now_v7())
}

/// Compares two strings by [Mastodon ID ordering](https://docs.joinmastodon.org/api/guidelines/#id).
pub fn cmp_mstdn_id(lhs: &str, rhs: &str) -> Ordering {
    // compare by length first
    match lhs.len().cmp(&rhs.len()) {
        // if equal length, compare by lexicial order.
        Ordering::Equal => lhs.cmp(rhs),
        otherwise => otherwise,
    }
}

#[cfg(test)]
mod tests {
    use super::{cmp_mstdn_id, from_order58, now_order58, to_order58};

    use std::{cmp::Ordering, thread::sleep, time::Duration};

    use uuid::Uuid;

    #[test]
    fn order58_works() {
        let _ = now_order58();

        let base_uuid = Uuid::now_v7();
        let encoded = to_order58(base_uuid);
        let decoded = from_order58(&encoded);

        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap(), base_uuid);
    }

    #[test]
    fn mstdn_cmp_works() {
        assert_eq!(cmp_mstdn_id("1", "2"), Ordering::Less);
        assert_eq!(cmp_mstdn_id("2", "2"), Ordering::Equal);
        assert_eq!(cmp_mstdn_id("9", "910"), Ordering::Less);
        assert_eq!(cmp_mstdn_id("1234", "123"), Ordering::Greater);
        assert_eq!(cmp_mstdn_id("ABC", "ABD"), Ordering::Less);

        let early_id = now_order58();
        sleep(Duration::from_millis(10));
        let late_id = now_order58();

        assert_eq!(cmp_mstdn_id(&early_id, &late_id), Ordering::Less);
        assert_eq!(cmp_mstdn_id(&late_id, &early_id), Ordering::Greater);
        assert_eq!(cmp_mstdn_id(&early_id, &early_id), Ordering::Equal);
    }
}
