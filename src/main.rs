mod ownership;
mod borrow;
mod move_examples;
mod vec_iter;

fn main() {
	println!("--- Ownership Example ---");
	ownership::run();

	println!("--- Borrow Example ---");
	borrow::run();

	println!("--- Mutable Borrow Example ---");
	borrow::run_mutable();

	println!("--- Take and Return Example ---");
	ownership::take_and_return();

	println!("--- Borrow Function Example ---");
	ownership::borrow_example();

	println!("--- &str Slice Example ---");
	ownership::str_slice_example();

	println!("--- Move/Copy/Clone quick reboot ---");
	move_examples::run_all();

	println!("--- Day 5: Vec + Iterator + Slice ---");
	vec_iter::run_all();
}
