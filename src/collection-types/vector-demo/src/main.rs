use std::fmt::Debug;

fn display<T: Debug>(vec: &Vec<T>) {
    println!("{:?}", vec);
}

#[derive(Debug)]
struct User {
    username: String,
    age: u32,
    password: String,
}

impl User {
    fn new(username: String, age: u32, password: String) -> User {
        User {
            username: username,
            age: age,
            password: password,
        }
    }
}

fn main() {
    // create
    let mut v = Vec::new();
    v.push(1);
    display(&v);
    println!("v[0] = {}", &v[0]);
    // get
    match v.get(2) {
        Some(num) => println!("v[2] = {}", num),
        None => println!("null"),
    }
    // loop visit
    let vec2 = vec![1, 2, 3];
    for i in &vec2 {
        println!("{}", i);
    }

    let mut vec3 = vec![1, 2, 3, 4, 5];
    for i in &mut vec3 {
        *i += 10
    }
    display(&vec3);
    // sort
    let mut users = vec![
        User::new("aaa".to_string(), 10, "aaa".to_string()),
        User::new("bbb".to_string(), 5, "aaa".to_string()),
        User::new("ccc".to_string(), 20, "aaa".to_string()),
    ];
    users.sort_by(|a, b| b.age.cmp(&a.age));
    display(&users);
}
