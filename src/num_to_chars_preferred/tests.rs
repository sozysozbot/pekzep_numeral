#[test]
fn less_than_10000() {
    use crate::num_to_chars_preferred::*;
    const TEST_CASES: [(i64, &str); 11] = [
        (1, "一"),
        (10, "十"),
        (11, "十一"),
        (23, "二十三"),
        (120, "百二十"),
        (123, "百二十三"), /* In this position, non-elision is preferred */
        (234, "二百三四"),
        (1234, "十二百三四"),
        (1230, "十二百三十"),
        (1210, "十二百十"),
        (-6848, "下六八百四八"),
    ];

    for (num, s) in TEST_CASES.iter() {
        let s2: String = pos_neg_zero(less_than_10000, *num).into();
        assert_eq!(&s2, *s)
    }
}
