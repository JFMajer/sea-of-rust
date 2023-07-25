fn main() {
    let x: bool = true;
    read(x);

    let a = Box::new([0; 1_000_000]);
    let b = a; // b is now the owner of the array

    let first: String = String::from("Ferris");
    let full = add_suffix(first);
    println!("full name: {}", full);
    // println!("first name: {}", first); // won't compile, ownership was moved to first and deallocated


    // below code won't compile
    // let s = String::from("hello");
    // let s2;
    // let c = false;
    // if c {
    //   s2 = s;
    // }
    // println!("{}", s);
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
    else {
        println!("y is not true");
    }
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
