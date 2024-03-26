use self::data::{
    get_init_pinyin_to_bopomofo_map, get_pinyin_toned_rime_tree, get_pinyin_untoned_rime_tree,
    PINYIN_NUC_CODAS, PINYIN_TONED_NUCS,
};

use super::bpmf_chars as bc;
use std::cmp::Ordering;
use std::fmt::{Display, Write};

use std::mem::transmute;
use std::str::FromStr;

pub mod data;
pub mod enums;
pub mod spell_tree;

pub use enums::*;

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Syllable {
    init: u8,
    med: u8,
    rime: u8,
    tone: u8,
}

impl Syllable {
    pub fn new(init: Init, med: Med, rime: Rime, tone: Tone) -> Syllable {
        Syllable {
            init: init as u8,
            med: med as u8,
            rime: rime as u8,
            tone: tone as u8,
        }
    }

    //#region PROPS

    pub fn init(&self) -> Init {
        unsafe { transmute(self.init) }
    }
    pub fn med(&self) -> Med {
        unsafe { transmute(self.med) }
    }
    pub fn rime(&self) -> Rime {
        unsafe { transmute(self.rime) }
    }
    pub fn tone(&self) -> Tone {
        unsafe { transmute(self.tone) }
    }
    pub fn init_char(&self) -> char {
        _bpmf_part_to_char(self.init, bc::BEFORE_BO)
    }
    pub fn med_char(&self) -> char {
        _bpmf_part_to_char(self.med, bc::ER)
    }
    pub fn rime_char(&self) -> char {
        _bpmf_part_to_char(self.rime, bc::S)
    }
    pub fn tone_char(&self) -> char {
        match self.tone {
            0 => '\0',
            2 => bc::TONE_2,
            3 => bc::TONE_3,
            4 => bc::TONE_4,
            5 => bc::TONE_5,
            _ => bc::TONE_1,
        }
    }

    pub fn ord(&self) -> u32 {
        self.tone as u32
            + self.rime as u32 * 41
            + self.med as u32 * 41 * 41
            + (if 0 == self.init { 40 } else { self.init as u32 }) * 41 * 41 * 41
    }

    pub fn is_empty(&self) -> bool {
        self.init == 0 && self.med == 0 && self.rime == 0 && self.tone == 0
    }

    pub fn byte_len(&self) -> usize {
        ((self.init > 0) as usize) * 3
            + ((self.med > 0) as usize) * 3
            + ((self.rime > 0) as usize) * 3
            + if self.tone > 1 { 2 } else { 0 }
    }

    //#endregion

    //#region PARSERS
    fn _skip_whitespaces(slice: &str) -> &str {
        let mut chars = slice.char_indices();
        while let Some((idx, ch)) = chars.next() {
            if !ch.is_whitespace() && ch != '\'' {
                return &slice[idx..];
            }
        }
        slice
    }
    /// This function parses the &str from the start. if succeeded it returns the Syllable and
    /// the maining part of the string (minus the consumed part); if failed it returns an error
    pub fn parse_bopomofo(mut slice: &str) -> Result<(Syllable, &str), ParseBopomofoError> {
        let (mut init, mut med, mut rime, mut tone) = (0, 0, 0, 0);
        slice = Self::_skip_whitespaces(slice);
        let mut offset = 0;
        let mut chars = slice.chars();
        let mut next = chars.next();

        //Does 輕聲MARK exist?
        if let Some(bc::TONE_5) = next {
            tone = 5u8;
            offset += 2;
            next = chars.next();
        }

        //Initial 聲母
        if let Some(ch @ 'ㄅ'..='ㄙ') = next {
            init = (ch as u32 - bc::BEFORE_BO as u32) as u8;
            offset += 3;
            next = chars.next();
        }

        //Medial 介音
        if let Some(ch @ 'ㄧ'..='ㄩ') = next {
            med = (ch as u32 - bc::ER as u32) as u8;
            offset += 3;
            next = chars.next();
        }

        //Rime 韻基
        if let Some(ch @ 'ㄚ'..='ㄦ') = next {
            rime = (ch as u32 - bc::S as u32) as u8;
            offset += 3;
            next = chars.next();
        }

        //Ohter tone marks at the end其他聲調
        if tone == 0 {
            tone = match next.and_then(|ch| "ˉˊˇˋ˙".find(ch)) {
                //all those marks are 2 bytes long
                Some(idx) => {
                    offset += 2;
                    (idx / 2 + 1) as u8
                }
                None => 1,
            };
        }

        if med > 0 || rime > 0 || init >= Init::Zhi as u8 && init <= Init::Si as u8 {
            Ok((Syllable { init, med, rime, tone }, &slice[offset..]))
        } else {
            Err(ParseBopomofoError)
        }
    }

    ///I won't fail! if no match is found it returs 0
    fn _parse_pinyin_initial(mut txt: &str) -> (u8, &str) {
        txt = Self::_skip_whitespaces(txt);
        let mut chars = (txt).chars();
        if let Some(first_ch) = chars.next() {
            //try to match first letter by looking up the table
            if let Some(&i) = get_init_pinyin_to_bopomofo_map().get(&first_ch) {
                let init = (i as u32 - bc::BEFORE_BO as u32) as u8;
                return match init {
                    19..=21 if chars.next() == Some('h') => (init - 4, &txt[2..]), //ㄗ~ㄙ -> ㄓ~ㄕ
                    _ => (init, &txt[1..]),
                };
            } else if let 'w' | 'y' = first_ch {
                //w,y are returned as special cases
                return (first_ch as u8, &txt[1..]);
            }
        }
        (0, txt)
    }

    fn _adjust_pinyin_parts(init: &mut u8, med: &mut Med, rime: Rime) {
        let (is_w, is_y) = (*init == b'w', *init == b'y');
        if is_w || is_y {
            *init = 0;
        }
        if *med == Med::Wu && (*init >= Init::Ji as u8 && *init <= Init::Xi as u8 || is_y) {
            //(j|q|x|y)u_ -> (ㄐ|ㄑ|ㄒ|_)ㄩ_
            *med = Med::Yu;
        } else if *med == Med::Yi
            && *init >= Init::Zhi as u8
            && *init <= Init::Si as u8
            && rime == Rime::NoRime
        {
            //zh,ch,sh,r,z,c,s + i -> ㄓㄔㄕㄖㄗㄘㄙ + ㄭ
            *med = Med::NoMed;
        } else if (is_w || is_y) && *med != Med::Yu {
            *med = if is_w { Med::Wu } else { Med::Yi }
        }
    }

    pub fn parse_pinyin(mut txt: &str) -> Result<(Syllable, &str), ParseBopomofoError> {
        let mut init;
        (init, txt) = Self::_parse_pinyin_initial(txt);

        if !txt.is_empty() {
            if let Some(((mut med, rime, tone), remainder)) = get_pinyin_toned_rime_tree().find(txt)
            {
                Self::_adjust_pinyin_parts(&mut init, &mut med, rime);

                return Ok((
                    Syllable {
                        init,
                        med: med as u8,
                        rime: rime as u8,
                        tone,
                    },
                    remainder,
                ));
            }
        }
        Err(ParseBopomofoError)
    }
    pub fn parse_ascii_pinyin(mut txt: &str) -> Result<(Syllable, &str), ParseBopomofoError> {
        let mut init;
        (init, txt) = Self::_parse_pinyin_initial(txt);
        if !txt.is_empty() {
            if let Some(((mut med, rime), mut remainder)) = get_pinyin_untoned_rime_tree().find(txt)
            {
                Self::_adjust_pinyin_parts(&mut init, &mut med, rime);
                let mut chars = remainder.chars();
                let tone = match chars.next() {
                    Some(ch @ '1'..='5') => {
                        remainder = chars.as_str();
                        (ch as u32 - '0' as u32) as u8
                    }
                    _ => 0u8,
                };
                return Ok((
                    Syllable {
                        init,
                        med: med as u8,
                        rime: rime as u8,
                        tone,
                    },
                    remainder,
                ));
            }
        }
        Err(ParseBopomofoError)
    }
    //#endregion

    //#region to_xxx

    pub fn to_pinyin(&self) -> String {
        let mut s = String::with_capacity(7);

        s.push_str(self._pinyin_inital());

        let (med, mut nuc, coda) = self._get_rime_parts();
        if med > '\0' {
            s.push(med);
        }
        nuc = PINYIN_TONED_NUCS
            .iter()
            .find(|(ch, _)| *ch == nuc)
            .map(|(_, letters)| letters.chars().nth(self.tone as usize).unwrap())
            .unwrap();
        s.push(nuc);
        if !coda.is_empty() {
            s.push_str(coda);
        }
        s
    }
    pub fn to_ascii_pinyin(&self) -> String {
        let mut s = String::with_capacity(8);
        s.push_str(self._pinyin_inital());
        let (med, nuc, coda) = self._get_rime_parts();
        if med > '\0' {
            s.push(med)
        }
        s.push(if nuc == 'ü' { 'v' } else { nuc });
        if !coda.is_empty() {
            s.push_str(coda);
        }
        if self.tone > 0 {
            s.push(unsafe { char::from_u32_unchecked('0' as u32 + self.tone as u32) })
        }
        s
    }

    /// It return the pinyin inital corresponding to the bopomofo initial
    /// If the bopomofo syllable has zero-initial and the medial is one of ㄧㄨㄩ,
    /// it returns b'y',b'w',b'y' accordingly
    fn _pinyin_inital(&self) -> &str {
        data::PINYIN_INITIALS[
            // med as inital 21 + 1|2|3, 0inital as ""
            if self.init == 0 && self.med != 0 {
                self.med + Init::Si as u8
            } else {
                self.init
            } as usize// or inital
        ]
    }

    fn _get_rime_parts(&self) -> (char, char, &str) {
        let (init_char, med_char, rime_char) =
            (self.init_char(), self.med_char(), self.rime_char());

        //default char for medials
        let mut med = match med_char {
            'ㄧ' => 'i',
            'ㄨ' => 'u',
            'ㄩ' => 'ü',
            _ => '\0',
        };

        //look the nucleus and the coda up from the table array.
        let (mut nuc, mut coda) = PINYIN_NUC_CODAS[if self.rime == 0 && self.med != 0 {
            self.med + Rime::Er as u8
        } else {
            self.rime
        } as usize];

        if med == nuc {
            //eliminate .?uu_ .?ii_ .?üü_
            med = '\0'
        }

        //Is it zero initial w~ or y~
        let is_w = self.init == 0 && med_char == 'ㄨ';
        let is_y = self.init == 0 && self.med > 0 && !is_w;

        //adjust the parts according to the quirks of pinyin
        //note that the tone marks can be only added on the nucleus
        if !is_w && med_char == 'ㄨ' && rime_char == 'ㄟ' {
            //[^w]uei -> [^w]ui
            (nuc, coda) = ('i', "")
        } else if !is_y && med_char == 'ㄧ' && rime_char == 'ㄡ' {
            //[^y]iou -> [^y]iu
            (nuc, coda) = ('u', "");
        }

        if !is_w && self.med > 0 && (rime_char == 'ㄣ' || rime_char == 'ㄥ') {
            //ien ieng uen ueng üen üeng -> in ing un ong ün iong
            med = '\0';
            match med_char {
                'ㄧ' => {  nuc = 'i' }
                'ㄨ' => { nuc = if rime_char == 'ㄥ' {'o'}else{'u'}}
                _ /*'ㄩ'*/ => {(med, nuc) = if rime_char == 'ㄥ' {('i', 'o')} else {('\0', 'ü')}}
            }
        }

        if is_y || init_char >= 'ㄐ' && init_char <= 'ㄒ' {
            if med_char == 'ㄩ' {
                if nuc == 'ü' || rime_char == 'ㄣ' {
                    //(j|q|x|y)ün? -> (j|q|x|y)un?
                    nuc = 'u';
                } else if rime_char != 'ㄥ' {
                    //(j|q|x|y)ü_ -> (j|q|x|y)u_
                    med = 'u'
                }
            }
            if is_y && (med_char == 'ㄧ' || med == 'i') {
                //yia,yii,yiong.. -> ya,yi,yong...
                med = '\0'
            }
        } else if is_w {
            //wuu, wuan... -> wu, wan...
            med = '\0'
        }

        (med, nuc, coda)
    }
    //#endregion
}

//#region TRAITS
impl Ord for Syllable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ord().cmp(&other.ord())
    }
}

impl PartialOrd for Syllable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn _bpmf_part_to_char(part: u8, lower_bound: char) -> char {
    if part == 0 {
        '\0'
    } else {
        unsafe { char::from_u32_unchecked(part as u32 + lower_bound as u32) }
    }
}

impl Display for Syllable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return f.write_str("");
        }

        if self.tone == 5 {
            f.write_char(bc::TONE_5)?
        }

        if self.init > 0 {
            f.write_char(_bpmf_part_to_char(self.init, bc::BEFORE_BO))?
        }
        if self.med > 0 {
            f.write_char(_bpmf_part_to_char(self.med, bc::ER))?
        }
        if self.rime > 0 {
            f.write_char(_bpmf_part_to_char(self.rime, bc::S))?
        }
        if self.tone > 1 && self.tone < 5 {
            f.write_char(match self.tone {
                2 => bc::TONE_2,
                3 => bc::TONE_3,
                _ => bc::TONE_4,
            })?
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ParseBopomofoError;

impl FromStr for Syllable {
    type Err = ParseBopomofoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Syllable::parse_bopomofo(s)
            .map(|x| x.0)
            .or_else(|_| Syllable::parse_pinyin(s).map(|x| x.0))
    }
}

//#endregion

//#region convenient functions

pub fn pinyin_to_bopomofo(txt: &str) -> Option<String> {
    Syllable::parse_pinyin(txt)
        .ok()
        .map(|res| res.0.to_string())
}

pub fn bopomofo_to_pinyin(txt: &str) -> Option<String> {
    Syllable::parse_bopomofo(txt)
        .ok()
        .map(|res| res.0.to_pinyin())
}

pub fn ascii_pinyin_to_bopomofo(txt: &str) -> Option<String> {
    Syllable::parse_ascii_pinyin(txt)
        .ok()
        .map(|res| res.0.to_string())
}
pub fn ascii_pinyin_to_pinyin(txt: &str) -> Option<String> {
    Syllable::parse_ascii_pinyin(txt)
        .ok()
        .map(|res| res.0.to_pinyin())
}
pub fn bopomofo_to_ascii_pinyin(txt: &str) -> Option<String> {
    Syllable::parse_bopomofo(txt)
        .ok()
        .map(|res| res.0.to_ascii_pinyin())
}
pub fn pinyin_to_ascii_pinyin(txt: &str) -> Option<String> {
    Syllable::parse_pinyin(txt)
        .ok()
        .map(|res| res.0.to_ascii_pinyin())
}
//#endregion
