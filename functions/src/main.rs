fn main() {
    hello();
    print_this("Hello, world!!!");
    // print_this(); // won't compile

    println!("sum(1, 2) = {}", sum(1, 2));
}

fn hello() {
    println!("Hello, world!");
}

fn print_this(s: &str) {
    println!("{}", s);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
