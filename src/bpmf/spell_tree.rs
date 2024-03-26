use core::fmt;
use std::{
    collections::{binary_heap::Iter, HashMap},
    fmt::{Display, Formatter},
};

pub struct SpellTree<T> {
    meaning: Option<T>,
    branches: HashMap<char, Self>,
}

impl<T> SpellTree<T> {
    pub fn new() -> SpellTree<T> {
        SpellTree {
            meaning: None,
            branches: HashMap::new(),
        }
    }

    pub fn insert(&mut self, term: &str, meaning: T) {
        if let Some(ch) = term.chars().next() {
            let ch_len = ch.len_utf8();
            let node = self.branches.entry(ch).or_insert(SpellTree::new());
            if ch_len == term.len() {
                //reaches the end
                node.meaning = Some(meaning)
            } else {
                //insert it in a descendant branch
                node.insert(&term[ch_len..], meaning)
            }
        }
    }

    pub fn build_from(list: Iter<(&str, T)>) -> SpellTree<T>
    where
        T: Copy,
    {
        let mut root = SpellTree::new();
        for (term, meaning) in list {
            root.insert(term, *meaning)
        }
        root
    }

    pub fn find<'a>(&self, term: &'a str) -> Option<(T, &'a str)>
    where
        T: Copy,
    {
        let mut chars = term.chars();
        if let Some(ch) = chars.next() {
            if let Some(node) = self.branches.get(&ch) {
                let remainder = chars.as_str();
                return if remainder.is_empty() {
                    node.meaning.map(|x| (x, remainder)) //reaches the end
                } else {
                    node.find(remainder)
                };
            }
        }
        self.meaning.map(|m| (m, term))
    }

    //#region props
    pub fn is_leaf(&self) -> bool {
        self.branches.len() == 0
    }

    fn _show_nodes(&self, f: &mut Formatter<'_>, tabs: usize) -> fmt::Result {
        if !self.is_leaf() {
            for (k, v) in self.branches.iter() {
                let match_star = if v.meaning.is_some() { "*" } else { "" };
                writeln!(f, "{:>width$}  {k}{}", "-", match_star, width = tabs * 4,)?;
                v._show_nodes(f, tabs + 1)?;
            }
        }
        Ok(())
    }

    pub fn total_nodes(&self) -> usize {
        self.branches.values().fold(0, |agg, node| {
            agg + if node.is_leaf() {
                1
            } else {
                node.total_nodes()
            }
        })
    }
    //#endregion
}

impl<T> Display for SpellTree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self._show_nodes(f, 0)
    }
}
