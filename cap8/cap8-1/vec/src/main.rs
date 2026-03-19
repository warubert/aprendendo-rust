
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2];

    println!("The third element is {:?}", third);
    let third: Option<&i32> = v.get(2);

    println!("The third element is {:?}", third);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("There is no third element"),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("The row is {:?}", row);
}
