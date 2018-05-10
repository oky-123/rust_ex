fn main() {
    println!("Hello, world!");

    /* 4.1 変数束縛 */

    // let式の左側はパターン
    let (x, y) = (1, 2);

    // 型アノテーション
    let x: i32 = 5;

    // インターポーレート
    println!("interpolate: {}", x);

    // format
    // debug
    let format = format!("format: {:?}", (x, y));
    println!("{}", format);

    // precision
    println!("Hello {0} is {1:.5}", "x", 0.01);
    // 引数0のpresision
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);

    /* 4.2 関数 */
    print_number(x);

    // !型は任意の型として扱える
    // let x: i32 = diverges();

    // 関数ポインタ
    let f: fn(i32) -> i32 = add_one;
    println!("{:?}", f);
    println!("{:#?}", f);

    let x = f(1);
    println!("{:?}", x);

    /* 4.3 プリミティブ型 */
    // char型
    // 4バイト！
    let two_hearts = '💕';
    println!("{}", two_hearts);

    // スライシング
    let a = [0, 1, 2, 3];
    let complete = &a[..];
    println!("{:?}", complete);
    // 1から3まで
    let complete = &a[1..4];
    println!("{:?}", complete);

    // タプル
    (0,); // 1要素のタプル
    (0); // 丸括弧に囲まれたゼロ

    /* 4.5 if */
    let x = 5;
    if x == 5 {
        println!("x は 5 です!");
    }

    let y = if x == 5 {
        10
    } else {
        15
    };

    println!("y は {} です!", y);

    /* 4.6 ループ */
    // loop
    // loop {
    //     println!("Loop forever!");
    // }
    
    // while
    let mut x = 5;
    let mut done = false;
    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 {
            done = true;
        }
    }

    // for 
    for x in 0..10 {
        println!("{}", x);
    }
    for (i,j) in (5..10).enumerate() {
        println!("i = {} and j = {}", i, j);
    }

    // ループラベル
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // x のループを継続
            if y % 2 == 0 { continue 'inner; } // y のループを継続
            println!("x: {}, y: {}", x, y);
        }
    }

    /* 4.7 所有権 */
    // 変数束縛... 束縛されているものの所有権をもつ

    // Copy型...トレイト
    // プリミティブには実装されている

    /* 4.8 借用 */
    // 借用
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = foo(&v1, &v2);

    // ここではv1とv2が使える!
    println!("{:?}", v1);

    // 借用ルール
    // ・借用は全て所有者のスコープより長く存続してはならない
    // ・リソースに対する1つ以上の参照（ &T ）
    // ・ただ1つのミュータブルな参照（ &mut T ）
    // は同時に持てない
    let mut x = 5;
    {
        let y = &mut x; // -+ &mut借用がここから始まる
        *y += 1;        //  |
    }                   // -+ ... そしてここで終わる
    println!("{}", x);  // <- ここでxを借用しようとする


    /* 4.9 ライフタイム */
}

// 引数の型は宣言しなければならない
fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

// !型...発散する
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // v1とv2についての作業を行う

    // 答えを返す
    42
}
