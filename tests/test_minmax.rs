//! Runtime tests for minmax functions

use verus_proof_time_testing::minmax::*;

#[test]
fn test_max_x_basic() {
    assert_eq!(max_x(3, 5), 5);
    assert_eq!(max_x(10, 2), 10);
    assert_eq!(max_x(7, 7), 7);
}

#[test]
fn test_min_x_basic() {
    assert_eq!(min_x(3, 5), 3);
    assert_eq!(min_x(10, 2), 2);
    assert_eq!(min_x(7, 7), 7);
}

#[test]
fn test_max_x_edge_cases() {
    assert_eq!(max_x(0, 0), 0);
    assert_eq!(max_x(u32::MAX, 0), u32::MAX);
    assert_eq!(max_x(0, u32::MAX), u32::MAX);
}

#[test]
fn test_min_x_edge_cases() {
    assert_eq!(min_x(0, 0), 0);
    assert_eq!(min_x(u32::MAX, 0), 0);
    assert_eq!(min_x(0, u32::MAX), 0);
}

#[test]
fn test_max_x_commutative() {
    for a in [0, 1, 10, 100, u32::MAX] {
        for b in [0, 1, 10, 100, u32::MAX] {
            assert_eq!(max_x(a, b), max_x(b, a), "max_x({a}, {b}) != max_x({b}, {a})");
        }
    }
}

#[test]
fn test_min_x_commutative() {
    for a in [0, 1, 10, 100, u32::MAX] {
        for b in [0, 1, 10, 100, u32::MAX] {
            assert_eq!(min_x(a, b), min_x(b, a), "min_x({a}, {b}) != min_x({b}, {a})");
        }
    }
}

#[test]
fn test_max_x_ge_min_x() {
    for a in [0, 1, 10, 100, u32::MAX] {
        for b in [0, 1, 10, 100, u32::MAX] {
            assert!(max_x(a, b) >= min_x(a, b), "max_x({a}, {b}) < min_x({a}, {b})");
        }
    }
}
