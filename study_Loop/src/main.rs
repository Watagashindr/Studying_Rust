const LOOPLIMIT: i32 = 20;

fn main() {
    let mut count = 0;

    loop {
        println!("{}",count + 1);

        count += 1;

        if count >= LOOPLIMIT {
            break;
        }
    }

    count = 0;
    'secondloop: loop {
        println!("secondloop start");

        'thirdloop: loop {

            println!("thirdloop start");

            break 'secondloop;

            println!("thirdloop end");//表示されない
        }

        println!("secondloop end");//表示されない
    }
}
