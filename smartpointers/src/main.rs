use crate::List::{Cons, Nil};

// recursive types with boxes
//(1, (2, (3, Nil)))
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // create a box that points to an integer on the heap
    let b = Box::new(5);

    println!("b = {}", b);

    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
