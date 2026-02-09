pub fn normalize_hashtag(tag: &str) -> String {
    tag.trim()
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '#')
        .take(50)
        .collect()
}
