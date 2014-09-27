/// Functions for use in euler projects

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

use std::collections::TrieSet;

use std::collections::HashMap;
use std::collections::hashmap::{Occupied, Vacant};

pub use primesieve::PrimeSet;

pub mod primesieve;

/// Count the number of occurrences of each value in an iterator
pub fn counter<K : std::hash::Hash + Eq, I : Iterator<K>>(mut list : I) -> HashMap<K, uint> {
	let mut counter : HashMap<K, uint> = HashMap::new();
	for key in list {
		match counter.entry(key) {
			Vacant(entry) => {entry.set(1);},
			Occupied(entry) => {(*entry.into_mut()) += 1;}
		}
	}
	counter
}

pub fn isqrt_opt(n : uint) -> Option<uint> {
	if n <= 1 {return Some(n);}
	let mut x = n / 2;
	let mut usedset = TrieSet::new();
	
	//println!("New x: {}", x);
	
	while x*x != n {
		usedset.insert(x);
		// let addx = x + n / x;
		x = (x + n / x + 1) / 2;
		//println!("New x: {} ({})", x, addx);
		if usedset.contains(&x) {
			//println!("Seen! {} ({}, {})", x, x*x, n);
			return None;
		}
	}
	Some(x)
}

pub fn isqrt(n : uint) -> uint {
	if n <= 1 { return n; }
	let mut x = n / 2;
	let mut usedset = TrieSet::new();
	
	while x*x != n {
		usedset.insert(x);
		let lastx = x;
		x = (x + n / x + 1) / 2;
		//if usedset.contains(&x) {
		if x == lastx {
			if x*x < n {
				return x;
			} else {
				return x-1;
			}
		}
	}
	return x
}

pub fn is_palindrome(n : uint) -> bool {
	let s = n.to_string();
	//let s_rev = String::from_utf8(s.into_bytes().iter().rev().collect());
	let s_rev = s.as_slice().as_bytes().iter().rev();
	let s_rev_vec : Vec<u8> = s_rev.map(|&b| b).collect();
	let s_rev = String::from_utf8(s_rev_vec);
	match s_rev {
		Ok(s2) => {s == s2},
		_ => false
	}
}

#[test]
fn test_square(){
	assert_eq!(isqrt_opt(4), Some(2));
	assert_eq!(isqrt_opt(5), None);
	
	let mut ntests = vec![1,7,8,9,10,11,12,189654,4294967295];
	
	for _ in range(0u, 1000u){
		let mut n = std::rand::random::<uint>();
		n = n % std::num::pow(2u, std::uint::BITS / 2);
		ntests.push(n);
	}
	
	for &n in ntests.iter(){
		println!("n: {}, n*n: {}", n, n*n);
		assert!(n < std::num::pow(2u, std::uint::BITS / 2));
		assert_eq!(isqrt_opt(n*n), Some(n));
		if n > 1 {
			assert_eq!(isqrt_opt(n*n - 1), None);
		} else {
			assert_eq!(isqrt_opt(n*n - 1), Some(0));
		}
		assert_eq!(isqrt_opt(n*n + 1), None);
	}
}

#[test]
fn test_isqrt(){
	assert_eq!(isqrt(0), 0);
	let mut ntests = vec![1,7,8,9,10,11,12,189654,4294967295];
	
	for i in range(1u, 1001u){
		let mut n = std::rand::random::<uint>();
		n = n % std::num::pow(2u, std::uint::BITS / 2);
		ntests.push(n);
		ntests.push(i);
	}
	
	for &n in ntests.iter() {
		assert!(n < std::num::pow(2u, std::uint::BITS / 2));
		let x = isqrt(n);
		assert!(x*x <= n);
		assert!((x+1)*(x+1) > n);
		assert_eq!(isqrt(n*n - 1), n-1);
		assert_eq!(isqrt(n*n + 1), n);
	}
}

#[test]
fn test_palindrome(){
	assert!(is_palindrome(1));
	assert!(is_palindrome(2));
	assert!(is_palindrome(8));
	assert!(!is_palindrome(10));
	assert!(!is_palindrome(12));
	assert!(!is_palindrome(100));
	assert!(!is_palindrome(11110));
	assert!(!is_palindrome(21111));
	assert!(!is_palindrome(12131));
	assert!(is_palindrome(11));
	assert!(is_palindrome(111));
	assert!(is_palindrome(232));
	assert!(is_palindrome(181));
}
