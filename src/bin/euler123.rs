/**
Prime square remainders
Problem 123

Let p_n be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when (p_n−1)^n + (p_n+1)^n is divided by p_n^2.

For example, when n = 3, p_3 = 5, and 43 + 63 = 280 ≡ 5 mod 25.

The least value of n for which the remainder first exceeds 109 is 7037.

Find the least value of n for which the remainder first exceeds 1010.

----------------------------------------------------------------------------------------------------
Note, learned in thread 120:

(a+1)^n == an+1 (mod a^2), and (a-1)^n == an-1 or 1-an (mod a^2) depending whether n is odd or even;
the sum is therefore either 2an (n odd) or 2 (n even).
**/

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate eulerrust;

fn get_rem(a : uint, n : uint) -> uint{
	if n % 2 == 0 {
		2
	} else {
		(2*a*n) % (a*a)
	}
}

#[test]
fn test_123(){
	let mut pset = eulerrust::PrimeSet::new();
	let (mut n, mut r) = (0,0);
	while r < 1000*1000*1000 {
		n += 1;
		let p = *pset.get(&n);
		r = get_rem(p, n);
	}
	assert_eq!(n, 7037);
}

pub fn main() {
	let mut pset = eulerrust::PrimeSet::new();
	println!("{}", pset.len());
	let p = *pset.get(&7037);
	println!("p_n: {}", p);
	println!("rem: {}", get_rem(p, 7037));
	
	let (mut n, mut p, mut r) = (0u,0u,0u);
	while r < 10*1000*1000*1000 {
		n += 1;
		p = *pset.get(&n);
		r = get_rem(p, n);
	}
	println!("n: {}, p: {}, r: {}", n, p, r);
	// Prints:
	// n: 21033, p: 237733, r: 10000476378
}
