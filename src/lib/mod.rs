extern crate rand;
use self::rand::distributions::{ IndependentSample, Range };

pub fn modpow(mut base : u64, mut exp : u64, modulus : u64) -> u64 {
	base %= modulus;
	let mut result : u64 = 1;
	while exp != 0 {
		if exp & 1 != 0 {
			result = (result * base) % modulus;
		}
		base = (base * base) % modulus;
		exp >>= 1;
	}
	return result;
}

pub fn modinv(n : u64, modulus : u64) -> u64 {
	let mut q;
	let mut x = [n, modulus];
	let mut a = [1u64, 0];
	let mut t : usize = 0;
	while x[t ^ 1] > 0 {
		q = x[t] / x[t ^ 1];
		x[t] -= q * x[t ^ 1];
		a[t] -= q * a[t ^ 1];
		t ^= 1;
	}
	return (t as u64) * modulus + a[t];
}

pub fn legendre(n : u64, p : u64) -> i32 {
  let m = modpow(n, (p - 1) / 2, p);
  if m != p - 1 {
  	return m as i32;
  }
  return -1;
}

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

pub fn is_prob_prime(n : u64, trials : u32) -> bool {

  if n == 2 || n == 3 {
  	return true;
  }
  if n <= 1  || n & 1 == 0 { 
  	return false;
  }

  let dist = rand::distributions::Range::new(2, n - 1);
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

pub fn int_sqrt(mut n : u64) -> u32 {
	let mut res : u64 = 0;
	let mut bit : u64 = 1 << 62;

	while bit > n {
		bit >>= 2;
	}

	while bit != 0 {
		if n >= res + bit {
			n -= res + bit;
			res = (res >> 1) + bit;
		}
		else {
			res >>= 1;
		}
		bit >>= 2;
	}
	return res as u32;
}

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