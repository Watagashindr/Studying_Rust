fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

//テキストを受け取った際に最後の文字を返す
fn last_char(text: &str) -> Option<char> {
    text.chars().last()
}

//テキストを受け取れた場合に4545を数値型で返す
fn chartest(text: &str) -> Option<i32> {
    Some(4545)
}

//配列と番号を取得し値があったらSome（）に返す
fn get_number(num: &[i32], index: usize) -> Option<i32> {
    num.get(index).copied()
}



//配列から偶数を取得し値があったらSome（）に返す
fn find_even(num: &[i32]) -> Option<i32> {

    for i in num{

        if *i % 2 == 0 {
            return Some(*i);
        }
        
    }

    return None
}

//配列の中の最初の正の値をSome（）に返す
fn find_positivenum(num: &[i32]) -> Option<i32> {
    for i in num {
        if *i >= 0 {
            return Some(*i);
        }
    }
    return None
}   

fn main() {

    let text = "Rust";

    match first_char(text) {
        Some(c) => println!("先頭の文字:{}",c),
        None => println!("文字列がからです"),
    }

    let text2 = "Fuck";

    match last_char(text2) {
        Some(returntext) => println!("最後の文字:{}",returntext),
        None => println!("WTF"),
    }

    let text3 = "数字ある？";

    match chartest(text3) {
        Some(returnnum) => println!("{}",returnnum),
        None => println!("noting num"),
    }

    let numbers = [10,20,30,40];

    match get_number(&numbers,2) {
        Some(returnnum) => println!("{}",returnnum),
        None => println!("fuckyou"),
    }

    let findevennum = [1,5,3,4,6];

    match find_even(&findevennum) {
        Some(returnnum) => println!("{}",returnnum),
        None => println!("fuckyou"),
    }
    
    let mut findpositivenum = [-1,-100,5,-99,3];

    match find_positivenum(&findpositivenum) {
        Some(returnnum) => println!("{}",returnnum),
        None => println!("fuckyou"),
    }
}

