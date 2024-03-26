use super::super::bpmf_chars as bc;
use std::mem::transmute;
// #region ENUMS

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Init {
    NoInit = 0,
    Bo,
    Po,
    Mo,
    Fo,
    De,
    Te,
    Ne,
    Le,
    Ge,
    Ke,
    He,
    Ji,
    Qi,
    Xi,
    Zhi,
    Chi,
    Shi,
    Ri,
    Zi,
    Ci,
    Si,
}

impl TryFrom<u8> for Init {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=21 => unsafe { Ok(transmute(value)) },
            _ => Err("Invalid value for bopomofo initial"),
        }
    }
}

impl From<Init> for char {
    fn from(value: Init) -> Self {
        enum_to_char(value as u32, bc::BEFORE_BO)
    }
}

fn enum_to_char(discriminant: u32, lower_bound: char) -> char {
    if discriminant == 0 {
        '\0'
    } else {
        unsafe { char::from_u32_unchecked(discriminant + lower_bound as u32) }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Med {
    NoMed = 0,
    Yi = 1,
    Wu,
    Yu,
}

impl TryFrom<u8> for Med {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=3 => unsafe { Ok(transmute(value)) },
            _ => Err("Invalid value for bopomofo medial"),
        }
    }
}

impl From<Med> for char {
    fn from(value: Med) -> Self {
        enum_to_char(value as u32, bc::ER)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rime {
    NoRime = 0,
    A,
    O,
    E,
    Eh,
    Ai,
    Ei,
    Ao,
    Ou,
    An,
    En,
    Ang,
    Eng,
    Er,
}
impl TryFrom<u8> for Rime {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=13 => unsafe { Ok(transmute(value)) },
            _ => Err("Invalid value for bopomofo rime"),
        }
    }
}
impl From<Rime> for char {
    fn from(value: Rime) -> Self {
        enum_to_char(value as u32, bc::S)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tone {
    NoTone,
    Level,
    Rise,
    Dip,
    Fall,
    Neut,
}
impl TryFrom<u8> for Tone {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0..=5 => unsafe { Ok(transmute(value)) },
            _ => Err("Invalid value for bopomofo tone"),
        }
    }
}

impl From<Tone> for char {
    fn from(v: Tone) -> char {
        match v as u8 {
            0 => '\0',
            2 => bc::TONE_2,
            3 => bc::TONE_3,
            4 => bc::TONE_4,
            5 => bc::TONE_5,
            _ => bc::TONE_1,
        }
    }
}
//#endregion
