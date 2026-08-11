fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}
fn main() {
    let good_string = "42";
    let result = parse_number(good_string);

    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse number: {}", e),
    }

    let bad_string = "not a number";
    let result = parse_number(bad_string);

    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse number: {}", e),
    }
}
