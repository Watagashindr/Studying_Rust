
fn search_even(num: &[i32]) -> Option<i32> {

    for i in num{
        if i % 2 == 0 {
            return Some(*i);
        }
    }
    return None;
}

fn sum_vec(num: &[i32]) -> i32 {
    
    let mut sum: i32 = 0;

    for i in num{
        sum += *i;
    }

    return sum
}


fn main() {
    let mut v1 = vec![1,2,3,4,5];
    let v2 = vec![0; 5];

    for i in &v1 {
        print!("{}",i);
    }
    println!("");
    v1.push(6);

    for i in &v1 {
        print!("{}",i);
    }
    println!("");
    v1.pop();

    for i in &v1 {
        print!("{}",i);
    }
    println!("");

    println!("{}",v1[1]);

    let v3 = vec![1,3,5,2,6];
    match search_even(&v3) {
        Some(returnnum) => println!("first even:{}",returnnum),
        None => println!("Not even"),
    }
}
