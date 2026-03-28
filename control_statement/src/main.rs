fn main() {
    println!("Program Start");

    // if文
    let x: u8 = 3;
    if x == 0 {

        println!("value: 0");
    }
    else if x == 1{

        println!("value: 1");
    }
    else{

        println!("value not 0 or 1")
    }

    // loop
    println!("loop");
    let mut count: u8 = 0;
    loop{

        if(count >= 5){
            break;
        }

        count += 1;
        println!("count: {}", count);
    }
    println!("loop end");

    // while
    println!("while start");
    let mut number: i8 = 5;
    while number > 0 {
        
        number -= 1;
        println!("number: {}", number);
    }
    println!("while end");

    // for
    let array = [1, 3, 5, 7];
    for index in (0..4){

        println!("for: value[{}] = {}", index, array[index]);
    }

    
    
}
