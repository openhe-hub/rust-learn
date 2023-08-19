/* closure grammar
 |param1: type, param2, ... | -> type {
    xxx;
    xxx;
    ret
 }
*/

/*  三种Fn特征
1. FnOnce 闭包捕获变量的所有权，该闭包只能运行一次
2. FnMut 闭包捕获变量的&mut
3. Fn 闭包捕获变量的&
*/

/*
1. move |...| {...} 捕获变量为引用
2. let mut closure = |...| {...} 捕获变量为可变引用
*/

struct Cacher<T, E>
where
    T: Fn(u32) -> E,
    E: Copy,
{
    query: T,
    value: Option<E>,
}

impl<T, E> Cacher<T, E>
where
    T: Fn(u32) -> E,
    E: Copy,
{
    fn new(query: T) -> Cacher<T, E> {
        Cacher {
            query: query,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> E {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn exec<'a, F>(mut f: F)
where
    F: FnMut(&'a str),
{
    f("hello");
}

fn main() {
    // fn once
    let x = vec![1, 2, 3];
    fn_once(|z| z == x.len());
    // fn mut
    let mut s = String::new();
    let update_str = |str| s.push_str(str);
    exec(update_str);
    println!("s = {s}");
}
