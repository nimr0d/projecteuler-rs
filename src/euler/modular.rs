
/// Modular multiplication.
/// Used for safe multiplication without overflow.
/// Warning: assumes a and b are in range.
pub fn modmul(mut a : u64, mut b : u64, modulus : u64) -> u64 {
	let mut r : u64 = 0;
	while b > 0 {
		if b & 1 != 0 {
			r += a;
			if r >= modulus {
				r -= modulus;
			}
		}
		b >>= 1;
		if b != 0 {
			a += a;
			if a >= modulus {
				a -= modulus;
			}
		}
	}
	return r;
}

/// Modular exponentiation.
/// Can potentially overflow on 'modulus' larger than 2^32.
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

/// Safe Modular exponentiation.
/// Does not overflow.
pub fn modpow_s(mut base : u64, mut exp : u64, modulus : u64) -> u64 {
	base %= modulus;
	let mut result : u64 = 1;
	while exp != 0 {
		if exp & 1 != 0 {
			result = modmul(result, base, modulus);
		}
		base = modmul(base, base, modulus);
		exp >>= 1;
	}
	return result;
}

/// Modular inverse.
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

/// Legendre symbol.
pub fn legendre(n : u64, p : u64) -> i32 {
  let m = modpow(n, (p - 1) / 2, p);
  if m != p - 1 {
  	return m as i32;
  }
  return -1;
}

/// Safe Legendre symbol.
pub fn legendre_s(n : u64, p : u64) -> i32 {
  let m = modpow_s(n, (p - 1) / 2, p);
  if m != p - 1 {
  	return m as i32;
  }
  return -1;
}