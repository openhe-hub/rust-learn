mod front_of_house; // 从与模块同名的文件中加载模块
mod test;

// std
use std::fmt::Result;
use std::io::Result as IoResult; // alias
use std::collections::*; // use * to import all

// third party
use rand::Rng;

// self
// pub use会将导入的模块同样导入当前的父模块
pub use crate::front_of_house::hosting; // 使用绝对路径引用hosting
use test::test_host;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // absolute
    hosting::add_to_waitlist(); // relative
    test_host::test_host();

    let _secret_num = rand::thread_rng().gen_range(1..101);
    let _dict: HashMap<String, i32> = HashMap::new();
}

// fn func1() -> Result {}
// fn func2() -> IoResult<()> {}
