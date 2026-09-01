// Meu smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; //Associated type that defines the type that will be returned when dereferencing the smart pointer.

    fn deref(&self) -> &T {
        &self.0
    }
}

//Deref Coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let w = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *w);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
