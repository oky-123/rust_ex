use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn sorted_string(s: &str) -> String {
    // collectはイテレータをcollectionに変換する
    let mut s = s.chars().collect::<Vec<_>>();
    // s...Vec
    s.sort();
    s.into_iter().collect::<String>()
}

struct Anagram(HashMap<String, Vec<String>>);

impl Anagram {
    // トレイト境界`AsRef<Path>`は、ざっくり意訳すると「パス名っぽいもの」を表す
    // `Self`は、`Anagram`へのエイリアス
    fn new
}

fn main() {
    let word: String = std::env::args().nth(1).expect("USAGE: word");
    println!("{} is offered.", word);
    let sorted_word: String = sorted_string(&word);
    println!("{} is sorted string.", sorted_word);
}
