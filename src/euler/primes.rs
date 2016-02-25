extern crate rand;
use super::{ int_sqrt, modmul, modpow, modpow_s };

/// Deterministic primality test.
pub fn is_prime(n : u64) -> bool {
	if n <= 1 {
		return false;
	}
	if n == 2 {
		return true;
	}
	if n & 1 == 0 {
		return false;
	}
	let mut x : u64 = 3;
	while x * x <= n {
		if n % x == 0 {
			return false;
		}
		x += 2;
	}
	return true;
}

/// Miller-Rabin.
/// Can potentially overflow on 'n' greater than 2^32.
/// Use is_prob_prime_s for a safe version.
pub fn is_prob_prime(n : u64, trials : u32) -> bool {

  use self::rand::distributions::{Range, IndependentSample};

  if n == 2 || n == 3 {
  	return true;
  }
  if n <= 1  || n & 1 == 0 { 
  	return false;
  }

  let dist = Range::new(2, n - 1);
  let mut rng = rand::thread_rng();

  let mut d : u64 = (n - 1) / 2;
  let mut s : u32 = 1;

  while d & 1 == 0 {
    d >>= 1;
    s += 1; 
  }

  'trial_loop: for _ in 0 .. trials {
    let a : u64 = dist.ind_sample(&mut rng);
    let mut x : u64 = modpow(a, d, n);

    if x == 1 || x == n - 1 {
      continue;
    }

    for _ in 0 .. s - 1 {
      x = (x * x) % n;
      if x == 1 {
      	return false;
      }
      if x == n - 1 {
      	continue 'trial_loop;
      }
    }
    return false;
  }
  return true;
}

/// Safe Miller-Rabin.
/// Does not overflow.
pub fn is_prob_prime_s(n : u64, trials : u32) -> bool {

  use self::rand::distributions::{Range, IndependentSample};

  if n == 2 || n == 3 {
    return true;
  }
  if n <= 1  || n & 1 == 0 { 
    return false;
  }

  let dist = Range::new(2, n - 1);
  let mut rng = rand::thread_rng();

  let mut d : u64 = (n - 1) / 2;
  let mut s : u32 = 1;

  while d & 1 == 0 {
    d >>= 1;
    s += 1; 
  }

  'trial_loop: for _ in 0 .. trials {
    let a : u64 = dist.ind_sample(&mut rng);
    let mut x : u64 = modpow_s(a, d, n);

    if x == 1 || x == n - 1 {
      continue;
    }

    for _ in 0 .. s - 1 {
      x = modmul(x, x, n);
      if x == 1 {
        return false;
      }
      if x == n - 1 {
        continue 'trial_loop;
      }
    }
    return false;
  }
  return true;
}

/// Sieve of Eratosthenes.
pub fn prime_sieve(max : usize) -> Vec<bool> {
  let sqrt_max = int_sqrt(max as u64) as usize;
  let mut is_prime = vec![false; max + 1];

  is_prime[2] = true;

  let mut n = 3;
  while n <= max {
    is_prime[n] = true;
    n += 2;
  }
  n = 3;
  while n <= sqrt_max {
    if is_prime[n] {
      let mut k = 2 * n;
      while k <= max {
        is_prime[k] = false;
        k += n;
      }
    }
    n += 2;
  }

  return is_prime;
}