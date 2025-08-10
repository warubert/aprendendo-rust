fn main() {
    println!("Hello, world!");

    // This is a imutable reference to a vector (com aliasing)
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    v.push(4);


    // This is a mutable reference to a vector (without aliasing)
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
}
