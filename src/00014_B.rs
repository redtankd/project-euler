use std::collections::HashMap;

const MAX_NUMBER: u64 = 1000000;

fn main() {
	
	// the sequence's length for all numbers
	// it is a cache
	let mut lengths: HashMap<u64, u64> = HashMap::new();
	lengths.insert(1, 1);

	for i in (0..MAX_NUMBER).rev() {
		ff(i, &mut lengths);
	}

	let mut max = 0;
	let mut number = 0;
	for (nn, nlengths) in lengths {
		if nn <= MAX_NUMBER && nlengths > max { max = nlengths; number = nn} 
	}
	println!("{} has the the max length {:?}", number, max);
}

fn ff(n: u64, lengths: &mut HashMap<u64, u64>) -> (u64, &mut HashMap<u64, u64>) {
	if n == 1 { 
		return (1, lengths); 
	}
	else if lengths.contains_key(&n) { 
		return (*lengths.get(&n).unwrap(), lengths);	
	}
	else {
		let (next_length, new_lengths) = ff(f(n), lengths);
		let length = next_length + 1;
		new_lengths.insert(n, length);
		return (length, new_lengths);
	}

}

fn f(n: u64) -> u64 {
	match n % 2 {
		0 => n / 2,
		1 => 3 * n + 1,
		_ => 0
	}
}