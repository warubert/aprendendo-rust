fn main() {

    // create a box that points to an integer on the heap
    let b = Box::new(5);

    println!("b = {}", b);
}
