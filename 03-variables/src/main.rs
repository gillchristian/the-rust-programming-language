fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // shadowing:
    let x = x + 7;
    println!("The value of x is: {}", x);
    // we can even change types when shadowing
    let x = "8";
    println!("The value of x is: {}", x);

    // integers
    let x = 100_000;
    println!("The value of x is: {}", x);
    let x = 0xff;
    println!("The value of x is: {}", x);
    let x = 0o77;
    println!("The value of x is: {}", x);
    let x = 0b10;
    println!("The value of x is: {}", x);
    let x = b'A';
    println!("The value of x is: {}", x);
    let x = b'A';
    println!("The value of x is: {}", x);

    // floats
    let x = 2.0; // f64
    println!("The value of x is: {}", x);
    let x: f32 = 3.0; // f32
    println!("The value of x is: {}", x);

    // chars
    let x = 'z';
    println!("The value of x is: {}", x);
    let x = '@';
    println!("The value of x is: {}", x);
    let x = '😻';
    println!("The value of x is: {}", x);

    // tuples
    let x = ("a", "b", "c");
    println!("The value of x is: {:?}", x);
    println!("The value of x.0 is: {}", x.0);
    println!("The value of x.1 is: {}", x.1);
    println!("The value of x.2 is: {}", x.2);
    let (x, y, z) = x;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    // arrays
    let x = [1, 2, 3, 4, 5];
    println!("The value of x is: {:?}", x);
    println!("The value of x[0] is: {}", x[0]);
}
