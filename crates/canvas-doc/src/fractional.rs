const DIGITS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";

fn digit_index(byte: u8) -> Option<usize> {
    DIGITS.iter().position(|d| *d == byte)
}

pub fn key_between(lower: Option<&str>, upper: Option<&str>) -> Option<String> {
    midpoint(lower.unwrap_or(""), upper)
}

pub fn first_key() -> String {
    key_between(None, None).unwrap_or_else(|| "i".to_string())
}

fn midpoint(lower: &str, upper: Option<&str>) -> Option<String> {
    if lower.ends_with('0') {
        return None;
    }
    if let Some(u) = upper {
        if u.ends_with('0') || lower >= u {
            return None;
        }
    }

    let lower_bytes = lower.as_bytes();

    if let Some(u) = upper {
        let upper_bytes = u.as_bytes();
        let mut shared = 0;
        while shared < upper_bytes.len()
            && lower_bytes.get(shared).copied().unwrap_or(b'0') == upper_bytes[shared]
        {
            shared += 1;
        }
        if shared > 0 {
            let lower_tail = if shared < lower_bytes.len() {
                &lower[shared..]
            } else {
                ""
            };
            let rest = midpoint(lower_tail, Some(&u[shared..]))?;
            return Some(format!("{}{}", &u[..shared], rest));
        }
    }

    let digit_a = match lower_bytes.first() {
        Some(byte) => digit_index(*byte)?,
        None => 0,
    };
    let digit_b = match upper {
        Some(u) => digit_index(*u.as_bytes().first()?)?,
        None => DIGITS.len(),
    };
    if digit_b <= digit_a {
        return None;
    }

    if digit_b - digit_a > 1 {
        let mid = digit_a + (digit_b - digit_a).div_ceil(2);
        return Some((DIGITS[mid] as char).to_string());
    }

    match upper {
        Some(u) if u.len() > 1 => Some(u[..1].to_string()),
        _ => {
            let tail = if lower_bytes.is_empty() {
                ""
            } else {
                &lower[1..]
            };
            let rest = midpoint(tail, None)?;
            Some(format!("{}{}", DIGITS[digit_a] as char, rest))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_key_sits_in_the_middle_of_the_space() {
        let first = first_key();
        assert!(first.as_str() > "0");
        assert!(first.as_str() < "z");
        assert!(key_between(None, Some(&first)).is_some());
        assert!(key_between(Some(&first), None).is_some());
    }

    #[test]
    fn appending_always_grows() {
        let mut keys = vec![first_key()];
        for _ in 0..500 {
            let next = key_between(keys.last().map(String::as_str), None).unwrap();
            assert!(*keys.last().unwrap() < next);
            keys.push(next);
        }
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }

    #[test]
    fn prepending_always_shrinks() {
        let mut keys = vec![first_key()];
        for _ in 0..500 {
            let previous = key_between(None, keys.first().map(String::as_str)).unwrap();
            assert!(previous < *keys.first().unwrap());
            keys.insert(0, previous);
        }
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }

    #[test]
    fn repeatedly_splitting_the_same_gap_stays_ordered() {
        let low = first_key();
        let high = key_between(Some(&low), None).unwrap();

        let mut upper = high.clone();
        for _ in 0..500 {
            let mid = key_between(Some(&low), Some(&upper)).unwrap();
            assert!(low < mid, "{low} < {mid} failed");
            assert!(mid < upper, "{mid} < {upper} failed");
            upper = mid;
        }
    }

    #[test]
    fn a_key_never_ends_in_a_zero() {
        let mut key = first_key();
        for _ in 0..200 {
            assert!(!key.ends_with('0'), "{key} ends in a zero");
            key = key_between(None, Some(&key)).unwrap();
        }
    }

    #[test]
    fn an_inverted_range_is_refused_rather_than_returning_a_bad_key() {
        let low = first_key();
        let high = key_between(Some(&low), None).unwrap();
        assert!(key_between(Some(&high), Some(&low)).is_none());
        assert!(key_between(Some(&low), Some(&low)).is_none());
    }

    #[test]
    fn a_key_with_a_trailing_zero_is_refused() {
        assert!(key_between(Some("a0"), None).is_none());
        assert!(key_between(None, Some("a0")).is_none());
    }

    #[test]
    fn interleaved_inserts_from_two_sides_stay_ordered() {
        let mut keys: Vec<String> = vec![first_key()];
        for step in 0..200 {
            let index = if step % 3 == 0 { 0 } else { keys.len() / 2 };
            let lower = if index == 0 {
                None
            } else {
                Some(keys[index - 1].as_str())
            };
            let upper = keys.get(index).map(String::as_str);
            let key = key_between(lower, upper).unwrap();
            keys.insert(index, key);
        }
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}
