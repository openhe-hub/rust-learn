fn main() {
    // === variable ===
    let x = 5;
    println!("x={}", x);
    // x=6; error the default var is unchangeable

    let mut y = 8;
    println!("y={}", y);
    y = 10;
    println!("y={}", y);

    // === deconstruction ===
    let (a, mut b) = (true, false);
    println!("a={},b={}", a, b);
    b = true;
    assert_eq!(a, b);

    // === use list/tuple/struct in deconstruction ===
    let (x, y, z, i, j);
    (x, y) = (1, 2); // tuple
    [z, .., i, _] = [1, 2, 3, 4, 5];
    struct Struct {
        j: i32,
    }
    Struct { j, .. } = Struct { j: 5 };
    assert_eq!([1, 2, 1, 4, 5], [x, y, z, i, j]);

    // === constant ===
    // const is constant value during compiling, let is constant during running period
    const MAX_VAL: u32 = 100_000;
    println!("{}", MAX_VAL);
}
