fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];
    // v2.push(4); won't work, v2 is immutable

    let mut v3 = vec![1, 2, 4];
    v3.push(3);
    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element in v3 is {third}"),
        None => println!("element not found"),
    }

    for ele in &v3 {
        println!("{ele}");
    }

    let mut v4 = vec![33, 66, 101];
    for ele in &mut v4 {
        *ele += 50;
    }

    for ele in &v4 {
        println!("{ele}");
    }

    let some_v = vec![String::from("Hello! ")];
    println!("{:?}", some_v);
    // let mut s1 = some_v[0]; won't compile
}
