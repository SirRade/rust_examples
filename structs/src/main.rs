struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}


fn main() {
    let mut point = Point { x: 3, y: 4 };
    point.x = 5;
    println!("The point is at ({}, {})", point.x, point.y);

    let origin = Point3d { x: 0, y: 0, z: 0 };
    println!("origin:\tX: {:?}, Y: {:?}, Z: {:?}", origin.x, origin.y, origin.z);
    let point = Point3d { z: 1, .. origin };
    println!("point:\tX: {:?}, Y: {:?}, Z: {:?}", point.x, point.y, point.z);

    struct TupleStructColor(i32, i32, i32);
    struct TupleStructPoint(i32, i32, i32);

    let black = TupleStructColor(0, 0, 0);
    let origin = TupleStructPoint(0, 0, 0);
    println!("black values: {:?}, {:?}, {:?}", black.0, black.1, black.2);
    let TupleStructPoint(x, y, z) = origin;
    println!("origin values: {:?}, {:?}, {:?}", x, y, z);
    // Doesn't work, different types 
    // Despite exactly same fields!
    // origin = black;

    // Ignoring x
    let TupleStructPoint(_, origin_y, origin_z) = origin;

    // a so called newtype
    struct Inches(i32);
    let length = Inches(10);

    // Doesn't work, needs tuple unpacking
    // let as_i32: i32 = length;

    let Inches(integer_length) = length;
    // same thing as
    let integer_length = length.0;

    println!("length is {} inches", integer_length);

    // Unit like struct
    struct Electron {}
    struct Proton;

    let electron = Electron{};
    let proton = Proton;
}
