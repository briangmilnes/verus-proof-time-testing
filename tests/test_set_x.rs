//! Runtime tests for SetX

use verus_proof_time_testing::set_x::*;
use verus_proof_time_testing::set_x_lit;

#[test]
fn test_empty() {
    let s: SetX<i32> = SetX::empty();
    assert_eq!(s.size(), 0);
}

#[test]
fn test_insert() {
    let mut s: SetX<i32> = SetX::empty();
    assert!(s.insert(1));
    assert!(s.insert(2));
    assert!(!s.insert(1)); // duplicate
    assert_eq!(s.size(), 2);
}

#[test]
fn test_mem() {
    let mut s: SetX<i32> = SetX::empty();
    s.insert(42);
    assert!(s.mem(&42));
    assert!(!s.mem(&99));
}

#[test]
fn test_iter() {
    let mut s: SetX<i32> = SetX::empty();
    s.insert(1);
    s.insert(2);
    s.insert(3);
    
    let mut count = 0;
    let mut sum = 0;
    let mut it = s.iter();
    while let Some(&x) = it.next() {
        count += 1;
        sum += x;
    }
    assert_eq!(count, 3);
    assert_eq!(sum, 6);
}

#[test]
fn test_iter_for_loop() {
    let mut s: SetX<i32> = SetX::empty();
    s.insert(10);
    s.insert(20);
    s.insert(30);
    
    let sum: i32 = s.iter().sum();
    assert_eq!(sum, 60);
}

#[test]
fn test_macro_empty() {
    let s: SetX<i32> = set_x_lit![];
    assert_eq!(s.size(), 0);
}

#[test]
fn test_macro_elements() {
    let s = set_x_lit![1, 2, 3];
    assert_eq!(s.size(), 3);
    assert!(s.mem(&1));
    assert!(s.mem(&2));
    assert!(s.mem(&3));
    assert!(!s.mem(&4));
}

#[test]
fn test_macro_duplicates() {
    let s = set_x_lit![1, 1, 2, 2, 3];
    assert_eq!(s.size(), 3); // duplicates ignored
}

