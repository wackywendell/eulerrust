/// Functions for use in euler projects

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

use std::collections::TrieSet;

use std::collections::HashMap;

pub mod primesieve;

/// Count the number of occurrences of each value in an iterator
pub fn counter<K : std::hash::Hash + Eq, I : Iterator<K>>(mut list : I) -> HashMap<K, uint> {
	let mut counter : HashMap<K, uint> = HashMap::new();
	for key in list {
		counter.insert_or_update_with(key, 1, |_, v| {*v += 1});
	}
	counter
}

pub fn isqrt(n : uint) -> Option<uint> {
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

#[test]
fn test_square(){
	assert_eq!(isqrt(4), Some(2));
	assert_eq!(isqrt(5), None);
	
	let mut ntests = vec![1,7,8,9,10,11,12,189654,4294967295];
	
	for _ in range(0u, 1000u){
		let mut n = std::rand::random::<uint>();
		n = n % std::num::pow(2u, std::uint::BITS / 2);
		ntests.push(n);
	}
	
	for &n in ntests.iter(){
		println!("n: {}, n*n: {}", n, n*n);
		assert!(n < std::num::pow(2u, std::uint::BITS / 2));
		assert_eq!(isqrt(n*n), Some(n));
		if n > 1 {
			assert_eq!(isqrt(n*n - 1), None);
		} else {
			assert_eq!(isqrt(n*n - 1), Some(0));
		}
		assert_eq!(isqrt(n*n + 1), None);
	}
}
