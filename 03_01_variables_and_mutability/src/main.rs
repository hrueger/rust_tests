const ANSWER: u32 = 42;

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in the inner scope: {x}");
    }

    println!("Value of x in the outer scope: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
}
