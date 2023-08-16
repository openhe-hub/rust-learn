use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1; 5];
    let c: [[i32; 5]; 2] = [[1; 5]; 2]; // 2-d
    println!("a={:#?}", a); // {:#?}格式化输出
    println!("b={:?}", b); // {:?}调试输出
    println!("c={:?}", c); // {:?}调试输出
    println!("a[0]={}", a[0]);
    println!("a[0-2]={:?}", &a[0..3]);

    println!("Please enter the index:");
    let mut index_str = String::new();

    io::stdin()
        .read_line(&mut index_str)
        .expect("Failed to read line");

    let index: usize = index_str
        .trim()
        .parse()
        .expect("Index entered is not a number");

    println!("a[{}] = {}", index, a[index]);
}
