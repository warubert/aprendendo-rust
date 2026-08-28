pub mod emoji_mappings;

/// Returns the emoji corresponding to the given name.
///
/// # Arguments
///
/// * `text` - A string slice that holds the sentence to convert
///
/// # Returns
///
/// A new `String` with matching words replaced with emojis
/// 
/// # Examples
/// 
/// ```
/// 
/// use text_to_emoji::convert_to_emojis;
/// 
/// let result = convert_to_emojis("smile");
/// assert_eq!(result, "😊");
///
/// ```
pub fn convert_to_emojis(text: &str) -> String {
    let emoji_map = emoji_mappings::get_emoji_map();

    text.split_whitespace()
        .map(|word| emoji_map.get(word).unwrap_or(&word).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_emojis() {
        let result = convert_to_emojis("smile");
        assert_eq!(result, "😊");
    }

    #[test]
    fn test_no_emoji() {
        let result = convert_to_emojis("hello");
        assert_eq!(result, "hello");
    }
}