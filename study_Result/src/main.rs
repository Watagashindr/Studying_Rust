
//整数の割り算
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        return Ok( a / b);
    } 
    return Err("Fuckyou".to_string())
}

fn main() {
    match divide(2,5){
        Ok(returnnum ) => println!("{}",returnnum),
        Err(returntext) => println!("{}",returntext),
    }
}
