fn main() {
    println!("Hello, world!");
    another_function();
    display_values(10, 11);

    let x = five();
    display_value(x);
}

fn another_function() {
    println!("Another function!");
}

fn display_value(x: i32){
    println!("The value of x is: {}", x);
}

fn display_values(x: i32, y: i32){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32{
    5
}
