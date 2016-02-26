pub fn fib(mut n : u64) -> u64 {
	let mut b = (1u64, 1u64);
	let mut f = (0u64, 1u64);
	while n > 0 {
		if n & 1 != 0 {
			f = (b.1 * f.0 + b.0 * (f.1 - f.0), b.1 * f.1 + b.0 * f.0);
		}
		b = (b.0 * (2 * b.1 - b. 0), b.1 * b.1 + b.0 * b.0);
		n >>= 1;
	}
	return f.0;
}

/*pub fn modfib(mut n : u64, modulus : u64) -> u64 {
	let mut b = (1u64, 1u64);
	let mut f = (0u64, 1u64);
	while n > 0 {
		if n & 1 != 0 {
			f = (b.1 * f.0 + b.0 * (f.1 - f.0), b.1 * f.1 + b.0 * f.0);
		}

		b = (b.0 * (2 * b.1 - b. 0), b.1 * b.1 + b.0 * b.0);
		n >>= 1;
	}
	return f.0;
}*/
