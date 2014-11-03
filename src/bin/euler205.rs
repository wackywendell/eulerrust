/**
Problem 205

Peter has nine four-sided (pyramidal) dice, each with faces numbered 1, 2, 3, 4.
Colin has six six-sided (cubic) dice, each with faces numbered 1, 2, 3, 4, 5, 6.

Peter and Colin roll their dice and compare totals: the highest total wins. The result is a draw if the totals are equal.

What is the probability that Pyramidal Pete beats Cubic Colin? Give your answer rounded to seven decimal places in the form 0.abcdefg
**/

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

extern crate eulerrust;
use std::collections::TrieMap;

fn multiply_rolls(die : &Vec<uint>, last : &TrieMap<uint>) -> TrieMap<uint>{
	let mut t = TrieMap::new();
	for (r, &count) in last.iter(){
		for &s in die.iter(){
			let prev = t.find(&(r+s)).map(|&v|{v}).unwrap_or(0u);
			t.insert(r+s, prev + count);
		}
	}
	t
}

fn get_rolls(dice : Vec<Vec<uint>>) -> TrieMap<uint>{
	let mut t = TrieMap::new();
	t.insert(0, 1);
	for die in dice.iter(){
		t = multiply_rolls(die, &t);
	}
	t
}

#[test]
fn test_get_rolls(){
	let v = vec![(2u, 1u), (3, 2), (4, 3), (5, 4), (6, 5),
			(7, 6), (8, 5), (9, 4), (10, 3), (11, 2), (12, 1)];
	let t : TrieMap<uint> = get_rolls(vec![vec![1u,2,3,4,5,6], vec![1,2,3,4,5,6]]);
	let rolls : Vec<(uint, uint)> = t.iter().map(|(a, &b)|{(a,b)}).collect();
	
	assert_eq!(v, rolls);
	
	let v = vec![(12u, 1u), (13, 1), (14, 2), (15, 3), (16, 2), (17, 2), (18, 1)];
	let t : TrieMap<uint> = get_rolls(vec![vec![1,2], vec![3,5,6], vec![8,10]]);
	let rolls : Vec<(uint, uint)> = t.iter().map(|(a, &b)|{(a,b)}).collect();
	
	assert_eq!(v, rolls);
}

pub fn main(){
	let die : Vec<uint> = vec![1,2,3,4];
	let peter_dice : Vec<Vec<uint>> = Vec::from_fn(9, |_|{die.clone()});
	let peter_rolls = get_rolls(peter_dice);
	
	let die : Vec<uint> = vec![1,2,3,4,5,6];
	let colin_dice : Vec<Vec<uint>> = Vec::from_fn(6, |_|{die.clone()});
	let colin_rolls = get_rolls(colin_dice);
	
	let (mut loss, mut tie, mut win) = (0u, 0u, 0u);
	
	for (p_roll, &p_count) in peter_rolls.iter(){
		for (c_roll, &c_count) in colin_rolls.iter(){
			let count = p_count * c_count;
			match p_roll.cmp(&c_roll){
				Greater => {win += count;}
				Equal => {tie += count;}
				Less => {loss += count;}
			}
		}
	}
	
	println!("{}-{}-{}", loss, tie, win);
	
	println!("{:.7f}", (win as f64) / ((loss + tie + win) as f64))
	// Should print:
	// 4355187942-865512042-7009890480
	// 0.5731441
}
