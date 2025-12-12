fn main() {
	let transposition_table_size: usize = 90_557; // A prime number close to 128MB
	// let transposition_table_size: usize = 8_388_593; // A prime number close to 64mb
	// let transposition_table_size: usize = 8_388_593; // A prime number close to 64mb
	let stack_size: usize = 1024 * 1024 * 128;
	assert!(transposition_table_size * 8 + 2 * 1024 * 1024 < stack_size);
	println!("cargo::rustc-env=RUST_MIN_STACK={}", stack_size);
	println!(
		"cargo::rustc-env=TRANSPOSITION_TABLE_SIZE={}",
		transposition_table_size
	);
}
