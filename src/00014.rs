use std::collections::HashMap;

fn main() {
	const MAX_NUMBER : u64 = 999999;

	// the sequence's length for all numbers
	// it is a cache
	let mut lengths = HashMap::new();
	lengths.insert(1u64, 1u64);

	// the sequence for a number
	let mut seq: [u64; 1000] = [0; 1000];

	let mut n = MAX_NUMBER;
	while n > 1 {
		// println!("n={:?}", n);
		let mut i = 0;
		seq[i] = n;
		while seq[i] != 1 && !lengths.contains_key(&seq[i]) {
			// skips if number is in the cache
			// println!("seq[{}]={}", i, seq[i]);
			i += 1;
			seq[i] = f(seq[i-1]);
		}
		// println!("seq[{}]={}", i, seq[i]);
		let r = *lengths.get(&seq[i]).unwrap();
		for k in 0..i {
			lengths.insert(seq[k], (i as u64) - (k as u64) + r);
			// println!("lengths[{}]={:?}", seq[k], lengths.get(&seq[k]).unwrap());
		}

		// println!("lengths[{}]={:?}\n", n, lengths.get(&n).unwrap());
		n -= 1;
	}

	let mut max = 0;
	let mut number = 0;
	for (nn, nlengths) in lengths {
		if nn <= MAX_NUMBER && nlengths > max { max = nlengths; number = nn} 
	}
	println!("{} has the the max length {:?}", number, max);
}

fn f(n: u64) -> u64 {
	if n == 1 { return 1; }
	match n % 2 {
		0 => n / 2,
		1 => 3 * n + 1,
		_ => 0
	}
}