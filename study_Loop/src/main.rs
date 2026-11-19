const LOOPLIMIT: i32 = 20;

struct Iter {
    current: usize,
    max : usize,
}

impl Iterator for Iter {
    type Item = usize;

    //next（）メゾッドの実装
    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current -1 < self.max {
            Some(self.current - 1)
        }else {
            None
        }
    }
}

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

    //Range型
    for number in  1..5 {
        println!("{}",number);
    }


    //Iteratorお試し
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it{
        println!("{}", num);
    }
}
