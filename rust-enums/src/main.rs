enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let my_shape = Shape::Circle(5.0);
    let area = calculate_area(my_shape);
    println!("{}", area);
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(a) => a * a,
        Shape::Square(a) => a * a,
    };
    return area;
}
