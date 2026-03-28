fn main() {
    
    println!("Program Start");

    // 定数
    const DAY_TIME_SEC: u32 = 24 * 60 * 60;
    println!("1Day = {}s", DAY_TIME_SEC);

    // 変数（可変）
    // 型注釈を書くことができる（書かなくても勝手にやってくれる）
    let mut x: i32 = 120;
    println!("1.value_x: {}", x);

    // mutをつけないとここでエラーになる
    x = 20;
    println!("2.value_x: {}", x);

    // 再宣言ができる
    let x: i32 = 10;
    println!("3.value_x: {}", x);

    // 括弧内でのみ変数が別の変数として振る舞う
    {
        let x: i32 = x * 2;
        println!("4.value_2x: {}", x);
    }

    // 括弧を外れたので、元の値(3で再宣言した10になる)
    println!("5.value_x: {}", x);

    // 再代入で変数型を変えることもできる
    // string
    let x: &str = "message";
    println!("6.string.x: {}", x);

    // char
    let x: char = 'A';
    println!("7.char: {}", x);

    // bool
    let x: bool = false;
    println!("8.is_valid: {}", x);

    // tuple
    let tup:(u8, bool, f32) = (27, false, 3.14);

    // 要素の取り出し
    let (x, y, z) = tup;
    println!("9.2tup {} {} {}", x, y, z);

    // tupleの出力は特殊(debug)
    // ドット演算子でアクセスできる
    println!("10.tup: {:?}, tup_content: {} {} {}", tup, tup.0, tup.1, tup.2);

    // 配列
    let array: [i32; 5] = [1,2,3,4,5];
    println!("11.array: {}", array[3]);

    // 配列外アクセスはエラーになる
    // println!("value: {}",array[5]);

}
