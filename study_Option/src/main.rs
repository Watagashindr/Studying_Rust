fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

fn last_char(text: &str) -> Option<char> {
    text.chars().last()
}

fn chartest(text: &str) -> Option<i32> {
    Some(4545)
}

fn get_number(num: &[i32], index: usize) -> Option<i32> {
    num.get(index).copied()
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

}

