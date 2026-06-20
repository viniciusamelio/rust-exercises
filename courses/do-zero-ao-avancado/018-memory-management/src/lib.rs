// Lifetime reference is needed since compiler needs to know how long the reference is valid
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("a", "b"), "b");
        assert_eq!(longest("ab", "a"), "ab");
    }
}
