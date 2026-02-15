fn main() {
	// let s1 = String::from("Hello World!");
	// let len = calculate_length(&s1);

	// println!("The length of {s1} is {len}");

	let mut s1 = String::from("hello");

	change(&mut s1);
}

//fn calculate_length(str: &String) -> usize { // str is a reference to a string
	//str.len()
//} // Here str goes out of scope. But because str does not have ownership of what it refers to, the string is not dropped.

fn change(s2: &mut String) {
	s2.push_str(", blaze");
}