pub fn run(){
	let s1 = String::from("hello");
	let s2 = s1.clone();

	println!("s1: {}, s2: {}", s1, s2);
}

pub fn take_and_return(){
	let s = String::from("hello");
	let s = takes_ownership(s);
	println!("Returned : {}", s);
}

fn takes_ownership(s: String) -> String {
	println!("Inside function: {}", s);
	s
}

pub fn borrow_example(){
	let s = String::from("hello");
	print_ref(&s);
	println!("Still usable: {}", s);
}

fn print_ref(s: &str){
	println!("Borrowed: {}", s);
}

pub fn str_slice_example(){
	let s = String::from("hello world");
	print_ref(&s);
    print_ref(&s[0..5]);
}