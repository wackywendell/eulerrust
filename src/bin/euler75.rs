#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate eulerrust;

use std::collections::HashMap;

//~ fn make_triplets(max_length : u64) -> Vec<(u64, u64, u64)> {
	//~ let mut v = Vec::new();
	//~ for n in range(3, (max_length + 1) * 2 / 3) {
		//~ for m in range(2,n) {
			//~ if n + m >= max_length {
				//~ break
			//~ }
			//~ let (nsq, msq) = (n*n, m*m);
			//~ let sqsum = nsq + msq;
			//~ match eulerrust::isqrt(sqsum) {
				//~ Some(l) if l+m+n <= max_length => { v.push((m,n,l)) }
			    //~ _ => { continue;}
			//~ }
		//~ }
	//~ }
	//~
	//~ v
//~ }

fn make_triplets(max_length : u64) -> Vec<(u64, u64, u64)> {
	let mut s = HashMap::new();
	for n in 0..max_length {
		let nsq = n*n;
		s.insert(nsq, n);
	}
	
	if max_length > 10000 {
		println!("Made set.");
	}
	
	
	let mut v = Vec::new();
	for n in 3..(((max_length as f64) / (2.0f64).sqrt()).ceil() as u64) {
		let nsq = n*n;
		// lower lim: ((n as f64)*2.0 +1.0).sqrt().floor() as u64
		for m in 2..n {
			let msq = m*m;
			if n + m > max_length { break;}
			match s.get(&(nsq + msq)) {
				Some(&l) if l+m+n <= max_length => { v.push((m,n,l)); }
				Some(_) => break,
				_ => continue
			}
		}
	}
	v
}

#[test]
fn simple_test(){
	let v = make_triplets(120);
	
	println!("{:?}", v);
	
	let ls : Vec<u64> = v.iter()
		.map(|&(m,l,n)| {l+m+n})
		.collect::<Vec<u64>>();
	
	println!("{:?}", ls);
	
	let lens20 = ls.iter()
		.filter(|&&length|{length == 20})
		.count();
	
	assert_eq!(lens20, 0);
	
	let lens120 = ls.iter()
		.filter(|&&length|{length == 120})
		.count();
	
	assert_eq!(lens120, 3);
}

pub fn main() {
	//println!("7: {}", eulerrust::is_square(7));
	
	let v = make_triplets(2000);
	
	println!("Made triplets: {}", v.len());
	
	let ls = v.iter().map(|&(m,l,n)| {l+m+n}).collect::<Vec<u64>>();
	
	let counts = eulerrust::counter(ls.iter());
	
	let len1 = counts.iter().filter(|&(_, &v)| {v == 1}).count();
	
	//println!("{}", counts);
	
	println!("{}", len1);
	
	
}
