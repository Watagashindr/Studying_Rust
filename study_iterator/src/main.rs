
struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new(max: usize) -> Counter {
        return Counter {
            current: 0,
            max: max,
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        if self.current < self.max {
            let returnnum = self.current;
            self.current += 1;
            return Some(returnnum);
            
        }
        return None;
    }
}

fn sum_iterator(mut iter: impl Iterator<Item = i32>) -> i32 {

    let mut sum = 0;

    loop {
        let num = iter.next();

        match num {
            Some(value) => {
                sum += value;
            }
            None => {
                break;
            }
        }

    }

    return sum;
}


fn main() {

    let numbers = vec![10,20,30];
    let mut iter = numbers.iter();

    println!("{:?}", iter.next()); // Some(10)
    println!("{:?}", iter.next()); // Some(20)
    println!("{:?}", iter.next()); // Some(30)
    println!("{:?}", iter.next()); // None

    let mut iter2 = numbers.iter();

    let first = iter2.next();
    let second = iter2.next();
    let third = iter2.next();

    println!("{:?}", first);
    println!("{:?}", second);
    println!("{:?}", third);

    //練習

    let numbers2 = vec![100,200,300];
    let mut iteratortest = numbers2.iter();

    println!("{:?}",iteratortest.next());
    println!("{:?}",iteratortest.next());



    let mut looptest = numbers2.iter();

    loop {
        let mut loopiter = looptest.next();

        match loopiter {
            Some(value) => {
                if value % 2 == 0 {
                    println!("{}",value);
                }
            }
            None => {
                break;
            }
        }
    }


    let mut counter = Counter {
        current : 0,
        max: 5,
    };
    
    let numbers3 = vec![10, 20, 30, 40];
    let result3 = sum_iterator(numbers.into_iter());

    println!("{}", result3); // 100
}

