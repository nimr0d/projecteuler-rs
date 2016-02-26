extern crate euler;

fn main() {
	let N = 999;

	let mut res = 0;
	let mut sign = 1;
	for k in [3, 15, 5].iter() {
		res += sign * k * (euler::tri((N / k) as u64) as i32);
		sign *= -1;
	}
	println!("{}", res);
}
