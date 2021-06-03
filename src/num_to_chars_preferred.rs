mod tests;
use crate::digit::Digit::*;
use crate::digit::VecDigits;

/// Example:
/// ```
/// use pekzep_numeral::num_to_chars_preferred::*;
/// assert_eq!(String::from(pos_neg_zero(less_than_10000, 234)), "二百三四");
/// assert_eq!(String::from(pos_neg_zero(less_than_10000, -6848)), "下六八百四八");
/// assert_ne!(String::from(pos_neg_zero(less_than_10000, -6848)), "下六十八百四十八");
/// ```
pub fn pos_neg_zero<T>(f: T, num: i64) -> VecDigits
where
    T: Fn(i64) -> VecDigits,
{
    use std::cmp::Ordering;
    match num.cmp(&0) {
        Ordering::Greater => f(num),
        Ordering::Equal => VecDigits(vec![N0]),
        Ordering::Less => {
            let mut ans = VecDigits(vec![Neg]);
            ans.extend(&f(-num));
            ans
        }
    }
}

/// Example:
/// ```
/// use pekzep_numeral::num_to_chars_preferred::*;
/// assert_eq!(String::from(less_than_10000(6848)), "六八百四八");
/// ```
pub fn less_than_10000(num: i64) -> VecDigits {
    assert!(num < 10000);
    assert!(num > 0);
    if num >= 200 {
        let mut ans = less_than_100_nun1_elided(num / 100);
        ans.push(N100);
        ans.extend(&if num % 100 == 0 {
            VecDigits(vec![])
        } else {
            less_than_100_nun1_elided(num % 100)
        });
        return ans;
    } else if num >= 100 {
        let mut ans = VecDigits(vec![N100]);
        ans.extend(&if num % 100 == 0 {
            VecDigits(vec![])
        } else {
            less_than_10000(num % 100)
        });
        return ans;
    }
    less_than_100(num)
}

pub fn less_than_10(num: i64) -> VecDigits {
    match num {
        1 => VecDigits(vec![N1]),
        2 => VecDigits(vec![N2]),
        3 => VecDigits(vec![N3]),
        4 => VecDigits(vec![N4]),
        5 => VecDigits(vec![N5]),
        6 => VecDigits(vec![N6]),
        7 => VecDigits(vec![N7]),
        8 => VecDigits(vec![N8]),
        9 => VecDigits(vec![N9]),
        _ => panic!(),
    }
}

pub fn less_than_100(num: i64) -> VecDigits {
    assert!(num < 100);
    assert!(num > 0);

    let last_digit_arr: VecDigits = match num % 10 {
        0 => VecDigits(vec![]),
        a => less_than_10(a),
    };
    if num >= 10 {
        let mut ans = match num / 10 {
            1 => VecDigits(vec![]),
            a => less_than_10(a),
        };
        ans.push(N10);
        ans.extend(&last_digit_arr);
        ans
    } else {
        last_digit_arr
    }
}

/// -6848 should be 下六八百四八, not 下六十八百四十八. This function converts 68 to 六八, not 六十八 to account for such subcases.
/// Example:
/// ```
/// use pekzep_numeral::num_to_chars_preferred::*;
/// assert_eq!(String::from(less_than_100_nun1_elided(3)), "三");
/// assert_eq!(String::from(less_than_100_nun1_elided(10)),"十");
/// assert_eq!(String::from(less_than_100_nun1_elided(12)),"十二");
/// assert_eq!(String::from(less_than_100_nun1_elided(60)),"六十");
/// assert_eq!(String::from(less_than_100_nun1_elided(68)),"六八");
/// ```
pub fn less_than_100_nun1_elided(num: i64) -> VecDigits {
    assert!(num < 100);
    assert!(num > 0);

    if num % 10 == 0 {
        match num / 10 {
            1 => VecDigits(vec![N10]),
            2 => VecDigits(vec![N2, N10]),
            3 => VecDigits(vec![N3, N10]),
            4 => VecDigits(vec![N4, N10]),
            5 => VecDigits(vec![N5, N10]),
            6 => VecDigits(vec![N6, N10]),
            7 => VecDigits(vec![N7, N10]),
            8 => VecDigits(vec![N8, N10]),
            9 => VecDigits(vec![N9, N10]),
            _ => panic!(),
        }
    } else {
        let last_digit_arr: VecDigits = less_than_10(num % 10);
        if num >= 10 {
            let mut ans = match num / 10 {
                1 => VecDigits(vec![N10]),
                a => less_than_10(a),
            };
            ans.extend(&last_digit_arr);
            ans
        } else {
            last_digit_arr
        }
    }
}
