enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Dog {
    name: String,
    age: u8,
    breed: String,
    genus: String,
}

struct Cat {
    name: String,
    age: u8,
    breed: String,
    genus: String,
}

struct Bird {
    name: String,
    age: u8,
    breed: String,
    genus: String,
    lifespan: u8,
}

enum Pet {
    Dog(Dog),
    Cat(Cat),
    Bird(Bird),
}

// way to print Pet info
impl Pet {
    fn info(&self) {
        match self {
            Pet::Dog(d) => println!("Dog: {} is a {} year old {}. Genus is {}.", d.name, d.age, d.breed, d.genus),
            Pet::Cat(c) => println!("Cat: {} is a {} year old {}. Genus is {}.", c.name, c.age, c.breed, c.genus),
            Pet::Bird(b) => println!("Bird: {} is a {} year old {}. Genus is {}. Lifespan is {} years.", b.name, b.age, b.breed, b.genus, b.lifespan),
        }
    }
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let my_dog = Pet::Dog(Dog {
        name: String::from("Rusty"),
        age: 5,
        breed: String::from("Dachshund"),
        genus: String::from("Canis"),
    });

    let my_cat = Pet::Cat(Cat {
        name: String::from("Misty"),
        age: 3,
        breed: String::from("Tabby"),
        genus: String::from("Felis"),
    });

    let my_bird = Pet::Bird(Bird {
        name: String::from("Tweety"),
        age: 1,
        breed: String::from("Canary"),
        genus: String::from("Serinus"),
        lifespan: 15,
    });

    my_dog.info();
    my_cat.info();
    my_bird.info();

    let some_number: Option<i32> = Some(5);
    let another_number: Option<i16> = Some(6);
    let some_char = Some('a');
    let absent_number: Option<i32> = None;
    // won't compile, different types
    // let sum = some_number + another_number;
}
