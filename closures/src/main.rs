fn main() {
    let closure = || "Hello, world!";

    println!("{}", closure());

    let add = |x: i32, y: i32| x + y;

    println!("{}", add(1, 2));

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    let add2 = | a, b | a + b;

    println!("{}", add2(hello, &world));

    let x = 50;

    let print_x = || println!("{}", x);

    print_x();

    let mut y= 100;

    let mut print_y = || {
        y += 1;
        println!("{}", y);
    };

    print_y();

    let z = String::from("Hello");

    let print_z = move || {
        println!("{}", z);
        drop(z);
    };

    print_z();

    let double = |x: i32| x * 2;

    apply(double);
}

fn apply<F>(f: F) where F: Fn(i32) -> i32 {
    println!("{}", f(10));
}
