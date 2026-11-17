
//整数の割り算
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b != 0 {
        return Ok( a / b);
    } 
    return Err("Fuckyou".to_string())
}

//テキストを整数に変換
fn parse_number(text: &str) -> Result<i32 , String> {
    let num: Result<i32, _ > = text.parse();
    match num {
        Ok(returnparse) => Ok(returnparse),
        Err(returnerror) => Err("OMGMEN".to_string()),
        
    }
}

//成人かどうかチェック
fn check_age(age: i32) -> Result<String,String> {
    if age >= 18 {
        return Ok("成人です".to_string());
    }
    return Err("未成年です".to_string());
}

//値のチェック
fn check_number(num: i32) -> Result<String, String>{

    if num <= 100 && num >= 0 {
        return Ok("正常な値です".to_string());
    }

    if num > 100 {
        return Err("100以上です".to_string());
    }else {
        return Err("0以下です".to_string());
    }
    
    return Err("予想外の入力値です".to_string());

}

//0以下かどうかを判断
fn check_underzero(num: i32) -> Result<i32, String> {
    if num > 0 {
        return Ok(num);
    }
    return Err("0以下です".to_string())
}
//100以上かどうかを判断
fn check_overhundred(num: i32) -> Result<i32, String> {
    if num < 100 {
        return Ok(num);
    }
    return Err("100より大きいです".to_string())
}

fn double_num(num: i32) -> Result<i32,String> {

    let check = check_underzero(num);

    match check {
        Ok(returnnum) => {
            let check2 = check_overhundred(returnnum * 2);
            match check2 {
                Ok(returnnum) => Ok(returnnum),
                Err(returntext) => Err(returntext),
            }
        }
        ,
        Err(returntext) => Err(returntext),
    }

}

//?を試す
fn double_num_question(num: i32) -> Result<i32, String> {
    
    let check = check_underzero(num)?;
    let check2 = check_overhundred(check * 2)?;

    return Ok(check2);
}

//and_then()を試す
fn double_num_andthen(num: i32) -> Result<i32 , String> {

    let check = check_underzero(num).and_then(|value| check_overhundred(value *2));

    return check;

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

    match check_age(200) {
        Ok(returntext ) => println!("{}",returntext),
        Err(returntext) => println!("{}",returntext),
    }

    let checknum = [-5,3,100];
    for i in checknum {
        match check_number(i) {
                Ok(returntext ) => println!("{}",returntext),
                Err(returntext) => println!("{}",returntext),
        }
    }
    match double_num(10) {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    match double_num(51) {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }

    match double_num_question(51) {
        Ok(value) => println!("{}", value),
        Err(error) => println!("{}", error),
    }
    
}
