fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v1 = vec![10, 20, 30];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&20));
    assert_eq!(v1_iter.next(), Some(&30));
    assert_eq!(v1_iter.next(), None);

    /* 
    1. iter()
    Purpose: Creates an iterator that borrows each element of the collection immutably.
    Ownership: The iterator borrows the collection, so the original collection remains accessible after iteration.
    Use Case: When you want to read elements without modifying them.
    */

    let numbers = vec![1, 2, 3];

    for number in numbers.iter() {
        println!("Got: {}", number);
    }

    println!("iter");
    println!("The vector is: {:?}", numbers);

    /* 
    2. iter_mut()
    Purpose: Creates an iterator that borrows each element of the collection mutably.
    Ownership: The iterator borrows the collection mutably, allowing you to modify the elements in place.
    Use Case: When you want to modify elements of the collection during iteration.
    */

    let mut numbers = vec![1, 2, 3];

    for number in numbers.iter_mut() {
        *number += 1;
        println!("Got: {}", number);
    }

    println!("iter_mut");
    println!("The vector is: {:?}", numbers);

    /* 
    3. into_iter()
    Purpose: Creates an iterator that takes ownership of the collection and yields its elements by value.
    Ownership: The iterator takes ownership of the collection, so the original collection is no longer accessible after iteration.
    Use Case: When you want to consume the collection and possibly transform its elements.
    */

    let numbers = vec![1, 2, 3];

    for number in numbers.into_iter() {
        println!("Got: {}", number);
    }

    println!("into_iter");
    // println!("The vector is: {:?}", numbers); // This line would cause a compile error

    /*
    Summary Table:
    | Method      | Purpose                                    | Ownership                         | Use Case                             |
    |-------------|--------------------------------------------|-----------------------------------|--------------------------------------|
    | iter()      | Borrow elements immutably                  | Borrows the collection            | Read elements without modifying them |
    | iter_mut()  | Borrow elements mutably                    | Borrows the collection mutably    | Modify elements in place             |
    | into_iter() | Take ownership and yield elements by value | Takes ownership of the collection | Consume and transform elements       |
    */

    // metodos que modificam ou consumem iterators

    let numbers = vec![1, 2, 3, 4, 5];

    //Map
    let squares: Vec<_> = numbers
        .iter()
        .map(|&x| x * x)
        .collect();

    println!("Squares: {:?}", squares);

    //Filter
    let even_numbers: Vec<_> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .collect();

    println!("Even numbers: {:?}", even_numbers);

    //Fold -> Reduce
    let sum: i32 = numbers
        .iter()
        .fold(0, |acc, &x| acc + x);

    println!("Sum: {}", sum);
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
