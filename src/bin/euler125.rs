#![feature(step_by)]

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate eulerrust;
use std::collections::HashSet;

pub fn get_palindrome_sums(max: u64) -> HashSet<u64> {
    let mut set = HashSet::new();

    for i in 1u64..(eulerrust::isqrt(max) + 1) {
        let mut isq = 0;
        // println!("isq: {}", isq);
        for j in ((i as i64)..0i64).step_by(-1i64) {
            isq += (j as u64) * (j as u64);
            if isq > max {
                break;
            }
            if j == i as i64 {
                continue;
            }
            if eulerrust::is_palindrome(isq) {
                if max <= 1000 {
                    println!("{} - {} -> {}", j, i, isq);
                }
                set.insert(isq);
            }
        }
        if i * i > max {
            break;
        }
    }
    set
}

#[test]
fn test_euler125() {
    let s = get_palindrome_sums(1000);
    let ssum = s.iter().fold(0u64, |a, &b| a + b);
    assert_eq!(ssum, 4164);
}

pub fn main() {
    let s = get_palindrome_sums(100000000);

    println!("s len: {}", s.len());

    let ssum = s.iter().fold(0u64, |a, &b| a + b);
    println!("s sum: {}", ssum);
}
