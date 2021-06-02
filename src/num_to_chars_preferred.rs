#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Digit {
    num00,
    num01,
    num02,
    num03,
    num04,
    num05,
    num06,
    num07,
    num08,
    num09,
    num10,
    neg,
    num100,
}

pub fn toDigits(num: i64) -> Vec<Digit> {
    use Digit::*;
    if (num >= 200) {
        let lastHundredArr: Vec<Digit> = if num % 100 == 0 {
            vec![]
        } else {
            toDigitsSub(num % 100)
        };

        let mut ans = toDigitsSub((num / 100));
        ans.push(num100);
        ans.extend(&lastHundredArr);
        return ans;
    } else if (num >= 100) {
        let lastHundredArr: Vec<Digit> = if num % 100 == 0 {
            vec![]
        } else {
            toDigits(num % 100)
        };
        let mut ans = vec![num100];
        ans.extend(&lastHundredArr);
        return ans;
    } else if (num < 0) {
        let mut ans = vec![neg];
        ans.extend(&toDigits(-num));
        return ans;
    } else if (num == 0) {
        return vec![num00];
    }

    let lastDigitArr: Vec<Digit> = match num % 10 {
        1 => vec![num01],
        2 => vec![num02],
        3 => vec![num03],
        4 => vec![num04],
        5 => vec![num05],
        6 => vec![num06],
        7 => vec![num07],
        8 => vec![num08],
        9 => vec![num09],
        _ => vec![],
    };
    if (num >= 20) {
        let mut ans = match num / 10 {
            2 => vec![num02],
            3 => vec![num03],
            4 => vec![num04],
            5 => vec![num05],
            6 => vec![num06],
            7 => vec![num07],
            8 => vec![num08],
            9 => vec![num09],
            _ => vec![],
        };
        ans.push(num10);
        ans.extend(&lastDigitArr);
        return ans;
    } else if (num >= 10) {
        let mut ans = vec![num10];
        ans.extend(&lastDigitArr);
        return ans;
    } else {
        return lastDigitArr;
    }
}

// -6848 should be 下六八百四八, not 下六十八百四十八. This function thus converts 68 to 六八, not 六十八.
fn toDigitsSub(num: i64) -> Vec<Digit> {
    use Digit::*;
    assert!(num < 100);
    assert!(num > 0);

    if num % 10 == 0 {
        match num {
            10 => vec![num10],
            20 => vec![num02, num10],
            30 => vec![num03, num10],
            40 => vec![num04, num10],
            50 => vec![num05, num10],
            60 => vec![num06, num10],
            70 => vec![num07, num10],
            80 => vec![num08, num10],
            90 => vec![num09, num10],
            _ => panic!(),
        }
    } else {
        let lastDigitArr: Vec<Digit> = match num % 10 {
            1 => vec![num01],
            2 => vec![num02],
            3 => vec![num03],
            4 => vec![num04],
            5 => vec![num05],
            6 => vec![num06],
            7 => vec![num07],
            8 => vec![num08],
            9 => vec![num09],
            _ => vec![],
        };

        if (num >= 20) {
            let mut ans = match num / 10 {
                2 => vec![num02],
                3 => vec![num03],
                4 => vec![num04],
                5 => vec![num05],
                6 => vec![num06],
                7 => vec![num07],
                8 => vec![num08],
                9 => vec![num09],
                _ => vec![],
            };
            ans.extend(&lastDigitArr);
            return ans;
        } else if (num >= 10) {
            let mut ans = vec![num10];
            ans.extend(&lastDigitArr);
            return ans;
        } else {
            return lastDigitArr;
        }
    }
}
