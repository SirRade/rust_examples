#[derive(PartialEq, PartialOrd)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new (x: f64, y: f64, radius: f64) -> Circle {
        Circle{
            x: x, 
            y: y, 
            radius: radius
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

struct CircleBuilder {
    x: f64,
    y: f64,
    radius: f64,
}

impl CircleBuilder {
    fn new() -> CircleBuilder {
        CircleBuilder {x: 0.0, y: 0.0, radius: 1.0}
    }

    fn x(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut CircleBuilder {
        self.y = coordinate;
        self
    }

    fn radius(&mut self, radius: f64) -> &mut CircleBuilder {
        self.radius = radius;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.x, y: self.y, radius: self.radius}
    }
}


fn main() {
    let circle0 = Circle::new(1.0, 2.0, 12.0);
    let circle1 = Circle{x: 1.0, y: 2.0, radius: 12.0};
    let circle2 = CircleBuilder::new()
                    .x(1.0)
                    .y(2.0)
                    .radius(12.0)
                    .finalize();    
    println!("all circles are equal: {}", circle0 == circle1 && circle1 == circle2);
    println!("area is {:?}", circle0.area());
}
