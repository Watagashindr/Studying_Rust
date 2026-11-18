
const MAX_FUCK: i32 = 810;
static MAX_YOU: i32 = 4545;

fn double_num(num: Box<i32>) -> Box<i32> {
    return Box::new(*num *2);
}

fn printbox(s: Box<[u8]>) {
    println!("{:?}",s);
}

fn triple_num(num: Box<i32>) -> Box<i32> {
    return Box::new(*num *3);
}

fn sum_box(num: Box<Vec<i32>>) -> i32 {
    let mut sum = 0;

    for i in *num {
        sum += i;
    }

    return sum;
}

fn main() {
    let num = 100;
    let mut boxed_num = Box::new(100);

    *boxed_num = 200;
    println!("{}",boxed_num);

    *boxed_num = 400;
    println!("{}",boxed_num);

    let a = Box::new(300);

    let mut b = a;

    println!("{}",b);

    *b = 4545;
    println!("{}",b);
    //これはできないbに値が移っちゃってるから
    // println!("{}",a);

    let c = double_num(b);
    println!("{}",c);

    let byte_array = [b'h',b'e',b'l'];
    printbox(Box::new(byte_array));

    let mut s2 = Box::new("WTF".to_string());
    println!("{}",s2);
    *s2 = "OMG".to_string();
    println!("{}",s2);

    let a2 = Box::new(500);
    let mut b2 = a2;
    *b2 = 1000;
    println!("{}",b2);

    let c2 = triple_num(b2);
    println!("{}",c2);

    let numbers = Box::new(vec![1,2,3,4,5]);

    let sumnum = sum_box(numbers);

    println!("{}",sumnum);

}


