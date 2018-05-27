fn apply<F>(f: F) where
    F: FnOnce() {
        f()
    }

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
    }

fn main() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    // `to_owned` creates owned data from borrowed one
    // let _farewell = "good bye";
    // let mut farewell = _farewell.to_owned();
    let mut farewell = "good bye".to_owned();

    // 変数を二つ補足
    let diary = || {
        // greeting is by reference.
        println!("I said {}.", greeting);

        // farewell の値を変更するのでこの時点で FnMutが必要
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now, I can sleep.");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
