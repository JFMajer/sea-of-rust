fn main() {
    let mut s1: String = String::from("hello world");

    let hello: &str = &s1[0..5];
    let world: &str = &s1[6..11];
    let s2: &String = &s1;

    // &str indicates a string slice

    let mut s2: String = String::from("hello!");
    let hello: &str = &s2[0..6];
    println!("{hello}");
    s2.push_str(" world");
    println!("{s2}");

    let word = first_word(&s1);
    // s1.clear(); //won't compile because of the immutable reference to s1
    println!("{word}");

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );

    let test_string: String = String::from("hello rust");
    consume_string(test_string);
    // println!("{test_string}"); // won't compile because test_string was moved into consume_string
}

// this function takes a string and returns the first word it finds in that string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn consume_string(s: String) {
    println!("om nom nom nom");
}
