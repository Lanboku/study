fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;

    println!("The value of y is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of y is: {}", z);
    println!("The value of tup.0 is: {}", tup.0);
}
