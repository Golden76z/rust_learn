fn main() {
    // Variables and mutations
    // Make this variable mutable so we can make modifications to it
    // let x = 5;
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The moddified value of x is {x}");

    // Constant declaration example
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is {THREE_HOURS_IN_SECONDS}");

    // Shadowing and inner scope
    let y = 5;

    // Shadowing the previous value
    let y = y + 1;

    // Inner scope
    {
        let y = y * 2;
        println!("The value of y in the inner scope is {y}");
    }
    println!("The value of y is {y}");
}
