fn main() {
    let mut x = 5;
    println!("The valueof x is: {}", x);
    x = 5;
    println!("The valueof x is: {}", x);

    let _signed:i8 = 1;
    let _unsigned:u8 = 1;

    let _x = 2.0; // f64
    let _y : f32 = 3.0;

    let tup: (i32, f64, u8) = (500, 4.6, 1); //tuple

    println!("this is a tuple ({}, {}, {})", tup.0, tup.1, tup.2);

    let _arr = [1,2,3,4,5];
    let a: [i32; 5] = [1,2,3,4,5];

    println!("Addressing array index 0: {}", a[0]);



}
