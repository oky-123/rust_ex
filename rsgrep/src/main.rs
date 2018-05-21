// File structをuse
use std::fs::File;
// BufReader struct, BufRead crateをuse
use std::io::{BufReader, BufRead};
// envモジュール全体をuse
use std::env;

fn usage() {
    println!("rsgrep PATTERN FILENAME");
}
fn main() {
    let filename = match env::args().nth(2) {
        // あった場合取り出す
        Some(filename) => filename,
        // なければヘルプを表示して終了
        None => {
            usage();
            return;
        }
    };
    // File構造体のopen関連関数でファイルをオープンする
    // 失敗した場合は, Resultが返される
    // 下の方でもう一度filenameを使うためにここでは&filenameと参照で渡していることに注意
    let file = match File::open(&filename) {
        // 成功すれば中身を引き出す
        Ok(file) => file,
        Err(e) => {
            println!("An error occured while opening file {}:{}", filename, e);
            return;
        }
    };

    // FileをBufReaderに包んで使う
    let input = BufReader::new(file);
    // lines()メソッドを使い, イテレータでforを回す
    for line in input.lines() {
        // 入力がUTF-8ではないなどの理由で行のパースに失敗することがあるので
        // `line`もResultに包まれている
        let line = match line {
            Ok(line) => line,
            // エラーなら出力して終了
            Err(e) => {
                println!("An error occurred while reading a line {}", e);
                return;
            }
        };
        println!("{}", line);
    }

}
