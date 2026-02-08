fn main() {
	let mut msg = String::from("Hello");
	msg.push_str(", world!");

	println!("{msg}");

	// let s1 = String::from("Hello");
	// let s2 = s1;

	// println!("{s1}"); // This will not execute as s1 gets moved to s2 and is invalid after being moved to s2
	// We can use the s1 by calling the copy method s2 = s1.copy() this will create a copy in the heap memory
	let s = String::from("Hello"); // Here s comes into scope.

	takes_ownership(s);	 // s is moved to the function
				// s is no longer valid here

	let x = 5;	// x comes into scope
	
	makes_copy(x); 	// As x is an integer it implements the copy trait. Its value doesn't move 
			// x can be used here
	let s1 = gives_ownership(); // this gives_ownership() function return value will be moved to s1.
	println!("{s1}");
	let s2 = String::from("Hello");
	let s3 = takes_and_gives_back(s2); // Here s2 is moved into takes_and_gives_back() and this function will return some value which will be moved to s3.
	println!("{s3}");

	let s4 = String::from("Blaze");
	let (s5, len) = calculate_length(s4);
	println!("The length of the string {s5} is {len}");
} // Here s3 is dropped, s2 is already moved so nothing happens, s1 is also dropped.

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len(); // len() returns the length of a string.
	
	(s, length)
}

fn gives_ownership() -> String {
	let some_string = String::from("yours");

	some_string	// the value of some_string is returned to s1.
}

fn takes_and_gives_back(a_string: String) -> String {
	// a_string comes into scope 
	a_string // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
	println!("{some_string}");
}

fn makes_copy(some_int: i32) {
	println!("{some_int}");
}

