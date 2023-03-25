use std::num;

// fn function_name(param: param_type) -> return_type{}
// use snake case
fn my_max(num1: i32, num2: i32) -> i32{
    if num1>num2 {num1} else {num2}  // return can be omitted
}

// void will return ()

// diverge function: never return
fn dead_end() -> ! {
  panic!("dead");
}

fn main() {
   println!("max(1,2)={}",my_max(1, 2));
   dead_end();
}
