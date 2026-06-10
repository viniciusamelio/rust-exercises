#[test]
fn test_get_post_at() {
    vectors::create_post("1".to_string(), "Title".to_string(), "Content".to_string());
    vectors::create_post(
        "2".to_string(),
        "Title 2".to_string(),
        "Content 2".to_string(),
    );
    assert_eq!(
        vectors::get_post_at(0),
        Some(vectors::post::Post::new(
            "1".to_string(),
            "Title".to_string(),
            "Content".to_string()
        ))
    );
    assert_eq!(
        vectors::get_post_at(1),
        Some(vectors::post::Post::new(
            "2".to_string(),
            "Title 2".to_string(),
            "Content 2".to_string()
        ))
    );
    assert_eq!(vectors::get_post_at(2), None);
}
