fn main() {
    let m1: String = String::from("Hello");
    let m2: String = String::from("world");
    greet(m1, m2);
    // println!("{m1}, {m2}"); // won't compile, m1 and m2 were borrowed
    let g1: String = String::from("Hello");
    let g2: String = String::from("world");
    greet_that_dont_borrow(&g1, &g2);
    println!("{g1}, {g2}");
    let s = format!("{g1} {g2}");

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;
    println!("x is {x}");
    println!("a is {a}");
    let r1: &Box<i32> = &x; // r1 is a reference to x, value is 2 (not copied)
    let b: i32 = **r1; // b is 2, we dereference r1 twice to get the value of x from heap

    // this below won't compile, need to use dereference operator i32::abs(*num1) or dot notation num1.abs()
    // let num1: Box<i32> = Box::new(-1);
    // let num1_abs = i32::abs(num1);
    // println!("num1 value is {num1} and num1_abs value is {num1_abs}");
    let num1: Box<i32> = Box::new(-1);
    let num1_abs = num1.abs();
    println!("num1 value is {num1} and absolute value is {num1_abs}");

    let some_string = String::from("Hello, world!");
    let str_len = str::len(&some_string);
    let str_len2 = some_string.len();

    let mut vec: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &vec[0];
    vec.push(4);
    println!("vec is {:?}", vec);
    // println!("zeroth element is {}", *num); //won't compile, num is a reference to vec[0] and vec[0] was moved somewhere else in memory, vec had to be recreated
}

fn greet(s1: String, s2: String){
    println!("{s1}, {s2}");
}

fn greet_that_dont_borrow(s1: &String, s2: &String) {
    println!("{s1}, {s2}");
}
