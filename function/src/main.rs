fn main() {
    
    let x = mul(7, 3);
    println!("value: {}", x);
}

// 関数宣言
fn mul(x: i32, y: i32) -> i32{

    // return文はセミコロンなし
    let ans = x*y;
    ans
}