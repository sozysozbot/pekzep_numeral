mod tests;

pub fn chars_to_num(s: &[char]) -> Option<i64> {
    match s {
        ['無'] => Some(0),
        ['下', tail @ ..] => chars_to_num::positive(tail).map(|a| -a),
        simple => chars_to_num::positive(simple),
    }
}

/// Example:
/// ```
/// use pekzep_numeral::str_to_num;
/// assert_eq!(str_to_num("二一億四七百四八万三六百四七"), Some(21_4748_3647));
/// assert_eq!(str_to_num("二十一億四七百四八万三六百四七"), Some(21_4748_3647));
/// assert_eq!(str_to_num("一二百三四万五六百七八"), Some(1234_5678));
/// assert_eq!(str_to_num("万二三百四五"), Some(12345));
/// assert_eq!(str_to_num("一万二千三百四十五"), None);
/// ```
pub fn str_to_num(s: &str) -> Option<i64> {
    chars_to_num(&s.chars().collect::<Vec<_>>())
}

mod chars_to_num;
