pub fn run_all() {
    println!("(1) Copy type (i32)");
    copy_type();

    println!("(2) Move with String");
    move_string();

    println!("(3) Clone with String");
    clone_string();

    println!("(4) Borrow immutably");
    borrow_immut();

    println!("(5) Borrow mutably");
    borrow_mut();
}

fn copy_type() {
    let a = 10;
    let b = a;
    println!("a={a}, b={b}");
}

fn move_string() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2={s2}");
    // println!("s1={}", s1); // uncomment to see move error
}

fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1={s1}, s2={s2}");
}

fn borrow_immut() {
    let s = String::from("hello");
    print_str(&s);
    println!("still have s={s}");
}

fn borrow_mut() {
    let mut s = String::from("hello");
    push_world(&mut s);
    println!("after mutate: {s}");
}

fn print_str(s: &str) {
    println!("borrowed: {s}");
}

fn push_world(s: &mut String) {
    s.push_str(" world");
}