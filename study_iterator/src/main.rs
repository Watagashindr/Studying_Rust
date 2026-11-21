
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

        if loopiter % 2 == 0 {
            println!("{:?}",loopiter);
        }

        if loopiter.to_string() == "None" ||  loopiter == 10 {
            break;
        }
    }


    


}
