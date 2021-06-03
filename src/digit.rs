// use pekzep_syllable::PekZepSyllable;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Digit {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    N100,
    N10000,
    N10000_0000,
    Neg,
}

impl From<Digit> for char {
    fn from(s: Digit) -> Self {
        match s {
            Digit::N0 => '無',
            Digit::N1 => '一',
            Digit::N2 => '二',
            Digit::N3 => '三',
            Digit::N4 => '四',
            Digit::N5 => '五',
            Digit::N6 => '六',
            Digit::N7 => '七',
            Digit::N8 => '八',
            Digit::N9 => '九',
            Digit::N10 => '十',
            Digit::N100 => '百',
            Digit::N10000 => '万',
            Digit::N10000_0000 => '億',
            Digit::Neg => '下',
        }
    }
}
/*
impl Into<PekZepSyllable> for Digit {
    fn into(self) -> PekZepSyllable {
        match self {
            Digit::N0 => PekZepSyllable::parse("mun1").unwrap(),
            Digit::N1 => PekZepSyllable::parse("et2").unwrap(),
            Digit::N2 => PekZepSyllable::parse("ik2").unwrap(),
            Digit::N3 => PekZepSyllable::parse("om2 ").unwrap(),
            Digit::N4 => PekZepSyllable::parse("ap1").unwrap(),
            Digit::N5 => PekZepSyllable::parse("un1").unwrap(),
            Digit::N6 => PekZepSyllable::parse("net2").unwrap(),
            Digit::N7 => PekZepSyllable::parse("nik2").unwrap(),
            Digit::N8 => PekZepSyllable::parse("nom2").unwrap(),
            Digit::N9 => PekZepSyllable::parse("nap1").unwrap(),
            Digit::N10 => PekZepSyllable::parse("nun1").unwrap(),
            Digit::N100 => PekZepSyllable::parse("kit1").unwrap(),
            Digit::N10000 => PekZepSyllable::parse("ue1").unwrap(),
            Digit::N10000_0000 => panic!(),
            Digit::Neg => PekZepSyllable::parse("ut2").unwrap(),
        }
    }
}*/

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VecDigits(pub Vec<Digit>);

impl VecDigits {
    pub fn push(&mut self, value: Digit) {
        self.0.push(value)
    }

    pub fn extend(&mut self, v: &Self) {
        self.0.extend(&v.0)
    }
}

impl From<VecDigits> for String {
    fn from(v: VecDigits) -> Self {
        let chars: Vec<char> = v.into();
        chars.iter().collect()
    }
}
impl From<VecDigits> for Vec<char> {
    fn from(v: VecDigits) -> Self {
        v.0.into_iter().map(|a| a.into()).collect::<Self>()
    }
}
