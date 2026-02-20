mod ownership;
mod borrow;

fn main() {
	println!("--- Ownership Example ---");
	ownership::run();

	println!("--- Borrow Example ---");
	borrow::run();

	println!("--- Mutable Borrow Example ---");
	borrow::run_mutable();

}
