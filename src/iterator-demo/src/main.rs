use std::collections::HashMap;

fn main() {
    test_sim_for();
    test_iter();
    test_consumer();
}

fn test_sim_for() {
    let values = vec![1, 2, 3];
    let result = match values.into_iter() {
        mut iter => loop {
            match iter.next() {
                Some(x) => print!("{x}\t"),
                None => break,
            }
        },
    };
}

/*
1. into_iter 所有权
2. iter 借用
3. iter_mut 可变借用
*/
fn test_iter() {
    let values = vec![1, 2, 3];
    for value in values.into_iter() {
        println!("{value}");
    }
    // print!("{:?}", values);  // error!

    let values = vec![1, 2, 3];
    for value in values.iter() {
        println!("{value}");
    }
    println!("{:?}", values); // no error

    let mut values = vec![1, 2, 3];
    let mut values_iter_mut = values.iter_mut();
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", values);
}

/*
1. 消费者适配器： [...] => val  夺走迭代器所有权
2. 迭代者适配器： [...] => [...]
*/
fn test_consumer() {
    // consumer iter
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    println!("{:?}", v1); // ok
                          // println!("{:?}", v1_iter); // error

    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // another demo
    let names = ["aaa", "bbb"];
    let ages = [10, 20];
    let folks: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", folks);
}
