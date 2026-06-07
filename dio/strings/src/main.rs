fn main() {
    let mut s1: String = String::from("Hello");
    s1 += " World";

    let s2: String = s1.clone();

    println!("String s1: {}, - referencia {:p}", s1, &s1);
    println!("String s2: {}, - referencia {:p}", s2, &s2);

    let s3: &str = "Hello World";
    let s4: String = format!("{} - teste", s3);

    println!("String s3: {}, - referencia {:p}", s3, &s3);
    println!("String s4: {}, - referencia {:p}", s4, &s4);
}
