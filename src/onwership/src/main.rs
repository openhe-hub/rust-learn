// three types of handling memory
// 1. GC: Java
// 2. manage by programmers: cpp
// 3. onwership: Rust

// ownership
// 1. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 2. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 3. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)

// 转移所有权
// 1. 移动或复制：堆内存转移后会被回收，栈内存（可Copy）转移后被认为是浅拷贝(ixx, uxx, fxx, bool, char, tuple, &T)
// 2. rust本身不使用深拷贝，若要使用可以用clone方法
// 这是因为复制堆内存消耗大，复制栈内存消耗小，所以对于栈内存或者字面量设置为可Copy

// 引用与借用
// Rust 通过 借用(Borrowing) 这个概念来达成上述的目的，获取变量的引用，称之为借用(borrowing)
// 引用默认是不可变的，可变引用&mut T
// 同一作用域，特定数据，只能有一个可变引用，且不可变引用不能和可变引用共存

fn main() {
    let s=String::from("hello world");
    take_onwership(s);
    let x=5;
    make_copy(x);
    // println!("{}",s); error!
    println!("{}",x);

    let a=5; 
    let b=&a;
    assert_eq!(5,a);
    assert_eq!(5,*b);

    let mut s2=String::from("hello Rust!");
    take_onwership2(&mut s2);
    println!("{}",s2); // no error
}

fn take_onwership(str: String) {
    println!("str={}", str);
}

fn take_onwership2(str: &mut String ){
    str.push_str("string");
    println!("str={}",str);
}

fn make_copy(num: i32) {
    println!("num={}", num);
}
