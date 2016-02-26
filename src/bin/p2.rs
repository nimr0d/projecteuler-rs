
extern crate euler;

fn main() {
	let N = 4000000;
	let mut f = (0, 1);
	let mut sum = 0;
	while f.0 <= N {
		sum += f.0;
		f = (f.0 + 2 * f.1, 2 * f.0 + 3 * f.1);
	}
	println!("{}", sum);
}
