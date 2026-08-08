#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

fn main() {
    let new_user = User {
        id: 1,
        username: "admin".to_string()
    };

    println!("{:?}", new_user);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user() {
        let new_user = User {
            id: 1,
            username: "admin".to_string()
        };
        assert_eq!(new_user.id, 1);
        assert_eq!(new_user.username, "admin");
    }
}
