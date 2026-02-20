pub fn run(){
	let s1 = String::from("hello");
	print_string(&s1);
	println!("{}", s1);
}

pub fn run_mutable() {
	let mut s = String::from("hello");

	add_world(&mut s);
	println!("after: {}", s);
}

fn add_world(s: &mut String){
	s.push_str(" world");
}

fn print_string(s: &String){
	println!("{}", s);
}

