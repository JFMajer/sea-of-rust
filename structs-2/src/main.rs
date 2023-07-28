fn main() {
    let height1: u32 = 50;
    let width1: u32 = 30;
    println!("Area of a rectangle with height {height1} and width {width1} is {}", rectangle_area(height1, width1));

    // using tuples
    let rect1 = (50, 30);
    println!("Area of a rectangle with dimensions: {} and {} is {}", rect1.0, rect1.1, area_tup(rect1));

    // using structs
    let rect2 = Rectangle {
        width: 30,
        height: 50
    };

    println!("Area of rectangle with height {} and width {} equals to {}.", rect2.height, rect2. width, area_struct(&rect2));

    // println!("Rectangle struct is {}", rect2); //won't compile
    // println!("Rectangle struct is {:?}", rect2); //again won't compile, the trait `Debug` is not implemented for `Rectangle`
    println!("Rectangle struct is {:?}", rect2); //works because of added #[derive(Debug)]
    println!("Rectangle struct is {:#?}", rect2); // prettier output
}

fn rectangle_area(height: u32, width: u32) -> u32 {
    height * width
}

fn area_tup(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//using structs
#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
