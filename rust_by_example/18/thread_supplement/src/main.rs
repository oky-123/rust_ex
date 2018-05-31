#![allow(dead_code)]
extern crate rand;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn do_something(x: i32) -> i32 {
    let sec = 1;
    println!("Duration is = {} seconds.", sec);
    let duration = Duration::from_secs(sec);
    thread::sleep(duration);
    x * 2
}

fn run1() {
    let data = 1;
    // threadを生成してその中でprintする
    thread::spawn(move || {
        let result = do_something(data);
        println!("{:?}", result);
    });

    thread::sleep(Duration::from_millis(500));
}

fn run2() {
    let data = 1;
    // スレッドを生成して
    let child = thread::spawn(move || do_something(data));

    // child: JoinHandleでアタッチしているスレッドの終了を待って
    // 終了したら結果をResult型で受け取れる
    println!("{:?}", child.join());
    println!("Completed.");
}

fn run3() {
    let data = 1;
    // tx: Sender, rx: Reciever
    // MPSC ... multi-producer, single-consumer FIFO queue communication primitives
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(data * 2).unwrap();
    });

    // Recieverでreceiveしている
    println!("{:?}", rx.recv());
}

fn main() {
    // run1();
    // run2();
    run3();
}
