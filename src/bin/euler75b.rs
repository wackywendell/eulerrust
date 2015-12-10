/// Solution to Euler #75, this time using the formula (v^2-u^2, 2uv, v^2+u^2)
/// Much faster than the other way!
#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate num;
extern crate eulerrust;

use num::integer::gcd;
use std::collections::BTreeSet;

// Calculate "basic" Pythagorean triplets (excluding those with a common factor)
// Max_length is the maximum of a + b + c
fn make_triplets(max_length: u64) -> BTreeSet<(u64, u64, u64)> {
    let mut s: BTreeSet<(u64, u64, u64)> = BTreeSet::new();
    for v in 1..max_length {
        let vsq = v * v;
        if vsq >= max_length {
            break;
        }
        for u in 1..v {
            if gcd(u, v) > 1 {
                continue;
            };
            if u % 2 == v % 2 {
                continue;
            };
            if 2 * vsq + 2 * u * v > max_length {
                break;
            }
            let usq = u * u;
            let (a, b, c) = (2 * u * v, vsq - usq, vsq + usq);
            let (a, b) = if a > b {
                (b, a)
            } else {
                (a, b)
            };
            let gcd1 = gcd(a, b);
            let gcd2 = gcd(b, c);
            let gcd3 = gcd(gcd1, gcd2);
            assert_eq!(gcd3, 1);

            s.insert((a, b, c));
        }
    }

    s
}

// Calculate all Pythagorean triplets
fn all_triplets(max_length: u64) -> BTreeSet<(u64, u64, u64)> {
    let primitives = make_triplets(max_length);
    let mut s = BTreeSet::new();
    for &(a, b, c) in primitives.iter() {
        for n in 1..((max_length as u64) / c) {
            let (an, bn, cn) = (a * n, b * n, c * n);
            if an + bn + cn > max_length {
                continue;
            }
            s.insert((an, bn, cn));
        }
    }

    s
}

#[test]
fn test_make_triplets() {
    let v = vec![(3, 4, 5),
                 (5, 12, 13),
                 (7, 24, 25),
                 (8, 15, 17),
                 (9, 40, 41),
                 (12, 35, 37),
                 (20, 21, 29)];

    let mut s0: BTreeSet<(u64, u64, u64)> = BTreeSet::new();
    for &tup in v.iter() {
        s0.insert(tup);
    }

    let s = make_triplets(120);

    assert_eq!(s, s0);
}

#[test]
fn test_all_triplets() {
    let v = vec![(3, 4, 5),
                 (5, 12, 13),
                 (6, 8, 10),
                 (7, 24, 25),
                 (8, 15, 17),
                 (9, 12, 15),
                 (9, 40, 41),
                 (10, 24, 26),
                 (12, 16, 20),
                 (12, 35, 37),
                 (14, 48, 50),
                 (15, 20, 25),
                 (15, 36, 39),
                 (16, 30, 34),
                 (18, 24, 30),
                 (20, 21, 29),
                 (20, 48, 52),
                 (21, 28, 35),
                 (24, 32, 40),
                 (24, 45, 51),
                 (27, 36, 45),
                 (30, 40, 50)];

    let mut s0: BTreeSet<(u64, u64, u64)> = BTreeSet::new();
    for &tup in v.iter() {
        s0.insert(tup);
    }

    let s = all_triplets(120);

    assert_eq!(s, s0);
}

pub fn main() {
    let s = make_triplets(120);
    println!("{:?}", s);

    let s = all_triplets(120);
    println!("{:?}", s);


    let ls = s.iter().map(|&(m, l, n)| l + m + n).collect::<Vec<u64>>();
    let counts = eulerrust::counter(ls.iter());

    let len1 = counts.iter().filter(|&(_, &v)| v == 1).count();

    println!("{}", len1);
}
