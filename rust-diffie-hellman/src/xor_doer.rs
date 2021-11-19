fn do_xor(g: &[u8], s: &[u8]) -> Vec<u8> {
	let mut vec = Vec::new();
	for i in 0..s.len() {
		vec.push(s[i]^g[i%g.len()]);	
	}
	vec
}

fn main() {
	let g = [10, 20, 30, 40];
	let s = [1, 2, 3, 4, 5, 6, 7, 8, 9];
	println!("{:?}", do_xor(&g, &s));
	println!("11, 22, 31, 44, 15")
}	
