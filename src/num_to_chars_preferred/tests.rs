#[test]
fn less_than_10000() {
    use crate::num_to_chars_preferred::{less_than_10000, pos_neg_zero};
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

    for (num, s) in &TEST_CASES {
        let s2: String = pos_neg_zero(less_than_10000, *num).into();
        assert_eq!(&s2, *s);
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn less_than_10000_cerke_online_img_path() {
    use crate::digit::Digit;
    use crate::num_to_chars_preferred::{less_than_10000, pos_neg_zero};
    let test_cases = [
        (-10, vec!["neg", "num10"]),
        (0, vec!["num00"]),
        (15, vec!["num10", "num05"]),
        (109, vec!["num100", "num09"]),
        (130, vec!["num100", "num03", "num10"]),
        (191, vec!["num100", "num09", "num10", "num01"]),
        (345, vec!["num03", "num100", "num04", "num05"]),
        (359, vec!["num03", "num100", "num05", "num09"]),
        (773, vec!["num07", "num100", "num07", "num03"]),
        (967, vec!["num09", "num100", "num06", "num07"]),
        (6, vec!["num06"]),
        (19, vec!["num10", "num09"]),
        (22, vec!["num02", "num10", "num02"]),
        (23, vec!["num02", "num10", "num03"]),
        (31, vec!["num03", "num10", "num01"]),
        (41, vec!["num04", "num10", "num01"]),
        (52, vec!["num05", "num10", "num02"]),
        (56, vec!["num05", "num10", "num06"]),
        (60, vec!["num06", "num10"]),
        (74, vec!["num07", "num10", "num04"]),
        (101, vec!["num100", "num01"]),
        (106, vec!["num100", "num06"]),
        (134, vec!["num100", "num03", "num10", "num04"]),
        (180, vec!["num100", "num08", "num10"]),
        (189, vec!["num100", "num08", "num10", "num09"]),
        (209, vec!["num02", "num100", "num09"]),
        (214, vec!["num02", "num100", "num10", "num04"]),
        (225, vec!["num02", "num100", "num02", "num05"]),
        (228, vec!["num02", "num100", "num02", "num08"]),
        (235, vec!["num02", "num100", "num03", "num05"]),
        (243, vec!["num02", "num100", "num04", "num03"]),
        (257, vec!["num02", "num100", "num05", "num07"]),
        (273, vec!["num02", "num100", "num07", "num03"]),
        (280, vec!["num02", "num100", "num08", "num10"]),
        (284, vec!["num02", "num100", "num08", "num04"]),
        (300, vec!["num03", "num100"]),
        (314, vec!["num03", "num100", "num10", "num04"]),
        (347, vec!["num03", "num100", "num04", "num07"]),
        (360, vec!["num03", "num100", "num06", "num10"]),
        (377, vec!["num03", "num100", "num07", "num07"]),
        (382, vec!["num03", "num100", "num08", "num02"]),
        (398, vec!["num03", "num100", "num09", "num08"]),
        (412, vec!["num04", "num100", "num10", "num02"]),
        (418, vec!["num04", "num100", "num10", "num08"]),
        (440, vec!["num04", "num100", "num04", "num10"]),
        (469, vec!["num04", "num100", "num06", "num09"]),
        (471, vec!["num04", "num100", "num07", "num01"]),
        (478, vec!["num04", "num100", "num07", "num08"]),
        (488, vec!["num04", "num100", "num08", "num08"]),
        (507, vec!["num05", "num100", "num07"]),
        (509, vec!["num05", "num100", "num09"]),
        (519, vec!["num05", "num100", "num10", "num09"]),
        (521, vec!["num05", "num100", "num02", "num01"]),
        (523, vec!["num05", "num100", "num02", "num03"]),
        (524, vec!["num05", "num100", "num02", "num04"]),
        (532, vec!["num05", "num100", "num03", "num02"]),
        (555, vec!["num05", "num100", "num05", "num05"]),
        (567, vec!["num05", "num100", "num06", "num07"]),
        (580, vec!["num05", "num100", "num08", "num10"]),
        (594, vec!["num05", "num100", "num09", "num04"]),
        (604, vec!["num06", "num100", "num04"]),
        (618, vec!["num06", "num100", "num10", "num08"]),
        (625, vec!["num06", "num100", "num02", "num05"]),
        (644, vec!["num06", "num100", "num04", "num04"]),
        (649, vec!["num06", "num100", "num04", "num09"]),
        (654, vec!["num06", "num100", "num05", "num04"]),
        (663, vec!["num06", "num100", "num06", "num03"]),
        (671, vec!["num06", "num100", "num07", "num01"]),
        (680, vec!["num06", "num100", "num08", "num10"]),
        (700, vec!["num07", "num100"]),
        (721, vec!["num07", "num100", "num02", "num01"]),
        (727, vec!["num07", "num100", "num02", "num07"]),
        (742, vec!["num07", "num100", "num04", "num02"]),
        (769, vec!["num07", "num100", "num06", "num09"]),
        (772, vec!["num07", "num100", "num07", "num02"]),
        (802, vec!["num08", "num100", "num02"]),
        (810, vec!["num08", "num100", "num10"]),
        (825, vec!["num08", "num100", "num02", "num05"]),
        (840, vec!["num08", "num100", "num04", "num10"]),
        (841, vec!["num08", "num100", "num04", "num01"]),
        (865, vec!["num08", "num100", "num06", "num05"]),
        (866, vec!["num08", "num100", "num06", "num06"]),
        (873, vec!["num08", "num100", "num07", "num03"]),
        (877, vec!["num08", "num100", "num07", "num07"]),
        (880, vec!["num08", "num100", "num08", "num10"]),
        (885, vec!["num08", "num100", "num08", "num05"]),
        (889, vec!["num08", "num100", "num08", "num09"]),
        (899, vec!["num08", "num100", "num09", "num09"]),
        (903, vec!["num09", "num100", "num03"]),
        (935, vec!["num09", "num100", "num03", "num05"]),
        (937, vec!["num09", "num100", "num03", "num07"]),
        (943, vec!["num09", "num100", "num04", "num03"]),
        (953, vec!["num09", "num100", "num05", "num03"]),
        (965, vec!["num09", "num100", "num06", "num05"]),
        (972, vec!["num09", "num100", "num07", "num02"]),
        (976, vec!["num09", "num100", "num07", "num06"]),
        (977, vec!["num09", "num100", "num07", "num07"]),
        (995, vec!["num09", "num100", "num09", "num05"]),
        (1087, vec!["num10", "num100", "num08", "num07"]),
        (1116, vec!["num10", "num01", "num100", "num10", "num06"]),
        (1139, vec!["num10", "num01", "num100", "num03", "num09"]),
        (1194, vec!["num10", "num01", "num100", "num09", "num04"]),
        (1255, vec!["num10", "num02", "num100", "num05", "num05"]),
        (1263, vec!["num10", "num02", "num100", "num06", "num03"]),
        (1287, vec!["num10", "num02", "num100", "num08", "num07"]),
        (1343, vec!["num10", "num03", "num100", "num04", "num03"]),
        (1424, vec!["num10", "num04", "num100", "num02", "num04"]),
        (1686, vec!["num10", "num06", "num100", "num08", "num06"]),
        (1695, vec!["num10", "num06", "num100", "num09", "num05"]),
        (1712, vec!["num10", "num07", "num100", "num10", "num02"]),
        (1851, vec!["num10", "num08", "num100", "num05", "num01"]),
        (1917, vec!["num10", "num09", "num100", "num10", "num07"]),
        (1967, vec!["num10", "num09", "num100", "num06", "num07"]),
        (2015, vec!["num02", "num10", "num100", "num10", "num05"]),
        (2275, vec!["num02", "num02", "num100", "num07", "num05"]),
        (2392, vec!["num02", "num03", "num100", "num09", "num02"]),
        (2515, vec!["num02", "num05", "num100", "num10", "num05"]),
        (2746, vec!["num02", "num07", "num100", "num04", "num06"]),
        (2751, vec!["num02", "num07", "num100", "num05", "num01"]),
        (2768, vec!["num02", "num07", "num100", "num06", "num08"]),
        (2909, vec!["num02", "num09", "num100", "num09"]),
        (2953, vec!["num02", "num09", "num100", "num05", "num03"]),
        (3057, vec!["num03", "num10", "num100", "num05", "num07"]),
        (3141, vec!["num03", "num01", "num100", "num04", "num01"]),
        (3270, vec!["num03", "num02", "num100", "num07", "num10"]),
        (3321, vec!["num03", "num03", "num100", "num02", "num01"]),
        (3430, vec!["num03", "num04", "num100", "num03", "num10"]),
        (3546, vec!["num03", "num05", "num100", "num04", "num06"]),
        (3662, vec!["num03", "num06", "num100", "num06", "num02"]),
        (3693, vec!["num03", "num06", "num100", "num09", "num03"]),
        (3707, vec!["num03", "num07", "num100", "num07"]),
        (3763, vec!["num03", "num07", "num100", "num06", "num03"]),
        (4145, vec!["num04", "num01", "num100", "num04", "num05"]),
        (4261, vec!["num04", "num02", "num100", "num06", "num01"]),
        (4483, vec!["num04", "num04", "num100", "num08", "num03"]),
        (4533, vec!["num04", "num05", "num100", "num03", "num03"]),
        (4819, vec!["num04", "num08", "num100", "num10", "num09"]),
        (4884, vec!["num04", "num08", "num100", "num08", "num04"]),
        (4895, vec!["num04", "num08", "num100", "num09", "num05"]),
        (5026, vec!["num05", "num10", "num100", "num02", "num06"]),
        (5067, vec!["num05", "num10", "num100", "num06", "num07"]),
        (5135, vec!["num05", "num01", "num100", "num03", "num05"]),
        (5547, vec!["num05", "num05", "num100", "num04", "num07"]),
        (5813, vec!["num05", "num08", "num100", "num10", "num03"]),
        (5830, vec!["num05", "num08", "num100", "num03", "num10"]),
        (5934, vec!["num05", "num09", "num100", "num03", "num04"]),
        (6058, vec!["num06", "num10", "num100", "num05", "num08"]),
        (6161, vec!["num06", "num01", "num100", "num06", "num01"]),
        (6167, vec!["num06", "num01", "num100", "num06", "num07"]),
        (6313, vec!["num06", "num03", "num100", "num10", "num03"]),
        (6365, vec!["num06", "num03", "num100", "num06", "num05"]),
        (6867, vec!["num06", "num08", "num100", "num06", "num07"]),
        (6874, vec!["num06", "num08", "num100", "num07", "num04"]),
        (6910, vec!["num06", "num09", "num100", "num10"]),
        (7051, vec!["num07", "num10", "num100", "num05", "num01"]),
        (7169, vec!["num07", "num01", "num100", "num06", "num09"]),
        (7182, vec!["num07", "num01", "num100", "num08", "num02"]),
        (7249, vec!["num07", "num02", "num100", "num04", "num09"]),
        (7470, vec!["num07", "num04", "num100", "num07", "num10"]),
        (7490, vec!["num07", "num04", "num100", "num09", "num10"]),
        (7545, vec!["num07", "num05", "num100", "num04", "num05"]),
        (7661, vec!["num07", "num06", "num100", "num06", "num01"]),
        (7677, vec!["num07", "num06", "num100", "num07", "num07"]),
        (7739, vec!["num07", "num07", "num100", "num03", "num09"]),
        (8035, vec!["num08", "num10", "num100", "num03", "num05"]),
        (8204, vec!["num08", "num02", "num100", "num04"]),
        (8309, vec!["num08", "num03", "num100", "num09"]),
        (8337, vec!["num08", "num03", "num100", "num03", "num07"]),
        (8368, vec!["num08", "num03", "num100", "num06", "num08"]),
        (8655, vec!["num08", "num06", "num100", "num05", "num05"]),
        (8659, vec!["num08", "num06", "num100", "num05", "num09"]),
        (8688, vec!["num08", "num06", "num100", "num08", "num08"]),
        (8813, vec!["num08", "num08", "num100", "num10", "num03"]),
        (8902, vec!["num08", "num09", "num100", "num02"]),
        (8922, vec!["num08", "num09", "num100", "num02", "num02"]),
        (9075, vec!["num09", "num10", "num100", "num07", "num05"]),
        (9243, vec!["num09", "num02", "num100", "num04", "num03"]),
        (9259, vec!["num09", "num02", "num100", "num05", "num09"]),
        (9265, vec!["num09", "num02", "num100", "num06", "num05"]),
        (9336, vec!["num09", "num03", "num100", "num03", "num06"]),
        (9390, vec!["num09", "num03", "num100", "num09", "num10"]),
        (9561, vec!["num09", "num05", "num100", "num06", "num01"]),
        (9743, vec!["num09", "num07", "num100", "num04", "num03"]),
        (9942, vec!["num09", "num09", "num100", "num04", "num02"]),
        (9996, vec!["num09", "num09", "num100", "num09", "num06"]),
    ];

    for (num, s) in &test_cases {
        let s2: Vec<Digit> = pos_neg_zero(less_than_10000, *num).0;
        assert_eq!(
            s2.iter()
                .map(|d| match *d {
                    Digit::N0 => "num00",
                    Digit::N1 => "num01",
                    Digit::N2 => "num02",
                    Digit::N3 => "num03",
                    Digit::N4 => "num04",
                    Digit::N5 => "num05",
                    Digit::N6 => "num06",
                    Digit::N7 => "num07",
                    Digit::N8 => "num08",
                    Digit::N9 => "num09",
                    Digit::N10 => "num10",
                    Digit::N100 => "num100",
                    Digit::N10000 => "num10000",
                    Digit::N10000_0000 => "num10000_0000",
                    Digit::Neg => "neg",
                })
                .collect::<Vec<_>>(),
            *s
        );
    }
}
