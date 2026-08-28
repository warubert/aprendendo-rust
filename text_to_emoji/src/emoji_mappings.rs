use std::collections::HashMap;

pub fn get_emoji_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("smile", "😊");
    map.insert("heart", "❤️");
    map.insert("thumbs_up", "👍");
    map.insert("star", "⭐");
    map.insert("fire", "🔥");
    // Add more mappings as needed
    map
}