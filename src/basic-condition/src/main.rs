fn main() {
    // if-else
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("num = {}", num);
    // for
    for i in 1..=5 {
        println!("for i = {}", i);
    }
    for i in 1..5 {
        println!("for i = {}", i);
    }
    // for in
    let a = [1, 2, 3, 4];
    for (idx, val) in a.iter().enumerate() {
        println!("a[{}] = {}", idx, val);
    }

    for item in &a {
        println!("{}", item);
    } // 使用&container避免转移所有权，使用&mut container对容器遍历修改

    // loop & while
    let mut cnt = 0;
    loop {
        println!("looping...");
        cnt += 1;
        if cnt >= 5 {
            break;
        }
    }
}
