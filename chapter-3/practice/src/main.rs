use std::io;

fn main() {
    let mut response = String::new();

    println!("What would you like to do?");
    display_menu();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line!");

    response = response.trim().to_string();

    if response == "1" {
        let input = get_input_as_float();
        let value = to_celsius(input);
        println!("{} fahrenheit -> {} celcius.", input, value);
    } else if response == "2" {
        let input = get_input_as_float();
        let value = to_fahrenheit(input);
        println!("{} celcius -> {} fahrenheit.", input, value);
    } else if response == "3" {
        let input = get_input_as_int();
        let value = fibonacci(input);
        println!("fibonacci({}) = {}", input, value);
    } else {
        println!("Not an option!");
    }
}

fn fibonacci(x: i32) -> i32{
    if x <= 0{
        return 0;
    }else if x == 1 {
        return 1;
    }

    return fibonacci(x - 2) + fibonacci(x - 1)
}

fn get_input_as_int() -> i32 {
    let parsed_value;
    loop {
        let mut response = String::new();
        println!("Enter a number.");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line!");
        parsed_value = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    parsed_value
}

fn get_input_as_float() -> f64 {
    let parsed_value;
    loop {
        let mut response = String::new();
        println!("Enter a number.");
        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line!");
        parsed_value = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    parsed_value
}

fn to_fahrenheit(cel: f64) -> f64 {
    cel * 9.0 / 5.0 + 32.0
}

fn to_celsius(fahr: f64) -> f64 {
    fahr - 32.0 * 5.0 / 9.0
}

fn display_menu() {
    println!("1. Convert to Celsius.");
    println!("2. Convert to Fahrenheit.");
    println!("3. Compute the nth Fibonacci number.");
    println!("4. Hear 'The Twelve Days of Christmas.'");
}
