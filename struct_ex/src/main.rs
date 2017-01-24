fn main() {
    println!("Hello, world!");

    let mut origin = Point { x: 0, y: 0};  // origin: Point

    println!{"The origin is at ({}, {})", origin.x, origin.y};

    origin.x = 5;
    println!{"The origin is at ({}, {})", origin.x, origin.y};

    let origin = origin;
    println!{"The origin is at ({}, {})", origin.x, origin.y};

    let mut space_point = Point3d { x: 0, y: 0, z: 0 };
    println!("origin: space_point is ({}, {}, {})", space_point.x, space_point.y, space_point.z);
    space_point = Point3d {y: 1, .. space_point};
    println!("after use include '..', space_point is ({}, {}, {})", space_point.x, space_point.y, space_point.z);

    // tuple struct with one element
    let length = Inches(10);

    let Inches(integer_length1) = length;

    let integer_length2 = length.0;

    println!("integer_length1 is {}", integer_length1);
    println!("integer_length2 is {}", integer_length2);
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

// one element tuple struct example as below
struct Inches(i32);


