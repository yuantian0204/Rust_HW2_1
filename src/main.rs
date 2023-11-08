use std::f64::consts::PI;

struct Shape {
    shape_type: String,
    parameters: [f64; 3],
}

impl Shape {
    pub fn new(shape_type: &str, parameters: &[f64]) -> Option<Shape> {
        match shape_type {
            "triangle" => Some(Shape {
                shape_type: shape_type.to_string(),
                parameters: [parameters[0], parameters[1], parameters[2]],
            }),
            "rectangle" => Some(Shape {
                shape_type: shape_type.to_string(),
                parameters: [parameters[0], parameters[1], 0.0],
            }),
            "circle" => Some(Shape {
                shape_type: shape_type.to_string(),
                parameters: [parameters[0], 0.0, 0.0],
            }),
            _ => None,
        }
    }

    pub fn perimeter(&self) -> f64 {
        match self.shape_type.as_str() {
            "triangle" => self.parameters[0] + self.parameters[1] + self.parameters[2],
            "rectangle" => 2.0 * (self.parameters[0] + self.parameters[1]),
            "circle" => 2.0 * PI * self.parameters[0],
            _ => 0.0,
        }
    }

    pub fn area(&self) -> f64 {
        match self.shape_type.as_str() {
            "triangle" => {
                let s = self.perimeter() / 2.0;
                (s * (s - self.parameters[0]) * (s - self.parameters[1])
                    * (s - self.parameters[2])).sqrt()
            }
            "rectangle" => self.parameters[0] * self.parameters[1],
            "circle" => PI * self.parameters[0] * self.parameters[0],
            _ => 0.0,
        }
    }

    pub fn double_perimeter(&mut self) {
        self.parameters[0] *= 2.0;
        self.parameters[1] *= 2.0;
        self.parameters[2] *= 2.0;
    }

    pub fn is_valid(&self) -> bool {
        match self.shape_type.as_str() {
            "triangle" => {
                self.parameters[0] > 0.0 && self.parameters[1] > 0.0 && self.parameters[2] > 0.0
                    && self.parameters[0] + self.parameters[1] > self.parameters[2]
                    && self.parameters[0] + self.parameters[2] > self.parameters[1]
                    && self.parameters[1] + self.parameters[2] > self.parameters[0]
            }
            "rectangle" => self.parameters[0] > 0.0 && self.parameters[1] > 0.0,
            "circle" => self.parameters[0] > 0.0,
            _ => false,
        }
    }
}

fn main() {
    let triangle = Shape::new("triangle", &[2.0, 3.0, 4.0]).unwrap();
    let rectangle = Shape::new("rectangle", &[2.0, 3.0]).unwrap();
    let circle = Shape::new("circle", &[2.0]).unwrap();
    let mut shapes = vec![triangle, rectangle, circle];
    for shape in shapes.iter_mut() {
        println!("{}, Perimeter: {}, Area: {}", shape.shape_type, shape.perimeter(), shape.area());
        shape.double_perimeter();
    }
    for shape in shapes {
        println!("{}, Perimeter: {}, Area: {}", shape.shape_type, shape.perimeter(), shape.area());
    }
    let triangle = Shape::new("triangle", &[1.0, 3.0, 3.0]).unwrap();
    println!("This triangle is valid: {}", triangle.is_valid());
}