/**
Efficient Exponentiation
Problem 122
https://projecteuler.net/problem=122



The most naive way of computing n^15 requires fourteen Multiplications:

n × n × ... × n = n^15

But using a "binary" method you can compute it in six Multiplications:

n × n = n^2
n^2 × n^2 = n^4
n^4 × n^4 = n^8
n^8 × n^4 = n^12
n^12 × n^2 = n^14
n^14 × n = n^15

However it is yet possible to compute it in only five Multiplications:

n × n = n^2
n^2 × n = n^3
n^3 × n^3 = n^6
n^6 × n^6 = n^12
n^12 × n^3 = n^15

We shall define m(k) to be the minimum number of Multiplications to compute n^k; for example m(15) = 5.

For 1 ≤ k ≤ 200, find ∑ m(k).

----------------------------------------------------------------------------------------------------

This code is a bit messy, and does things it doesn't need to - it took me a while to figure things out.

Basically, it just follows the tree to every possibility < n, and stops at a max level of the tree.

**/

extern crate eulerrust;
use std::collections::{HashMap,VecDeque};

pub struct Multiplications {
	sum : u64,
	path : Vec<(u64, u64, u64)>, // x,y,z where n^x = n^y * n^z
	increasing : bool
}

impl Multiplications {
	pub fn contains(&self, n : u64) -> bool {
		let mut f = self.path.iter().filter(|&&(nsum, _, _)| {n == nsum});
		match f.next() {
			Some(_) => true,
			None => false
		}
	}
	
	pub fn append(&self, n1 : u64, n2 : u64) -> Multiplications {
		let mut newpath = self.path.clone();
		newpath.push((n1 + n2, n1, n2));
		let increasing = match self.path.last() {
			None => true,
			Some(&(_, _, m2)) if m2 > n2 => false,
			Some(&(_, _, m2)) if m2 == n2 => self.increasing,
			Some(_) => true,
			
		};
		Multiplications {
			sum : n1 + n2,
			path : newpath,
			increasing : increasing
		}
	}
}

fn find_ms(n : u64, max_depth : Option<usize>) -> HashMap<u64, Vec<(u64,u64,u64)>> {
	let mut tmap = HashMap::new();
	let mut queue = VecDeque::new();
	
	let first = Multiplications {
		sum : 1,
		path : vec![(1,0,0)],
		increasing : true
	};
	
	tmap.insert(first.sum, first.path.clone());
	
	queue.push_back(first);
	
	let maxn = (n as f32).log2().ceil() as usize * 2;
	
	loop {
		let last = match queue.pop_front() {
			Some(newk1) => newk1,
			None => {return tmap;}
		};
		
		//~ println!("------------------------------");
		//~ println!("{} : {}", last.sum, last.path);
		
		let n1 = last.sum;
		for &(n2, _, _) in last.path.iter() {
			if n1 + n2 > n { continue;}
			if !last.increasing {
				//~ match last.path.last() {
					//~ None => panic!("This should be impossible."),
					//~ Some(&(m12, m1, m2)) if m2 < n2 => {
						//~ println!("CONTINUING: {} :: {}", last.path, (n1,n2));
						//~ continue;
					//~ }
					//~ _ => {}
				//~ }
			};
			// let maxn = ((n1 + n2) as f32).log2().ceil() as u64 * 2;
			if last.path.len() >= maxn {continue;}
			match max_depth {
				Some(d) if d < last.path.len() => {continue;}
				_ => {}
			}
			let new_m = last.append(n1,n2);
			//~ println!("  =>  {} : {}", new_m.sum, new_m.path);
			//~ match tmap.find_mut(&new_m.sum) {
				//~ None => {tmap.insert(new_m.sum, new_m.path.clone());},
				//~ Some(old) => { if old.len() > new_m.path.len() {*old = new_m.path};}
			//~ }
			
			let should_insert = match tmap.get_mut(&new_m.sum) {
				None => true,
				Some(old) => {
						if old.len() > new_m.path.len() {
							//~ if new_m.sum == 37 {
								//~ println!("  =>  {} : {}", new_m.sum, new_m.path);
								//~ println!("      replacing {}", old);
							//~ }
							*old = new_m.path.clone()
						//~ } else if new_m.sum == 37 {
							//~ println!("  =>  {} : {}", new_m.sum, new_m.path);
							//~ println!("      NOT replacing {}", old);
						};
						false
					}
			};
			
			if should_insert {
				//~ if new_m.sum == 37 {
					//~ println!("  =>  {} : {}", new_m.sum, new_m.path);
					//~ println!("      New!");
				//~ }
				tmap.insert(new_m.sum, new_m.path.clone());
			};
			
			if new_m.sum < n {queue.push_front(new_m);};
		}
	}
}

#[test]
fn test_find_ms() {
	let ms = find_ms(15, Some(11));
	let v = match ms.get(&15){
		None => panic!("15 NOT FOUND"),
		Some(v2) => v2
	};
	
	assert_eq!(v.len() - 1, 5);
}

pub fn main(){
	let bigm = 200;
	let ms = find_ms(bigm, Some(11)); // 11 found by trial and error; 10 doesn't get everything, 11 works, so take 11...
	
	let mut msum = 0;
	for k in 1..(bigm+1){
		let v = match ms.get(&k){
			None => panic!("NOT FOUND: {}", k),
			Some(v2) => v2
		};
	//~ for (k, v) in ms.iter() {
		println!("{} :: {} {:?}", k, v.len() - 1, v);
		msum += v.len() - 1;
	}
	println!("================================================================================");
	println!("Final sum: {} {}", msum, ms.len());
}
