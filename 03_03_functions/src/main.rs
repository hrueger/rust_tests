fn main() {
    println!("Hello, world!");

    another_function(5, 's');
    let f = five();
    println!("Five: {f}");
}

fn another_function(x: i32, unit: char) {
    println!("Value: {x}{unit}");
}

fn five() -> i32 {
    5
}
