fn find_user(username: &str) -> Option<String> {
    if username == "alice" {
        Some("Alice".to_string())
    } else if username == "bob" {
        Some("Bob".to_string())
    } else {
        None
    }
}

fn main() {
    let user_to_find = "alice";
    let found_user = find_user(user_to_find);

    if found_user.is_some() {
        println!("Found user: {}", found_user.unwrap());
    } else {
        println!("User not found");
    }

    println!("----");
    let user_to_find = "charlie";
    let found_user = find_user(user_to_find);

    if found_user.is_some() {
        println!("Found user: {}", found_user.unwrap());
    } else {
        println!("User not found");
    }
}
