pub fn capitalize_words(words: &str) -> String {
    words.split_whitespace().map(|word| {
        let mut chars = word.chars();
        match chars.next() {
            Some(first_char) => {
                first_char.to_uppercase().collect::<String>() + chars.as_str()
            }
            None => String::new(),
        }
    }).collect::<Vec<String>>().join(" ")
}