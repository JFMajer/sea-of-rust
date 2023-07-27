fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser123@gmail.com"),
        sign_in_count: 1,
    };

    user1.sign_in_count = 2; // user1 needs to be mutable
    let user2 = build_user(String::from("test@test.com"), String::from("testuser"));
    println!("{}: {}", user2.username, user2.email);

    let user3 = User {
        email: String::from("test123@o2.pl"),
        ..user2 // copy the rest of the fields from user2
    };
    println!("{}: {}", user3.username, user3.email);

    let mut a = Point { x: 1, y: 2 };
    a.x += 1;
    let b = Point { y: 1, ..a };
    a.x += 1;
    println!("{}", b.x); // 2 because b is a shallow copy of a
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Vector(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

struct Point {
    x: i32,
    y: i32,
}
