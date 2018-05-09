fn main() {
    let mut x = 4;
    println!("The value of x is :{}", x);
    x = 5;
    println!("The value of x is :{}", x);

    let x = 5;

    let x = x + 1;

    println!("The value of x is :{}", x);

    // 4.2

    // 関数宣言
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    let x = add_one(x);
    println!("The value of x is :{}", x);

    /* 式と文 */
    // 文1...宣言文
    // とはかけないので注意
    let mut y = 1;

    let x = y = 2;
    // 式ではなく, y = 2 は空のタプル()を返すので
    // println!("The value of x is :{}", x);
    // するとエラー

    // 文2...式文
    // ; は式を文にすることができる
    // rustでは文の後に文が続く

    // その他は全て式

    // 関数
    /* 関数ポインタ */
    let f: fn(i32) -> i32 = add_one;
    let x2 = f(y);
    println!("The value of y is :{}", x2);

    // 4.3 プリミティブ型
    // ブーリアン型
    let x = true;
    println!("The value of x is :{}", x);

    // char 型
    // 1バイトではなく4バイト
    let x = 'x';
    let two_hearts = '💕';
    println!("The value of x is :{}", x);
    println!("The value of two_hearts is :{}", two_hearts);

    // 配列
    let a = [1, 2, 3]; // a: [i32; 3]
    let a = [0; 20]; 
    println!("The value of a[1] is :{}", a[1]);
    // println!("The value of a[1] is :{}", a[20]); // error

    // スライス
    let complete = &a[..];
    let middle = &a[1..4];

    // タプル
    (0,); // 1要素のタプル
    (0); // 丸括弧に囲まれたゼロ
    let tuple = (1, 2, 3);

    let x = tuple.0;
    let y = tuple.1;
    let z = tuple.2;

    println!("x is {}", x);

    // 4.6 ループ
    // for
    for (i, j) in (5..10).enumerate() {
        println!("i = {}, j = {}", i, j);
    }

    let mut x = 5;

    loop {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { break; }
    }

    for x in 0..10 {
        if x % 2 == 0 { continue; }

        println!("{}", x);
    }

    // ループラベル
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; }
            if y % 2 == 0 { continue 'inner; }
            println!("x: {}, y: {}", x, y);
        }
    }


    // 4.7 所有権
    fn take(v: Vec<i32>) {
        // ここで何が起きるかは重要ではない
    }
    let v = vec![1, 2, 3];
    // 所有権が移る, moveという
    take(v);
    // エラーが生じる, moved
    // println!("v[0] is: {}", v[0]);

    // ただし, 全てのプリミティブ型はCopyトレイトを実装しているのでムーブは起こらない。

    fn double(x: i8) -> i8 {
        x * 2
    }

    let a: i8 = 12;
    let b = double(a);
    println!("{}", a);

    let b = a;
    println!("{}", a);

    let val = vec![1,2,3];
    let sub = val;
    // println!( "{}", val[0] ); // error

}
