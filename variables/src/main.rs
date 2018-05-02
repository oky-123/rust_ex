fn main() {
    let mut x = 4;
    println!("The value of x is :{}", x);
    x = 5;
    println!("The value of x is :{}", x);

    let x = 5;

    let x = x + 1;

    println!("The value of x is :{}", x);

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

    /* 関数ポインタ */
    let f: fn(i32) -> i32 = add_one;
    let x2 = f(y);
    println!("The value of y is :{}", x2);
}
