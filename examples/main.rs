use bpmf_py::bpmf::*;
fn main() {
    //Assemble a mandarin syllable from parts directly .
    let shuai4 = Syllable::new(Init::Shi, Med::Wu, Rime::Ai, Tone::Fall);

    println!("Constructed: '{shuai4}'"); //outputs: ㄕㄨㄞˋ

    //Four enums represent bopomofo parts:
    //They all implement From<ENUM> for char trait
    assert_eq!(char::from(Init::Shi), 'ㄕ');
    assert_eq!(char::from(Med::Yu), 'ㄩ');
    assert_eq!(char::from(Rime::Er), 'ㄦ');
    assert_eq!(char::from(Tone::Fall), 'ˋ');

    //They all have a default value which corresponds to '\0'
    assert_eq!(char::from(Init::NoInit), '\0');

    //All discriminants of these four enums have a unique name so it's okay to
    // bring them all into the scope if messing up the namespace is'nt your concern
    use Init::*;
    use Med::*;
    use Rime::*;
    use Tone::*;

    let qiang2 = Syllable::new(Qi, Yi, Ang, Rise);
    println!("Constructed '{}'", qiang2); //outputs: "ㄑㄧㄤˊ"

    //Parsing
    //The parser will skip all whitespaces and the syllable separator "'"
    let mut txt = "\t ㄎㄨㄟˋ 'ㄖㄣˊ";

    //Parse bopomofo
    let (mut syl, mut remainder) = Syllable::parse_bopomofo(txt).unwrap();
    assert_eq!(syl, Syllable::new(Ke, Wu, Ei, Fall));
    assert_eq!(remainder, " 'ㄖㄣˊ");

    //continue to parse the next syllable
    (syl, remainder) = Syllable::parse_bopomofo(remainder).unwrap();
    assert_eq!(syl, Syllable::new(Ri, NoMed, En, Rise));
    assert_eq!(remainder, "");

    //If the text is ill-formed it returns An Error
    txt = "万X尢";
    if let Err(ParseBopomofoError) = Syllable::parse_bopomofo(txt) {
        println!("Failed to parse '{}'", txt)
    }

    //parsing pinyin and ascii pinyin are similar:
    txt = "ráo";
    (syl, _) = Syllable::parse_pinyin(txt).unwrap();
    println!("Pinyin '{txt}' parsed to '{}'", syl.to_pinyin()); //outputs:'ráo'

    //***Ascii pinyin*** is a commonly used alternative form of standard pinyin,
    //in which letter 'ü' is substituted with 'v' and all vowel letters with
    //diacritical tone marks are not used, instead tones are indicated with
    //a trailing number. E.g. zhuǎng is spelt as zhuang3
    txt = "lve4";
    (syl, _) = Syllable::parse_ascii_pinyin(txt).unwrap();
    println!("Ascii pinyin '{txt}' parsed to '{}'", syl.to_pinyin()); //outputs:'lüè'

    //Even unshortened form is recognized
    txt = "qiou2";
    (syl, _) = Syllable::parse_ascii_pinyin(txt).unwrap();
    println!(
        "Long form ascii pinyin '{txt}' parsed to '{}'",
        syl.to_pinyin()
    ); //outputs:'qiú'

    //FromStr trait is implemented, both bopomofo and pinyin(standard form)
    //are recognized
    txt = "ㄑㄧㄠˇ";
    syl = txt.parse().unwrap();
    println!("Bopomofo '{txt}' recognized as '{}'", syl); //outputs: 'ㄑㄧㄠˇ'

    txt = "qiǎo";
    syl = txt.parse().unwrap();
    println!("Pinyin {txt} recognized as '{}'", syl); //also outputs: 'ㄑㄧㄠˇ'

    //The Syllable object implemented Eq and Ord
    syl = Syllable::new(Ri, Wu, Ang, Dip); // a fabricated sound
    let (syl2, _) = Syllable::parse_ascii_pinyin("ruang3").unwrap();
    assert_eq!(syl, syl2);
    let orig_syllables = ["zhuan4", "an3", "an1", "bo2", "qi3"];
    let mut sorted_syllables: Vec<Syllable> = orig_syllables
        .into_iter()
        .map(Syllable::parse_ascii_pinyin)
        .map(|res| res.unwrap().0)
        .collect();
    sorted_syllables.sort();
    println!(
        "{:?} sorted: {:?}",
        orig_syllables,
        sorted_syllables
            .iter()
            .map(Syllable::to_ascii_pinyin)
            .collect::<Vec<String>>()
    );
    //outputs: [bo2, qi3, zhuan4, an1, an3]
    //The order conforms to the order of bopomofo: b p ... i u ü

    //For you convenience the following utility functions are also provided:
    //They all returns Option<String>
    println!("\n- - -\nCalling convenient functions performing direct conversion");
    println!("{}", pinyin_to_ascii_pinyin("ráo").unwrap()); //outputs: rao2
    println!("{}", ascii_pinyin_to_pinyin("rao2").unwrap()); //outputs: ráo
    println!("{}", bopomofo_to_pinyin("ㄑㄩㄥ").unwrap()); //outputs: "qiōng"
    println!("{}", pinyin_to_bopomofo("qiōng").unwrap()); //outputs: "ㄑㄩㄥ
    println!("{}", ascii_pinyin_to_bopomofo("qiong1").unwrap()); //outputs: "ㄑㄩㄥ
    println!("{}", bopomofo_to_ascii_pinyin("ㄑㄩㄥ").unwrap()); //outputs: "qiong1
}
