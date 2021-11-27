use rand_core::OsRng;
use num_bigint::RandBigInt;

fn main() {
	println!("{}", OsRng.gen_biguint(159));
}
