use super::{enums::*, spell_tree::SpellTree};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use sugars::hmap;

pub const PINYIN_INITIALS: [&'static str; 25] = [
    "", //0 initial
    "b", "p", "m", "f", "d", "t", "n", "l", "g", "k", "h", //11th
    "j", "q", "x", "zh", "ch", "sh", "r", "z", "c", "s", //21th
    "y", "w", "y", // yi wu yu
];
pub const PINYIN_NUC_CODAS: [(char, &'static str); 17] = [
    ('i', ""), //ㄭ with no rime no medial
    ('a', ""),
    ('o', ""),
    ('e', ""),
    ('e', ""),
    ('a', "i"),
    ('e', "i"),
    ('a', "o"),
    ('o', "u"),
    ('a', "n"),
    ('e', "n"),
    ('a', "ng"),
    ('e', "ng"),
    ('e', "r"),
    ('i', ""),
    ('u', ""),
    ('ü', ""),
];

pub const PINYIN_TONED_NUCS: [(char, &str); 6] = [
    ('a', "aāáǎàa"),
    ('e', "eēéěèe"),
    ('i', "iīíǐìi"),
    ('o', "oōóǒòo"),
    ('u', "uūúǔùu"),
    ('ü', "üǖǘǚǜü"),
];


static INIT_PINYIN_TO_BOPOMOFO_MAP: OnceCell<HashMap<char, char>> = OnceCell::new();
pub fn get_init_pinyin_to_bopomofo_map() -> &'static HashMap<char, char> {
    INIT_PINYIN_TO_BOPOMOFO_MAP.get_or_init(|| {
        HashMap::from_iter(
            get_init_bopomofo_to_pinyin_map()
                .iter()
                .filter(|(_, v)| v.len() == 1)
                .map(|(k, v)| (v.chars().next().unwrap(), *k)),
        )
    })
}


static PINYIN_INIT_MAP: OnceCell<HashMap<char, &'static str>> = OnceCell::new();
pub fn get_init_bopomofo_to_pinyin_map() -> &'static HashMap<char, &'static str> {
    PINYIN_INIT_MAP.get_or_init(|| {
        hmap! {
            '\0'=>"",
            'ㄅ'=>"b",
            'ㄆ'=>"p",
            'ㄇ'=>"m",
            'ㄈ'=>"f",
            'ㄉ'=>"d",
            'ㄊ'=>"t",
            'ㄋ'=>"n",
            'ㄌ'=>"l",
            'ㄍ'=>"g",
            'ㄎ'=>"k",
            'ㄏ'=>"h",
            'ㄐ'=>"j",
            'ㄑ'=>"q",
            'ㄒ'=>"x",
            'ㄓ'=>"zh",
            'ㄔ'=>"ch",
            'ㄕ'=>"sh",
            'ㄖ'=>"r",
            'ㄗ'=>"z",
            'ㄘ'=>"c",
            'ㄙ'=>"s",
        }
    })
}

//#region rime trees
use Med::*;
use Rime::*;
pub const PINYIN_UNTONED_RIMES: [(&str, (Med, Rime)); 46] = [
    ("a", (NoMed, A)),
    ("o", (NoMed, O)),
    ("e", (NoMed, E)),
    ("ê", (NoMed, Eh)),
    ("eh", (NoMed, Eh)),
    ("ai", (NoMed, Ai)),
    ("ei", (NoMed, Ei)),
    ("er", (NoMed, Er)),
    ("ao", (NoMed, Ao)),
    ("ou", (NoMed, Ou)),
    ("an", (NoMed, An)),
    ("en", (NoMed, En)),
    ("ang", (NoMed, Ang)),
    ("eng", (NoMed, Eng)),
    ("i", (Yi, NoRime)),
    ("ia", (Yi, A)),
    ("io", (Yi, O)),
    ("ie", (Yi, Eh)),
    ("iai", (Yi, Ai)),
    ("iao", (Yi, Ao)),
    ("iu", (Yi, Ou)),
    ("iou", (Yi, Ou)),
    ("ian", (Yi, An)),
    ("in", (Yi, En)),
    ("iang", (Yi, Ang)),
    ("ing", (Yi, Eng)),
    ("u", (Wu, NoRime)),
    ("ua", (Wu, A)),
    ("uo", (Wu, O)),
    // ("ue", (Yu, Eh)),
    ("uai", (Wu, Ai)),
    ("ui", (Wu, Ei)),
    ("uei", (Wu, Ei)),
    ("uan", (Wu, An)),
    ("un", (Wu, En)),
    ("uang", (Wu, Ang)),
    ("ong", (Wu, Eng)),
    ("v", (Yu, NoRime)),
    ("ve", (Yu, Eh)),
    ("van", (Yu, An)),
    ("vn", (Yu, En)),
    ("iong", (Yu, Eng)),
    ("ü", (Yu, NoRime)),
    ("üe", (Yu, Eh)),
    ("ue", (Yu, Eh)),
    ("üan", (Yu, An)),
    ("ün", (Yu, En)),
];

static PINYIN_UNTONED_RIME_TREE: OnceCell<SpellTree<(Med, Rime)>> = OnceCell::new();
pub fn get_pinyin_untoned_rime_tree() -> &'static SpellTree<(Med, Rime)> {
    PINYIN_UNTONED_RIME_TREE.get_or_init(|| {
        let mut tree = SpellTree::new();
        for (term, med_rime) in PINYIN_UNTONED_RIMES {
            tree.insert(term, med_rime);
        }
        tree
    })
}

pub const PINYIN_TONED_RIMES: [(&str, (Med, Rime, u8)); 190] = [
    ("ā", (NoMed, A, 1)),
    ("á", (NoMed, A, 2)),
    ("ǎ", (NoMed, A, 3)),
    ("à", (NoMed, A, 4)),
    ("a", (NoMed, A, 5)),
    ("āi", (NoMed, Ai, 1)),
    ("ái", (NoMed, Ai, 2)),
    ("ǎi", (NoMed, Ai, 3)),
    ("ài", (NoMed, Ai, 4)),
    ("ai", (NoMed, Ai, 5)),
    ("ān", (NoMed, An, 1)),
    ("án", (NoMed, An, 2)),
    ("ǎn", (NoMed, An, 3)),
    ("àn", (NoMed, An, 4)),
    ("an", (NoMed, An, 5)),
    ("āng", (NoMed, Ang, 1)),
    ("áng", (NoMed, Ang, 2)),
    ("ǎng", (NoMed, Ang, 3)),
    ("àng", (NoMed, Ang, 4)),
    ("ang", (NoMed, Ang, 5)),
    ("āo", (NoMed, Ao, 1)),
    ("áo", (NoMed, Ao, 2)),
    ("ǎo", (NoMed, Ao, 3)),
    ("ào", (NoMed, Ao, 4)),
    ("ao", (NoMed, Ao, 5)),
    ("ē", (NoMed, E, 1)),
    ("é", (NoMed, E, 2)),
    ("ě", (NoMed, E, 3)),
    ("è", (NoMed, E, 4)),
    ("e", (NoMed, E, 5)),
    ("ēh", (NoMed, Eh, 1)),
    ("éh", (NoMed, Eh, 2)),
    ("ěh", (NoMed, Eh, 3)),
    ("èh", (NoMed, Eh, 4)),
    ("eh", (NoMed, Eh, 5)),
    ("ēi", (NoMed, Ei, 1)),
    ("éi", (NoMed, Ei, 2)),
    ("ěi", (NoMed, Ei, 3)),
    ("èi", (NoMed, Ei, 4)),
    ("ei", (NoMed, Ei, 5)),
    ("ēn", (NoMed, En, 1)),
    ("én", (NoMed, En, 2)),
    ("ěn", (NoMed, En, 3)),
    ("èn", (NoMed, En, 4)),
    ("en", (NoMed, En, 5)),
    ("ēng", (NoMed, Eng, 1)),
    ("éng", (NoMed, Eng, 2)),
    ("ěng", (NoMed, Eng, 3)),
    ("èng", (NoMed, Eng, 4)),
    ("eng", (NoMed, Eng, 5)),
    ("ēr", (NoMed, Er, 1)),
    ("ér", (NoMed, Er, 2)),
    ("ěr", (NoMed, Er, 3)),
    ("èr", (NoMed, Er, 4)),
    ("er", (NoMed, Er, 5)),
    ("ō", (NoMed, O, 1)),
    ("ó", (NoMed, O, 2)),
    ("ǒ", (NoMed, O, 3)),
    ("ò", (NoMed, O, 4)),
    ("o", (NoMed, O, 5)),
    ("ōu", (NoMed, Ou, 1)),
    ("óu", (NoMed, Ou, 2)),
    ("ǒu", (NoMed, Ou, 3)),
    ("òu", (NoMed, Ou, 4)),
    ("ou", (NoMed, Ou, 5)),
    ("ī", (Yi, NoRime, 1)),
    ("í", (Yi, NoRime, 2)),
    ("ǐ", (Yi, NoRime, 3)),
    ("ì", (Yi, NoRime, 4)),
    ("i", (Yi, NoRime, 5)),
    ("iā", (Yi, A, 1)),
    ("iá", (Yi, A, 2)),
    ("iǎ", (Yi, A, 3)),
    ("ià", (Yi, A, 4)),
    ("ia", (Yi, A, 5)),
    ("iān", (Yi, An, 1)),
    ("ián", (Yi, An, 2)),
    ("iǎn", (Yi, An, 3)),
    ("iàn", (Yi, An, 4)),
    ("ian", (Yi, An, 5)),
    ("iāng", (Yi, Ang, 1)),
    ("iáng", (Yi, Ang, 2)),
    ("iǎng", (Yi, Ang, 3)),
    ("iàng", (Yi, Ang, 4)),
    ("iang", (Yi, Ang, 5)),
    ("iāo", (Yi, Ao, 1)),
    ("iáo", (Yi, Ao, 2)),
    ("iǎo", (Yi, Ao, 3)),
    ("iào", (Yi, Ao, 4)),
    ("iao", (Yi, Ao, 5)),
    ("iē", (Yi, Eh, 1)),
    ("ié", (Yi, Eh, 2)),
    ("iě", (Yi, Eh, 3)),
    ("iè", (Yi, Eh, 4)),
    ("ie", (Yi, Eh, 5)),
    ("īn", (Yi, En, 1)),
    ("ín", (Yi, En, 2)),
    ("ǐn", (Yi, En, 3)),
    ("ìn", (Yi, En, 4)),
    ("in", (Yi, En, 5)),
    ("īng", (Yi, Eng, 1)),
    ("íng", (Yi, Eng, 2)),
    ("ǐng", (Yi, Eng, 3)),
    ("ìng", (Yi, Eng, 4)),
    ("ing", (Yi, Eng, 5)),
    ("iū", (Yi, Ou, 1)),
    ("iōu", (Yi, Ou, 1)),
    ("iú", (Yi, Ou, 2)),
    ("ióu", (Yi, Ou, 2)),
    ("iǔ", (Yi, Ou, 3)),
    ("iǒu", (Yi, Ou, 3)),
    ("iù", (Yi, Ou, 4)),
    ("iòu", (Yi, Ou, 4)),
    ("iu", (Yi, Ou, 5)),
    ("iou", (Yi, Ou, 5)),
    ("ū", (Wu, NoRime, 1)),
    ("ú", (Wu, NoRime, 2)),
    ("ǔ", (Wu, NoRime, 3)),
    ("ù", (Wu, NoRime, 4)),
    ("u", (Wu, NoRime, 5)),
    ("uā", (Wu, A, 1)),
    ("uá", (Wu, A, 2)),
    ("uǎ", (Wu, A, 3)),
    ("uà", (Wu, A, 4)),
    ("ua", (Wu, A, 5)),
    ("uāi", (Wu, Ai, 1)),
    ("uái", (Wu, Ai, 2)),
    ("uǎi", (Wu, Ai, 3)),
    ("uài", (Wu, Ai, 4)),
    ("uai", (Wu, Ai, 5)),
    ("uān", (Wu, An, 1)),
    ("uán", (Wu, An, 2)),
    ("uǎn", (Wu, An, 3)),
    ("uàn", (Wu, An, 4)),
    ("uan", (Wu, An, 5)),
    ("uāng", (Wu, Ang, 1)),
    ("uáng", (Wu, Ang, 2)),
    ("uǎng", (Wu, Ang, 3)),
    ("uàng", (Wu, Ang, 4)),
    ("uang", (Wu, Ang, 5)),
    ("uī", (Wu, Ei, 1)),
    ("uēi", (Wu, Ei, 1)),
    ("uí", (Wu, Ei, 2)),
    ("uéi", (Wu, Ei, 2)),
    ("uǐ", (Wu, Ei, 3)),
    ("uěi", (Wu, Ei, 3)),
    ("uì", (Wu, Ei, 4)),
    ("uèi", (Wu, Ei, 4)),
    ("ui", (Wu, Ei, 5)),
    ("uei", (Wu, Ei, 5)),
    ("ūn", (Wu, En, 1)),
    ("ún", (Wu, En, 2)),
    ("ǔn", (Wu, En, 3)),
    ("ùn", (Wu, En, 4)),
    ("un", (Wu, En, 5)),
    ("ōng", (Wu, Eng, 1)),
    ("óng", (Wu, Eng, 2)),
    ("ǒng", (Wu, Eng, 3)),
    ("òng", (Wu, Eng, 4)),
    ("ong", (Wu, Eng, 5)),
    ("uō", (Wu, O, 1)),
    ("uó", (Wu, O, 2)),
    ("uǒ", (Wu, O, 3)),
    ("uò", (Wu, O, 4)),
    ("uo", (Wu, O, 5)),
    ("ǖ", (Yu, NoRime, 1)),
    ("ǘ", (Yu, NoRime, 2)),
    ("ǚ", (Yu, NoRime, 3)),
    ("ǜ", (Yu, NoRime, 4)),
    ("ü", (Yu, NoRime, 5)),
    ("üān", (Yu, An, 1)),
    ("üán", (Yu, An, 2)),
    ("üǎn", (Yu, An, 3)),
    ("üàn", (Yu, An, 4)),
    ("üan", (Yu, An, 5)),
    ("uē", (Yu, Eh, 1)),
    ("üē", (Yu, Eh, 1)),
    ("ué", (Yu, Eh, 2)),
    ("üé", (Yu, Eh, 2)),
    ("uě", (Yu, Eh, 3)),
    ("üě", (Yu, Eh, 3)),
    ("uè", (Yu, Eh, 4)),
    ("üè", (Yu, Eh, 4)),
    ("ue", (Yu, Eh, 5)),
    ("üe", (Yu, Eh, 5)),
    ("iōng", (Yu, Eng, 1)),
    ("ióng", (Yu, Eng, 2)),
    ("iǒng", (Yu, Eng, 3)),
    ("iòng", (Yu, Eng, 4)),
    ("iong", (Yu, Eng, 5)),
];

static PINYIN_TONED_RIME_TREE: OnceCell<SpellTree<(Med, Rime, u8)>> = OnceCell::new();
pub fn get_pinyin_toned_rime_tree() -> &'static SpellTree<(Med, Rime, u8)> {
    PINYIN_TONED_RIME_TREE.get_or_init(|| {
        let mut tree = SpellTree::new();
        for (term, fin) in PINYIN_TONED_RIMES {
            tree.insert(term, fin);
        }
        tree
    })
}
//#endregion
