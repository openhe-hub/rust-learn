use num::complex::Complex;
fn main() {
    //1. int: i8-i128
    //2. unsigned int: u8-128
    //3. float: f32, f64
    //4. range: 1..5 => 1~4, 1..=5 => 1~5
    for i in 1..5 {
        println!("{}", i)
    }

    for j in 1..=5 {
        println!("{}", j)
    }

    // 5. complex
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);

    // 6. char
    // 7. bool
    // 8. unit type ()
    // 9. expression
    let x = 1;
    let y = if x % 2 == 1 { "odd" } else { "even" }; // 3-op expr, last expr in field is return val
    println!("y={}",y)
}
