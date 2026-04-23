//! Integration tests — generated 2026-04-23

#[test]
fn test_arithmetic() {
    assert_eq!(2 + 2, 4);
    assert_eq!(10 / 2, 5);
    assert_eq!(3_i32.pow(2), 9);
}

#[test]
fn test_string_operations() {
    let s = "hello world".to_string();
    assert!(s.contains("hello"));
    assert_eq!(s.len(), 11);
    assert_eq!(s.to_uppercase(), "HELLO WORLD");
}

#[test]
fn test_vector_operations() {
    let mut v: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();
    assert_eq!(v[0], 1);
    assert_eq!(*v.iter().max().unwrap(), 9);
    let evens: Vec<&i32> = v.iter().filter(|&&x| x % 2 == 0).collect();
    assert!(!evens.is_empty());
}

#[test]
fn test_option_handling() {
    let value: Option<i32> = Some(42);
    assert!(value.is_some());
    assert_eq!(value.unwrap(), 42);
    let none: Option<i32> = None;
    assert_eq!(none.unwrap_or(0), 0);
}
