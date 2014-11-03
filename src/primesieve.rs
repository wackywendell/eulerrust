/// A prime generator object, using Eratosthenes' sieve.

#[warn(non_camel_case_types)]
#[warn(non_snake_case)]
#[warn(unused_qualifications)]
#[warn(non_upper_case_globals)]
#[warn(missing_docs)]

/// A prime generator object, using Eratosthenes' sieve.

//use std::iter::Iterator;
use std::ops::Index;
use std::slice;
use std::iter;


fn sqrt_floor(n : uint) -> uint {
    (n as f64).sqrt().floor() as uint
}

/// A prime generator, using the Sieve of Eratosthenes.
pub struct PrimeSet {
    lst : Vec<uint>
}

/// An iterator over generated primes. Created by PrimeSet::iter or
/// PrimeSet::generator
pub struct PrimeSetIter<'a> {
    p : &'a mut PrimeSet,
    n : uint,
    expand : bool
}

impl PrimeSet {
    /// A new prime penerator, primed with 2 and 3
    pub fn new() -> PrimeSet {
        PrimeSet{lst:vec!(2,3)}
    }
    
    /// Finds one more prime, and adds it to the list
    pub fn expand(&mut self) {
        let mut l = self.lst[self.lst.len()-1] + 2;
        let mut sql = sqrt_floor(l);
        let mut remainder = 0;
        loop {
            for &n in self.lst.iter() {
                remainder = l % n;
                if remainder == 0 || n > sql {
                    break;
                }
            };
            
            if remainder != 0 {
                self.lst.push(l);
                break;
            };
            
            l += 2;
            sql = sqrt_floor(l);
        }
    }
    
    /// Number of primes found so far
    pub fn len(&self) -> uint {
        self.lst.len()
    }
    
    /// Return all primes found so far as a slice
    pub fn list<'a>(&'a self) -> &'a [uint] {
        self.lst.as_slice()
    }
    
    /// Iterator over all primes not yet found
    pub fn generator<'a>(&'a mut self) -> PrimeSetIter<'a> {
        let myn = self.len();
        PrimeSetIter{p:self, n:myn, expand:true}
    }
    
    /// Iterator over all primes, starting with 2
    pub fn iter<'a>(&'a mut self) -> PrimeSetIter<'a> {
        PrimeSetIter{p:self, n:0, expand:true}
    }
    
    //~ pub fn iter_once(&'self mut self) -> PrimeSetIter<'self> {
        //~ PrimeSetIter{p:self, n:0, expand:false}
    //~ }
    
    /// Iterator over just the primes found so far
    pub fn iter_vec<'a>(&'a self) -> slice::Items<'a, uint> {
        self.lst.iter()
    }
    
    /// Find the next largest prime from a number
    pub fn find(&mut self, n: uint) -> (uint, uint) {
        while n > *(self.lst.last().unwrap_or(&0)){
            self.expand();
        }
        self.find_vec(n).unwrap()
    }
    
    /// Check if a number is prime
    pub fn is_prime(&mut self, n: uint) -> bool {
        if n <= 1 {return false;}
        if n == 2 {return true;} // otherwise we get 2 % 2 == 0!
        for m in self.iter() {
            if n % m == 0 {return false;};
            if m*m > n {return true;};
        }
        panic!("This iterator should not be empty.");
    }
    
    /// Find the next largest prime from a number, if it is within the already-found list
    /// Returns (idx, prime)
    /// Note that if n is prime, then the output will be (idx, n)
    pub fn find_vec(&self, n: uint) -> Option<(uint, uint)> {
        if n > *(self.lst.last().unwrap_or(&0)){ return None;}
        
        let mut base : uint = 0;
        let mut lim : uint = self.len();

        while lim != 0 {
            let ix = base + (lim >> 1);
            match self.lst[ix].cmp(&n) {
                Equal => return Some((ix, self.lst[ix])),
                Less => {
                    base = ix + 1;
                    lim -= 1;
                }
                Greater => ()
            }
            lim >>= 1;
        }
        return Some((base, self.lst[base]));
    }
    
    pub fn get(&mut self, index : &uint) -> &uint {
		for _ in range(0, (*index as int) + 1 - (self.lst.len() as int)){
			self.expand();
		}
        self.lst.index(index)
	}
		
}

impl Index<uint, uint> for PrimeSet {
    fn index(&self, index: &uint) -> &uint {
        self.lst.index(index)
    }
}

impl<'a> Iterator<uint> for PrimeSetIter<'a> {
    fn next(&mut self) -> Option<uint> {
        while self.n >= self.p.len(){
            match self.expand {
                true => self.p.expand(),
                false => return None
            }
        }
        self.n += 1;
        
        let m = self.p.lst[self.n-1];
        
        Some(m)
    }
}

/// Find the first factor (other than 1) of a number
fn firstfac(x: uint) -> uint {
    let m = ((x as f64).sqrt()).ceil() as uint;
    if x % 2 == 0 { return 2; };
    for n in iter::range_step(3, m + 1, 2) {
        if x % n == 0 { return n; };
    }
    return x;
}

/// Find all prime factors of a number
pub fn factors(x: uint) -> Vec<uint> {
    let mut lst: Vec<uint> = Vec::new();
    let mut curn = x;
    loop  {
        let m = firstfac(curn);
        lst.push(m);
        if m == curn { break  } else { curn /= m };
    }
    return lst
}

/// Find all unique prime factors of a number
pub fn factors_uniq(x: uint) -> Vec<uint> {
    let mut lst: Vec<uint> = Vec::new();
    let mut curn = x;
    loop  {
        let m = firstfac(curn);
        lst.push(m);
        if curn == m { break ; }
        while curn % m == 0 { curn /= m; }
        if curn == 1 { break ; }
    }
    return lst
}

/// Test whether a number is prime. Checks every odd number up to sqrt(n).
pub fn is_prime(n : uint) -> bool {
    if n <= 1 {return false;}
    firstfac(n) == n
}

#[test]
fn test_iter(){
    let mut pset = PrimeSet::new();
    let first_few = [2u,3,5,7,11,13,17,19,23];
    for (m, &n) in pset.iter().zip(first_few.iter()) {
        assert_eq!(m, n);
        break;
    }
}


#[test]
fn test_primes(){
    let mut pset = PrimeSet::new();
    
    // note: some are repeated, because the pset list grows as it goes
    
    assert!(!pset.is_prime(1));
    assert!(!is_prime(1));
    assert!(pset.is_prime(2));
    assert!(is_prime(2));
    assert!(pset.is_prime(13));
    assert!(is_prime(13));
    assert!(!pset.is_prime(45));
    assert!(!is_prime(45));
    assert!(!pset.is_prime(13*13));
    assert!(!is_prime(13*13));
    assert!(pset.is_prime(13));
    assert!(pset.is_prime(7));
    assert!(is_prime(7));
    assert!(!pset.is_prime(9));
    assert!(!is_prime(9));
    assert!(pset.is_prime(5));
    assert!(is_prime(5));
}
