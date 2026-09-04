#[allow(dead_code)]
use std::rc::Rc;
use List::{Const, Nil};

enum List {
    Const(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Const(5, Rc::new(Const(10, Rc::new(Nil))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Const(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Const(4, Rc::clone(&a));
        println!("count inside block = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
