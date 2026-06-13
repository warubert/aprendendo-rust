trait Display {
    fn display(&self) -> String;
}

struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn display(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

struct Circle {
    x: i32,
    y: i32,
    radius: u32,
}

impl Display for Circle {
    fn display(&self) -> String {
        format!("Circle({}, {}, {})", self.x, self.y, self.radius)
    }
}

fn print_display_single_with_impl(item: &impl Display) {
    println!("{}", item.display());
}

fn print_display_multiple_with_dyn(items: &[&dyn Display]) {
    for item in items {
        println!("{}", item.display());
    }
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let circle = Circle { x: 3, y: 4, radius: 5 };

    print_display_single_with_impl(&point);
    print_display_single_with_impl(&circle);

    let shapes: [&dyn Display; 2] = [&point, &circle];
    print_display_multiple_with_dyn(&shapes);
}
