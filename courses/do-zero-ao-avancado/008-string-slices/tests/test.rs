#[test]
fn test_slice_phrase_with_length() {
    let phrase = "Hello, World!";
    let result = string_slices::slice_phrase(phrase, 5);
    assert_eq!(result, "Hello");
}

#[test]
fn test_slice_phrase_with_length_0() {
    let phrase = "Hello, World!";
    let result = string_slices::slice_phrase(phrase, 0);
    assert_eq!(result, "");
}

#[test]
fn test_slice_phrase_with_length_1() {
    let phrase = "Hello, World!";
    let result = string_slices::slice_phrase(phrase, 1);
    assert_eq!(result, "H");
}
