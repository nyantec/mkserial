use std::collections::HashSet;
use std::process::Command;
use std::str;

use num_bigint::BigUint;

fn mkserial() -> BigUint {
	let out = Command::new(env!("CARGO_BIN_EXE_mkserial")).output().unwrap();
	assert!(out.status.success());
	assert!(out.stderr.is_empty());

	str::from_utf8(&out.stdout).unwrap().trim_end().parse::<BigUint>().unwrap()
}

fn sample(samples: usize) {
	let mut set = HashSet::with_capacity(samples);

	for _ in 0..samples {
		let serial = mkserial();

		assert!(serial.bits() >= 128);
		assert!(serial.bits() < 160);
		assert!(set.insert(serial));
	}
}

#[test]
fn sample_few() {
	sample(16384);
}

#[test] #[ignore]
fn sample_many() {
	sample(1048576);
}
