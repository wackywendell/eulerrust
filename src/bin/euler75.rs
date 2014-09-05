#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unnecessary_qualification)]
#[warn(non_uppercase_statics)]
#[warn(missing_doc)]

extern crate eulerrust;

use std::collections::TrieMap;

//~ fn make_triplets(max_length : uint) -> Vec<(uint, uint, uint)> {
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

fn make_triplets(max_length : uint) -> Vec<(uint, uint, uint)> {
	let mut s = TrieMap::new();
	for n in range(0, max_length) {
		let nsq = n*n;
		s.insert(nsq, n);
	}
	
	if max_length > 10000 {
		println!("Made set.");
	}
	
	
	let mut v = Vec::new();
	for n in range(3, ((max_length as f64) / (2.0f64).sqrt()).ceil() as uint) {
		let nsq = n*n;
		// lower lim: ((n as f64)*2.0 +1.0).sqrt().floor() as uint
		for m in range(2, n) {
			let msq = m*m;
			if n + m > max_length { break;}
			match s.find(&(nsq + msq)) {
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
	
	println!("{}", v);
	
	let ls : Vec<uint> = v.iter()
		.map(|&(m,l,n)| {l+m+n})
		.collect::<Vec<uint>>();
	
	println!("{}", ls);
	
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
	
	let ls = v.iter().map(|&(m,l,n)| {l+m+n}).collect::<Vec<uint>>();
	
	let counts = eulerrust::counter(ls.iter());
	
	let len1 = counts.iter().filter(|&(_, &v)| {v == 1}).count();
	
	//println!("{}", counts);
	
	println!("{}", len1);
	
	
}
