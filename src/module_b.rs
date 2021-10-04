pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[ test ]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_ne!(0, add(0, 1));
}