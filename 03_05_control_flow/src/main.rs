fn main() {
    let x = 5;
    let y = 9;
    if x > y {
        println!("{x} is greater than {y}");
    }
    let verb = if x > y { "greater" } else { "less" };

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 1_000_000_000 {
            break counter * 2;
        }
    };
    println!("Result: {counter}");

    more_counting();
    while_loop();
    iterating();
    range_countdown();
    let fahrenheit = 90.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}° Fahrenheit = {celsius}° Celsius");
    let fib35 = fib(35);
    println!("The 35th number of the Fibonacci sequencce is {fib35}");
}

fn more_counting() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn iterating() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn range_countdown() {
    for i in (1..4).rev() {
        println!("{i}!");
    }
    println!("Boom!");
}

// Test Programs //
fn fahrenheit_to_celsius(input: f32) -> f32 {
    (input - 32.0) * 5.0 / 9.0
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
