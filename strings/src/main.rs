fn main() {

    // string slice
    let s = "hello";

    let s2: String = String::from("Hello");
    let s3: String = "hello".to_string();

    let mut empty = String::new();
    empty.push_str("not anymore");
    println!("var empty is now: {empty}");

    let mut some_string: String = String::from("foo");
    let s2 = "bar";
    some_string.push_str(s2);
    println!("s2 is {s2}");
    let s3 = some_string + s2;
    // println!("can i print some_string? {some_string}"); // won't compile, add takes ownership of some_string
    
}
