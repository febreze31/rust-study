pub fn run_all() {
    println!("(1) vec basics");
    vec_basics();

    println!("(2) iter vs iter_mut");
    iter_vs_iter_mut();

    println!("(3) into_iter (moves ownership)");
    into_iter_moves();

    println!("(4) slice parameter");
    slice_param_demo();
}

fn vec_basics() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v = {:?}", v);

    let last = v.pop();
    println!("popped = {:?}, v = {:?}", last, v);
}

fn iter_vs_iter_mut() {
    let mut v = vec![1, 2, 3];

    // 읽기 전용 borrow 반복
    for x in v.iter() {
        print!("{x} ");
    }
    println!();

    // 수정 가능한 borrow 반복
    for x in v.iter_mut() {
        *x *= 10;
    }
    println!("after iter_mut: {:?}", v);
}

fn into_iter_moves() {
    let v = vec![1, 2, 3];

    for x in v.into_iter() {
        print!("{x} ");
    }
    println!();

    // println!("{:?}", v); // 주석 풀면 에러: v는 move됨
}

fn sum_slice(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn slice_param_demo() {
    let v = vec![1, 2, 3, 4];
    let s = sum_slice(&v);        // Vec도 slice로 빌려줄 수 있음
    let s2 = sum_slice(&v[1..3]); // slice도 바로 가능
    println!("sum(v)={s}, sum(v[1..3])={s2}");
}