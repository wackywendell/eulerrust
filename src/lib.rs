/// Functions for use in euler projects


#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate primes;
extern crate rand;

use std::collections::HashSet;

use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub use primes::PrimeSet;

const U64_BITS : usize = 64;

/// Count the number of occurrences of each value in an iterator
pub fn counter<K, I>(list : I) -> HashMap<K, u64>
	where 	K : Eq + std::hash::Hash,
			I : Iterator<Item=K>
{
	let mut counter : HashMap<K, u64> = HashMap::new();
	for key in list {
		match counter.entry(key) {
			Entry::Vacant(entry) => {entry.insert(1);},
			Entry::Occupied(entry) => {(*entry.into_mut()) += 1;}
		}
	}
	counter
}

pub fn isqrt_opt(n : u64) -> Option<u64> {
	if n <= 1 {return Some(n);}
	let mut x = n / 2;
	while x > 2u64.pow((U64_BITS / 2) as u32){
		// Prevents overflows
		x = (x + n / x + 1) / 2;
	}
	let mut usedset = HashSet::new();
	
	while x*x != n {
		usedset.insert(x);
		x = (x + n / x + 1) / 2;
		if usedset.contains(&x) {
			return None;
		}
	}
	Some(x)
}

pub fn isqrt(n : u64) -> u64 {
	if n <= 1 { return n; }
	let mut x = n / 2;
	while x > 2u64.pow((U64_BITS / 2) as u32){
		// Prevents overflows
		x = (x + n / x + 1) / 2;
	}
	
	while x*x != n {
		let lastx = x;
		x = (x + n / x + 1) / 2;
		println!("new x: {}", x);
		if x == lastx {
			println!("inside if: {}", x);
			if x*x < n {
				return x;
			} else {
				return x-1;
			}
		}
		println!("if passed: {}", x);
	}
	return x
}

pub fn is_palindrome(n : u64) -> bool {
	let s = n.to_string();
	//let s_rev = String::from_utf8(s.into_bytes().iter().rev().collect());
	let s_rev = s.as_bytes().iter().rev();
	let s_rev_vec : Vec<u8> = s_rev.map(|&b| b).collect();
	let s_rev = String::from_utf8(s_rev_vec);
	match s_rev {
		Ok(s2) => {s == s2},
		_ => false
	}
}

#[derive(Clone)]
pub struct Pairs<'a, T :'a>{
	vector : &'a [T],
	first : usize,
	second : usize
}

impl<'a, T> Iterator for Pairs<'a, T> {
	type Item=(&'a T, &'a T);
	fn next(&mut self) -> Option<(&'a T, &'a T)> {
		let l = self.vector.len();
		if self.second >= (l-1) && self.first >= (l-2) {return None;}
		self.second += 1;
		
		if self.second >= l {
			self.first += 1;
			self.second = self.first + 1;
		}
		
		return unsafe { Some((self.vector.get_unchecked(self.first), self.vector.get_unchecked(self.second)))}
	}
}

pub fn pairs<'a, T>(vec : &'a [T]) -> Pairs<'a, T>{
	Pairs {
		vector : vec,
		first : 0,
		second : 0
	}
}

#[test]
fn test_square(){
	assert_eq!(isqrt_opt(4), Some(2));
	assert_eq!(isqrt_opt(5), None);
	
	let mut ntests = vec![1,7,8,9,10,11,12,189654,4294967295];
	
	for _ in 0u64..1000 {
		let mut n = rand::random::<u64>();
		n = n % 2u64.pow((U64_BITS / 2) as u32);
		ntests.push(n);
	}
	
	for &n in ntests.iter(){
		println!("n: {}, n*n: {}", n, n*n);
		assert!(n < 2u64.pow((U64_BITS / 2) as u32));
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
	let mut ntests : Vec<u64> = vec![1,7,8,9,10,11,12,189654,4294967295];
	
	for i in 1u64..1001 {
		let mut n = rand::random::<u64>();
		n = n % 2u64.pow((U64_BITS / 2) as u32);
		ntests.push(n);
		ntests.push(i);
	}
	
	for n in ntests {
		assert!(n < 2u64.pow((U64_BITS / 2) as u32));
		let x : u64 = isqrt(n);
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

#[test]
fn test_pairs(){
	let v = [1,2,5,4u64];
	let mut my_pairs = pairs(&v);
	
	assert_eq!(my_pairs.next(), Some((&v[0], &v[1])));
	assert_eq!(my_pairs.next(), Some((&v[0], &v[2])));
	assert_eq!(my_pairs.next(), Some((&v[0], &v[3])));
	assert_eq!(my_pairs.next(), Some((&v[1], &v[2])));
	assert_eq!(my_pairs.next(), Some((&v[1], &v[3])));
	assert_eq!(my_pairs.next(), Some((&v[2], &v[3])));
	//~ assert_eq!(my_pairs.next(), Some((&1, &5)));
	//~ assert_eq!(my_pairs.next(), Some((&1, &4)));
	//~ assert_eq!(my_pairs.next(), Some((&2, &5)));
	//~ assert_eq!(my_pairs.next(), Some((&2, &4)));
	//~ assert_eq!(my_pairs.next(), Some((&5, &4)));
	assert_eq!(my_pairs.next(), None);
	assert_eq!(my_pairs.next(), None);
}
