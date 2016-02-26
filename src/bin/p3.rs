
extern crate euler;

fn main() {
	let N : u64 = 600851475143;
	
	let mut n = N;
	let mut k = 3;
	while k * k <= n {
		while n % k == 0 {
			n /= k;
		}
		k += 2;
	}

	println!("{}", n);
}
