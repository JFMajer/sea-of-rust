fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x * 2;
        println!("The value of x in inner scope is: {x}");
    }

    println!("The value of x in outer scope is: {x}");

    let spaces = "          ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    let f1 = 34.5; // float64
    let f2: f32 = 34.0; // float32

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    // format float
    println!("The value of quotient is: {:.2}", quotient);

    // remainder
    let _remainder = 43 % 5;

    let _t = true;

    let f: bool = false; // with explicit type annotation

    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // tuples
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let something_else = tup.2;
    println!("The value of something_else is: {something_else}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ]; // array is better than vector if you know the size

    // printing all elements in array
    for month in months.iter() {
        println!("The value of month is: {month}");
    }

    let arr1 = [3; 5]; // [3, 3, 3, 3, 3]

    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{}", a[0] + t.1[0]); // 4
    println!("a[0] value is: {}", a[0]); // 1
    println!("t.1[0] value is: {}", t.1[0]); // 3
}
