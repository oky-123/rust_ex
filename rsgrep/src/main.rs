// File structをuse
use std::fs::File;
// BuffReader struct, BuffRead crateをuse
use std::io::{BuffReader, BuffRead};
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

}
