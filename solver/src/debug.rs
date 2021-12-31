pub fn print_mask(mask: u64) {
	let mut str = String::new();
	for i in 0..7 {
		for j in 0..7 {
			str.push(
				if  0 != mask & 1 << j << 7 * i { 'X' } else { '.' }
			)
		}
		str.push_str("\n");
	}
	println!("{}", str);
}
