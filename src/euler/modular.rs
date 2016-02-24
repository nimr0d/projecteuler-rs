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