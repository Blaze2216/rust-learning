fn main() {
	// let s1 = String::from("Hello World!");
	// let len = calculate_length(&s1);

	// println!("The length of {s1} is {len}");

	let mut s1 = String::from("hello");

	// Error because mutable references can be assigned to only one
	// let r1 = &mut s1; 
	// let r2 = &mut s1; 

       // {
       //     let _r1 = &mut s1;
       // } // r1 goes out of the scope so we can make new references with no problem
        
       // let _r2 = &mut s1;
        

	change(&mut s1);

	// can't use at the same time both unmutable and mutable
	//let r0 = &s1;
    //let r1 = &s1;
    //let r2 = &mut s1;

    let r0 = &s1;
    let r1 = &s1;
    println!("the value of {r0}, {r1}"); // variables r0 and r1 will not be used after this

    let r2 = &mut s1;
    println!("the value of {r2}");

    let reference_to_something = no_dangle();
    println!("{reference_to_something}");
        
}

fn no_dangle() -> String {
	let s = String::from("Hello, Blaze");
	s
}

//fn calculate_length(str: &String) -> usize { // str is a reference to a string
	//str.len()
//} // Here str goes out of scope. But because str does not have ownership of what it refers to, the string is not dropped.

fn change(s2: &mut String) {
	s2.push_str(", blaze");
}
