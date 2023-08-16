fn main() {
    // 在 Rust 的 println! 宏中，{} 是用来格式化实现了 std::fmt::Display trait 的类型的，而 {:?} 则是用来格式化实现了 std::fmt::Debug trait 的类型的。
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: false,
        username: String::from("openhe"),
        email: String::from("xxx"),
        sign_in_count: 1,
    };

    println!("user1 = {:?}", user1);

    println!("username = {}", user1.username);

    let user2 = User {
        username : String::from("openhe2"),
        ..user1 // omit for the same data
    };

    println!("username = {}", user2.username);
}
