fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x={}, y={}", x, tup.1);

    let str = String::from("hello world");
    dbg!(str_length(str));
}

fn str_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
