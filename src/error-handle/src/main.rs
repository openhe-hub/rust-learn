use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

// panic是程序直接终止的错误处理，可以栈展开或者直接终止
fn main() {
    test_result();
    test_unwrap();
    test_expect();
}

fn test_result() {
    let f = File::open("./assets/test.txt");
    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("./assets/test.txt") {
                Ok(file) => file,
                Err(_) => panic!("cannot create file."),
            },
            _ => panic!("Error: {:?}", err),
        },
    };
}

// get result directly, if fail then panic
fn test_unwrap() {
    let f = File::open("./assets/test.txt").unwrap();
}

// get result directly, if fail then print expect
fn test_expect() {
    let f = File::open("./assets/test.txt").expect("cannot open file");
}

fn test_panic() {
    let v = vec![1, 2, 3];
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let idx = buf.trim().parse::<usize>().unwrap();

    if idx >= v.len() {
        panic!("Err: out of range!");
    }
}

// error propaganda
// 使用?进行错误传播
fn read_username_from_file() -> Result<String, io::Error> {
    /*     let f = File::open("./assets/test.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    } */
    let mut s = String::new();
    File::open("./assets/test.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
