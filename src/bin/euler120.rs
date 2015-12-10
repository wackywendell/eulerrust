#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate eulerrust;
use std::collections::HashSet;

fn get_max_rem(a: u64) -> u64 {
    let mut set = HashSet::new();
    let asq = a * a;

    let mut alo = 1;
    let mut ahi = 1;

    for n in 1..(2 * asq) {
        alo *= a - 1;
        alo %= asq;
        ahi *= a + 1;
        ahi %= asq;

        if n % 2 == 0 {
            continue;
        }

        let r = (alo + ahi) % asq;
        // ~ println!("a: {}, n: {}, r: {}", a, n, r);
        if set.contains(&r) {
            break;
        }
        set.insert(r);
    }
    set.iter().fold(0, |a, &b| {
        if a > b {
            a
        } else {
            b
        }
    })
}

#[test]
fn test_max_rem() {
    assert_eq!(get_max_rem(7), 42);
}

#[test]
fn test_max_rem_long() {
    let mut sum = 0;
    for a in 3u64..1001u64 {
        sum += get_max_rem(a);
    }
    assert_eq!(sum, 333082500);
}

pub fn main() {
    let mut sum = 0;
    for a in 3u64..1001u64 {
        sum += get_max_rem(a);
    }

    println!("sum: {}", sum);
}
