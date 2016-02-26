/// Integer square root.
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

/// Triange number
pub fn tri(n : u64) -> u64{
	return ((n + 1) * n) / 2;
}