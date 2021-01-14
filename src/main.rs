extern crate rand;
extern crate num_bigint as bigint;

use rand::rngs::OsRng;
use bigint::RandBigInt;

fn main() {
	println!("{}", OsRng.gen_biguint(159));
}
