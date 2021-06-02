#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit {
    Num00,
    Num01,
    Num02,
    Num03,
    Num04,
    Num05,
    Num06,
    Num07,
    Num08,
    Num09,
    Num10,
    Neg,
    Num100,
}

pub fn to_digits(num: i64) -> Vec<Digit> {
    use Digit::*;
    if num >= 200 {
        let last_hundred_arr: Vec<Digit> = if num % 100 == 0 {
            vec![]
        } else {
            to_digits_sub(num % 100)
        };

        let mut ans = to_digits_sub(num / 100);
        ans.push(Num100);
        ans.extend(&last_hundred_arr);
        return ans;
    } else if num >= 100 {
        let last_hundred_arr: Vec<Digit> = if num % 100 == 0 {
            vec![]
        } else {
            to_digits(num % 100)
        };
        let mut ans = vec![Num100];
        ans.extend(&last_hundred_arr);
        return ans;
    } else if num < 0 {
        let mut ans = vec![Neg];
        ans.extend(&to_digits(-num));
        return ans;
    } else if num == 0 {
        return vec![Num00];
    }

    let last_digit_arr: Vec<Digit> = match num % 10 {
        1 => vec![Num01],
        2 => vec![Num02],
        3 => vec![Num03],
        4 => vec![Num04],
        5 => vec![Num05],
        6 => vec![Num06],
        7 => vec![Num07],
        8 => vec![Num08],
        9 => vec![Num09],
        _ => vec![],
    };
    if num >= 20 {
        let mut ans = match num / 10 {
            2 => vec![Num02],
            3 => vec![Num03],
            4 => vec![Num04],
            5 => vec![Num05],
            6 => vec![Num06],
            7 => vec![Num07],
            8 => vec![Num08],
            9 => vec![Num09],
            _ => vec![],
        };
        ans.push(Num10);
        ans.extend(&last_digit_arr);
        ans
    } else if num >= 10 {
        let mut ans = vec![Num10];
        ans.extend(&last_digit_arr);
        ans
    } else {
        last_digit_arr
    }
}

// -6848 should be 下六八百四八, not 下六十八百四十八. This function thus converts 68 to 六八, not 六十八.
fn to_digits_sub(num: i64) -> Vec<Digit> {
    use Digit::*;
    assert!(num < 100);
    assert!(num > 0);

    if num % 10 == 0 {
        match num {
            10 => vec![Num10],
            20 => vec![Num02, Num10],
            30 => vec![Num03, Num10],
            40 => vec![Num04, Num10],
            50 => vec![Num05, Num10],
            60 => vec![Num06, Num10],
            70 => vec![Num07, Num10],
            80 => vec![Num08, Num10],
            90 => vec![Num09, Num10],
            _ => panic!(),
        }
    } else {
        let last_digit_arr: Vec<Digit> = match num % 10 {
            1 => vec![Num01],
            2 => vec![Num02],
            3 => vec![Num03],
            4 => vec![Num04],
            5 => vec![Num05],
            6 => vec![Num06],
            7 => vec![Num07],
            8 => vec![Num08],
            9 => vec![Num09],
            _ => vec![],
        };

        if num >= 20 {
            let mut ans = match num / 10 {
                2 => vec![Num02],
                3 => vec![Num03],
                4 => vec![Num04],
                5 => vec![Num05],
                6 => vec![Num06],
                7 => vec![Num07],
                8 => vec![Num08],
                9 => vec![Num09],
                _ => vec![],
            };
            ans.extend(&last_digit_arr);
            ans
        } else if num >= 10 {
            let mut ans = vec![Num10];
            ans.extend(&last_digit_arr);
            ans
        } else {
            last_digit_arr
        }
    }
}
