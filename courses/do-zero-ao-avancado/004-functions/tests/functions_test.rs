#[test]
fn lorem_ipsum() {
    assert_eq!(functions::lorem_ipsum(), "Lorem Ipsum");
}

#[test]
fn sum_test() {
    assert_eq!(functions::sum(2, 3), 5);
}

#[test]
fn sub_test() {
    assert_eq!(functions::sub(5, 3, 2), 0);
}
