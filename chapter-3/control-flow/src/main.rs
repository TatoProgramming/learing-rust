fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was true.");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    example_loop();
    for_each();
    reverse_range();
}

fn example_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn for_each() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the Value is: {}", element);
    }
}

fn reverse_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
