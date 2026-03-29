fn main() {
    
    // バイナリ（実行ファイルのデータ領域）への確保
    let s1: &str = "message1";
    println!("string_s1: {}", s1);

    // s1が所有権をもっているわけではないので、これはコピーが可能（内部処理されてる）
    // s1, s2両方とも使える
    let s2: &str = s1;
    println!("string_s2: {}", s2);
    println!("string_s1: {}", s1);

    // ヒープ上への確保
    let s3: String = String::from("message2");
    println!("string_s3: {}", s3);

    // 所有権を移しているので、ここでs3は破棄される
    // s3を使うことはできない
    let s4: String = s3;
    println!("string_s4: {}", s4);
    //println!("string_s3: {}", s3); // エラー

    // s4に所有権を持たせたまま、s5にコピー（s5はコピーされた変数の所有権を持つ）
    let s5: String = s4.clone();
    println!("string_s4: {}", s4);
    println!("string_s5: {}", s5);

    // s4に所有権を持たせたまま、s4の参照を受け取る（s6は参照のみで、所有権を持っていない）
    let s6: &String = &s4;
    println!("string_s6: {}", s6);


}       
