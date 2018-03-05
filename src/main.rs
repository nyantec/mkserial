extern crate rand;
extern crate num_bigint as bigint;

use rand::OsRng;
use bigint::RandBigInt;

fn main() {
	let mut rng = OsRng::new().expect("Failed to create RNG");

	println!("{}", rng.gen_biguint(159));
}
