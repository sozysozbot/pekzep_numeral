mod tests;

/// Example:
/// ```
/// use pekzep_numeral::num_to_chars_preferred::*;
/// assert_eq!(pos_neg_zero(less_than_10000, 234), vec!['二', '百', '三', '四']);
/// assert_eq!(pos_neg_zero(less_than_10000, -6848), vec!['下', '六', '八', '百', '四', '八']);
/// assert_ne!(pos_neg_zero(less_than_10000, -6848), vec!['下', '六', '十', '八', '百', '四', '十', '八']);
/// ```
pub fn pos_neg_zero<T>(f: T, num: i64) -> Vec<char>
where
    T: Fn(i64) -> Vec<char>,
{
    use std::cmp::Ordering;
    match num.cmp(&0) {
        Ordering::Greater => f(num),
        Ordering::Equal => vec!['無'],
        Ordering::Less => {
            let mut ans = vec!['下'];
            ans.extend(&f(-num));
            ans
        }
    }
}

/// Example:
/// ```
/// use pekzep_numeral::num_to_chars_preferred::*;
/// assert_eq!(less_than_10000(6848), vec!['六', '八', '百', '四', '八']);
/// ```
pub fn less_than_10000(num: i64) -> Vec<char> {
    assert!(num < 10000);
    assert!(num > 0);
    if num >= 200 {
        let mut ans = less_than_100_nun1_elided(num / 100);
        ans.push('百');
        ans.extend(&if num % 100 == 0 {
            vec![]
        } else {
            less_than_100_nun1_elided(num % 100)
        });
        return ans;
    } else if num >= 100 {
        let mut ans = vec!['百'];
        ans.extend(&if num % 100 == 0 {
            vec![]
        } else {
            less_than_10000(num % 100)
        });
        return ans;
    }
    less_than_100(num)
}

pub fn less_than_10(num: i64) -> Vec<char> {
    match num {
        1 => vec!['一'],
        2 => vec!['二'],
        3 => vec!['三'],
        4 => vec!['四'],
        5 => vec!['五'],
        6 => vec!['六'],
        7 => vec!['七'],
        8 => vec!['八'],
        9 => vec!['九'],
        _ => panic!(),
    }
}

pub fn less_than_100(num: i64) -> Vec<char> {
    assert!(num < 100);
    assert!(num > 0);

    let last_digit_arr: Vec<char> = match num % 10 {
        0 => vec![],
        a => less_than_10(a),
    };
    if num >= 10 {
        let mut ans = match num / 10 {
            1 => vec![],
            a => less_than_10(a),
        };
        ans.push('十');
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
/// assert_eq!(less_than_100_nun1_elided(3), vec!['三']);
/// assert_eq!(less_than_100_nun1_elided(10), vec!['十']);
/// assert_eq!(less_than_100_nun1_elided(12), vec!['十', '二']);
/// assert_eq!(less_than_100_nun1_elided(60), vec!['六', '十']);
/// assert_eq!(less_than_100_nun1_elided(68), vec!['六', '八']);
/// ```
pub fn less_than_100_nun1_elided(num: i64) -> Vec<char> {
    assert!(num < 100);
    assert!(num > 0);

    if num % 10 == 0 {
        match num / 10 {
            1 => vec!['十'],
            2 => vec!['二', '十'],
            3 => vec!['三', '十'],
            4 => vec!['四', '十'],
            5 => vec!['五', '十'],
            6 => vec!['六', '十'],
            7 => vec!['七', '十'],
            8 => vec!['八', '十'],
            9 => vec!['九', '十'],
            _ => panic!(),
        }
    } else {
        let last_digit_arr: Vec<char> = less_than_10(num % 10);
        if num >= 10 {
            let mut ans = match num / 10 {
                1 => vec!['十'],
                a => less_than_10(a),
            };
            ans.extend(&last_digit_arr);
            ans
        } else {
            last_digit_arr
        }
    }
}
