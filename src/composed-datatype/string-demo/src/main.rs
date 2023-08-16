// 1. 字符串字面量是切片
// 2. &str转String: String::from("xxx")  
//    String转&str: &s
fn main() {
    let mut s = String::from("hello world");
    let (hello, world) = (&s[0..5], &s[6..11]);
    println!("hello={}, world={}", hello, world);
    println!("str={}", &s);

    // 1. push
    s.push('a');
    println!("str={}", &s);

    // 2. push_str
    s.push_str("aaa");
    println!("str={}", &s);

    // 3. insert & insert_str
    // 4. replace
    // 5. pop
    let lst_char = s.pop();
    dbg!(lst_char);
    // 6. remove
    // 7. clear
}
