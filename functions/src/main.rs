fn main() {
    hello();
    print_this("Hello, world!");
}

fn hello() {
    println!("Hello, world!");
}

fn print_this(s: &str) {
    println!("{}", s);
}
