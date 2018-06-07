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
    // この構造体は辞書を読み取ってここからanagram用の辞書を作成する
    fn new<P: AsRef<Path>>(dictfile: P) -> Result<Self, io::Error> {
        // file ... `Result<Ok(File), Error>`
        let file = File::open(dictfile)?;
        let file = io::BufReader::new(file);
        // preperate anagram
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?;
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    fn add_word(&mut self, word: String) {
        let sorted = sorted_string(&word);
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    fn find(&self, word: String) -> Option<&Vec<String>> {
        let word = sorted_string(&word);
        self.0.get(&word)
    }
}

fn main() {
    let word: String = std::env::args().nth(1).expect("USAGE: word");
    println!("{} is offered.", word);
    // Unixの場合ここに辞書がある
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");
    println!("{:?}", table.find(word));
}
