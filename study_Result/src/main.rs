
//整数の割り算
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        return Ok( a / b);
    } 
    return Err("Fuckyou".to_string())
}

//テキストを整数に変換
fn parse_number(text: &str) -> Result<i32 , String> {
    let num: Result<i32, _> = text.parse();
    match num {
        Ok(returnparse) => Ok(returnparse),
        Err(returnerror) => Err("OMGMEN".to_string()),
        
    }
}

fn main() {
    match divide(2,5){
        Ok(returnnum ) => println!("{}",returnnum),
        Err(returntext) => println!("{}",returntext),
    }

    match parse_number("0120444444") {
        Ok(returnnum ) => println!("{}",returnnum),
        Err(returntext) => println!("{}",returntext),
    }

    println!("test fuckyou");
}
