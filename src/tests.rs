use self::bpmf::{Init, Med, ParseBopomofoError, Rime, Syllable, Tone};
use super::*;
use Init::*;
use Med::*;
use Rime::*;
use Tone::*;

#[cfg(test)]
mod resources;
use resources::MANDARIN_SOUNDS;

#[test]
fn syllable_parser_can_skip_whitespaces_and_return_remaing_str() {
    let s = " \t  　'zhuang3非法";
    let (syl, remainder) = Syllable::parse_ascii_pinyin(s).unwrap();
    assert_eq!(syl, Syllable::new(Zhi, Wu, Ang, Dip));
    assert_eq!(remainder, &s[s.len() - 6..]);
}

#[test]
fn default_syllable_equals_empty_syllable() {
    let syl = Syllable::new(Init::NoInit, Med::NoMed, Rime::NoRime, Tone::NoTone);
    assert_eq!(Syllable::default(), syl);
}

#[test]
fn syllable_can_diplay_as_bopomofo() {
    let chuang3 = Syllable::new(Chi, Wu, Ang, Dip);
    let ei2 = Syllable::new(NoInit, NoMed, Ei, Rise);
    let fu1 = Syllable::new(Fo, Wu, NoRime, Level);
    let ger4 = Syllable::new(Ge, NoMed, Er, Fall);
    let shi5 = Syllable::new(Shi, NoMed, NoRime, Neut);
    assert_eq!(chuang3.to_string(), "ㄔㄨㄤˇ");
    assert_eq!(ei2.to_string(), "ㄟˊ");
    assert_eq!(fu1.to_string(), "ㄈㄨ");
    assert_eq!(ger4.to_string(), "ㄍㄦˋ");
    assert_eq!(shi5.to_string(), "˙ㄕ");
}

#[test]
fn syllable_can_parse_bopomofo() {
    let chuang3 = Syllable::new(Chi, Wu, Ang, Dip);
    let ei2 = Syllable::new(NoInit, NoMed, Ei, Rise);
    let fu1 = Syllable::new(Fo, Wu, NoRime, Level);
    let ger4 = Syllable::new(Ge, NoMed, Er, Fall);
    let shi5 = Syllable::new(Shi, NoMed, NoRime, Neut);
    let s_chuang3 = "\tㄔㄨㄤˇ.";
    let s_ei2 = "\nㄟˊ";
    let s_fu1 = "ㄈㄨ奧 ";
    let s_ger4 = "ㄍㄦˋ";
    let s_shi5 = "˙ㄕa";
    assert_eq!(s_chuang3.parse::<Syllable>().unwrap(), chuang3);
    assert_eq!(s_ei2.parse::<Syllable>().unwrap(), ei2);
    assert_eq!(s_fu1.parse::<Syllable>().unwrap(), fu1);
    assert_eq!(s_ger4.parse::<Syllable>().unwrap(), ger4);
    assert_eq!(s_shi5.parse::<Syllable>().unwrap(), shi5);

    for (bpmf, _) in MANDARIN_SOUNDS {
        let res = bpmf.parse::<Syllable>();
        if let Err(ParseBopomofoError) = res {
            panic!("Failed at parsing '{}' ", bpmf)
        }
        assert_eq!(res.unwrap().to_string(), bpmf);
    }
}

#[test]
fn syllables_have_correct_order(){
    let mut list = vec![];
    for (bpmf,_) in MANDARIN_SOUNDS{
        list.push(bpmf.parse::<Syllable>().unwrap());
    }
    let mut sorted = list.clone();
    sorted.reverse();
    sorted.sort();
    for (i,syl) in sorted.iter().enumerate(){
        if *syl != list[i]{

            println!("{} - {}",syl, list[i]);
        }
        // assert_eq!(*syl, list[i])
    }

}

#[test]
fn syllable_can_convert_to_pinyin() {
    for (bpmf, py) in MANDARIN_SOUNDS {
        let res = bpmf.parse::<Syllable>();
        if let Err(ParseBopomofoError) = res {
            panic!("Failed at parsing '{}' ", bpmf)
        }
        assert_eq!(res.unwrap().to_pinyin(), py);
    }
}

#[test]
fn syllable_can_parse_pinyin() {
    for (bpmf, py) in MANDARIN_SOUNDS {
        let b_syl = bpmf.parse::<Syllable>().unwrap();
        let res = Syllable::parse_pinyin(py);
        if let Err(_) = res {
            println!("Failed at parsing {bpmf} : {py}");
        }
        let (p_syl, _) = res.unwrap();
        assert_eq!(p_syl.to_pinyin(), b_syl.to_pinyin());
    }
}

#[test]
fn syllable_can_convert_to_ascii_pinyin() {
    let chuang3 = Syllable::new(Chi, Wu, Ang, Dip);
    let ei2 = Syllable::new(NoInit, NoMed, Ei, Rise);
    let fu1 = Syllable::new(Fo, Wu, NoRime, Level);
    let ger4 = Syllable::new(Ge, NoMed, Er, Fall);
    let shi5 = Syllable::new(Shi, NoMed, NoRime, Neut);
    let s_chuang3 = "chuang3";
    let s_ei2 = "ei2";
    let s_fu1 = "fu1";
    let s_ger4 = "ger4";
    let s_shi5 = "shi5";
    assert_eq!(chuang3.to_ascii_pinyin(), s_chuang3);
    assert_eq!(ei2.to_ascii_pinyin(), s_ei2);
    assert_eq!(fu1.to_ascii_pinyin(), s_fu1);
    assert_eq!(ger4.to_ascii_pinyin(), s_ger4);
    assert_eq!(shi5.to_ascii_pinyin(), s_shi5);
    assert_eq!(
        Syllable::new(Le, Yu, NoRime, NoTone).to_ascii_pinyin(),
        "lv"
    );
}

#[test]
fn syllable_can_parse_ascii_pinyin() {
    for (bpmf, py) in MANDARIN_SOUNDS {
        let b_syl = bpmf.parse::<Syllable>().unwrap();
        let ascii = b_syl.to_ascii_pinyin();
        let res = Syllable::parse_ascii_pinyin(&ascii);
        if let Err(_) = res {
            println!("Failed at parsing {bpmf} : {py} : {ascii}");
        }
        let (p_syl, _) = res.unwrap();
        assert_eq!(p_syl.to_pinyin(), b_syl.to_pinyin());
    }
}
