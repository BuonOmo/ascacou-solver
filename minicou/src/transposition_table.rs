pub struct TranspositionTable<const S: usize> {
	table: [Entry; S],
}

// 1 byte!
#[derive(Copy, Clone)]
struct Entry {
	// 56 bits
	key: Key,
	// 8 bits
	value: i8,
}

#[repr(packed)]
#[derive(Copy, Clone, PartialEq)]
struct Key(u32, u16, u8);

impl Default for Key {
	fn default() -> Self {
		Key(0, 0, 0)
	}
}

impl Default for Entry {
	fn default() -> Self {
		Entry {
			key: Key::default(),
			value: 0,
		}
	}
}

impl<const S: usize> Default for TranspositionTable<S> {
	fn default() -> Self {
		TranspositionTable {
			table: [Entry::default(); S],
		}
	}
}

impl<const S: usize> TranspositionTable<S> {
	fn index(&self, key: u64) -> usize {
		(key as usize) % S
	}

	pub fn get(&self, key: u64) -> Option<i8> {
		let index = self.index(key);
		let entry = &self.table[index];
		let entry_key = Self::key_from_u64(key);
		if entry.key == entry_key {
			Some(entry.value)
		} else {
			None
		}
	}

	pub fn insert(&mut self, key: u64, value: i8) {
		let index = self.index(key);
		let entry_key = Self::key_from_u64(key);
		self.table[index] = Entry {
			key: entry_key,
			value,
		};
	}

	fn key_from_u64(key: u64) -> Key {
		Key(
			(key & 0xFFFFFFFF) as u32,
			((key >> 32) & 0xFFFF) as u16,
			((key >> 48) & 0xFF) as u8,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	extern crate test;

	#[bench]
	fn bench_vec_access(b: &mut test::Bencher) {
		let size = 1_000_000;
		let mut vec = vec![0i8; size];
		let key: usize = 123_456;
		vec[key] = 42;
		b.iter(|| {
			let _value = vec[key];
		});
	}

	#[bench]
	fn bench_slice_access(b: &mut test::Bencher) {
		const SLICE_SIZE: usize = 1_000_000;
		let mut slice = [0i8; SLICE_SIZE];
		let key: usize = 123_456;
		slice[key] = 42;
		b.iter(|| {
			let _value = slice[key];
		});
	}

	#[bench]
	fn bench_transposition_table_access(b: &mut test::Bencher) {
		const TABLE_SIZE: usize = 1_000_000;
		let mut table: TranspositionTable<TABLE_SIZE> = TranspositionTable::default();
		let key: u64 = 123_456;
		table.insert(key, 42);
		b.iter(|| {
			let _value = table.get(key);
		});
	}

	#[bench]
	fn bench_vec_insertion(b: &mut test::Bencher) {
		let size = 1_000_000;
		let mut vec = vec![0i8; size];
		let key: usize = 123_456;
		b.iter(|| vec[key] = 42);
	}

	#[bench]
	fn bench_slice_insertion(b: &mut test::Bencher) {
		const SLICE_SIZE: usize = 1_000_000;
		let mut slice = [0i8; SLICE_SIZE];
		let key: usize = 123_456;
		b.iter(|| slice[key] = 42);
	}

	#[bench]
	fn bench_transposition_table_insertion(b: &mut test::Bencher) {
		const TABLE_SIZE: usize = 1_000_000;
		let mut table: TranspositionTable<TABLE_SIZE> = TranspositionTable::default();
		let key: u64 = 123_456;
		b.iter(|| table.insert(key, 42));
	}
}
