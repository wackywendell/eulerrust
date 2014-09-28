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

**/

extern crate eulerrust;
use std::collections::TrieMap;
use std::collections::dlist::DList;
use std::collections::Deque;

fn merge_vecs<T : Eq + Clone>(v1 : &Vec<T>, v2 : &Vec<T>) -> Vec<T> {
	let mut v : Vec<T> = v1.clone();
	for val in v2.iter() {
		if v.contains(val) {continue;};
		v.push(val.clone());
	}
	
	v
}

pub struct Multiplications {
	sum : uint,
	path : Vec<(uint, uint, uint)> // x,y,z where n^x = n^y * n^z
}

impl Multiplications {
	pub fn contains(&self, n : uint) -> bool {
		let mut f = self.path.iter().filter(|&&(nsum, _, _)| {n == nsum});
		match f.next() {
			Some(_) => true,
			None => false
		}
	}
	
	pub fn append(&self, n1 : uint, n2 : uint) -> Multiplications {
		let mut newpath = self.path.clone();
		newpath.push((n1 + n2, n1, n2));
		Multiplications {
			sum : n1 + n2,
			path : newpath
		}
	}
}

fn find_ms(n : uint) -> TrieMap<Vec<(uint,uint,uint)>> {
	let mut tmap = TrieMap::new();
	let mut queue = DList::new();
	
	let first = Multiplications {
		sum : 1,
		path : vec![(1,0,0)]
	};
	
	tmap.insert(first.sum, first.path.clone());
	
	queue.push(first);
	
	loop {
		let last = match queue.pop_front() {
			Some(newk1) => newk1,
			None => {return tmap;}
		};
		
		//~ println!("------------------------------");
		//~ println!("{} : {}", last.sum, last.path);
		
		for (i1, &(n1, _, _)) in last.path.iter().enumerate() {
			for &(n2, _, _) in last.path.slice_from(i1).iter() {
				if n1 + n2 <= last.sum { continue; }
				if n1 + n2 > n { continue;}
				let maxn = ((n1 + n2) as f32).log2().ceil() as uint * 2;
				if last.path.len() >= maxn {continue;}
				let new_m = last.append(n1,n2);
				//~ println!("  =>  {} : {}", new_m.sum, new_m.path);
				//~ match tmap.find_mut(&new_m.sum) {
					//~ None => {tmap.insert(new_m.sum, new_m.path.clone());},
					//~ Some(old) => { if old.len() > new_m.path.len() {*old = new_m.path};}
				//~ }
				
				let should_insert = match tmap.find_mut(&new_m.sum) {
					None => true,
					Some(old) => {
							if old.len() > new_m.path.len() {
								println!("  =>  {} : {}", new_m.sum, new_m.path);
								println!("      replacing {}", old);
								*old = new_m.path.clone()
							};
							false
						}
				};
				
				if should_insert {
					println!("  =>  {} : {}", new_m.sum, new_m.path);
					println!("      New!");
					tmap.insert(new_m.sum, new_m.path.clone());
				};
				
				if new_m.sum < n {queue.push_front(new_m);};
			}
		}
	}
}


#[test]
fn test_merge_vecs(){
	let (v1, v2) = (vec!(1u,2,3), vec!(2,3,4,5));
	let v = merge_vecs(&v1, &v2);
	assert_eq!(v, vec!(1,2,3,4,5));
}

pub fn main(){
	let ms = find_ms(100);
	
	for (k, v) in ms.iter() {
		println!("{} :: {}", k, v.len() - 1);
	}
}
