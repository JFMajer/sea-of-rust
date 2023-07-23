fn main() {
    println!("Hello, world!");
    println!("Number 3 is: {}", even_or_odd(3));

    let some_bool = true;
    let num1 = if some_bool { 50 } else { 100 };
    println!("num1 is: {}", num1);

    let mut counter: i32 = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter; // break the loop and return the value, value to be returned must be the same type as the loop and placed after the break keyword
        }
    };

    println!("result is: {result}");

    let test1 = [5; 10]; // array of 10 elements with value 5

    let mut countdown: i32 = 10;

    while countdown > 0 {
        println!("{countdown}");
        countdown -= 1
    }

    println!("Liftoff!");

    let arr1 = [1, 2, 4, 7, 10];

    for ele in arr1 {
        println!("Value is {ele}");
    }

    // better liftoff
    for count in (1..10).rev() {
        println!("{count}")
    }

    println!("Liftoff!");

    let temp1 = fahrenheit_to_celcius(32.0);
    println!("32F equals to {temp1}C");

    println!("0th fibo is: {}", generate_fibo(0));
    println!("1st fibo is: {}", generate_fibo(1));
    println!("15th fibo is: {}", generate_fibo(15));

}

// function that checks if ineteger is orr or even
fn even_or_odd(n: i32) -> &'static str {
    if n % 2 == 0 {
        "Even"
    } else {
        "Odd"
    }
}

fn fahrenheit_to_celcius(temp_in_f: f32) -> f32 {
    return (temp_in_f - 32.0) * (5.0/9.0)
}

fn generate_fibo(n: i32) -> i32 {
    if n == 0 {
        return 0
    }
    else if n == 1 {
        return 1
    }
    else {
        return generate_fibo(n-1) + generate_fibo(n-2)
    }
}


