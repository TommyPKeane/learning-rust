fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);

    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);

    let mut x: i32 = 10;
    let ref_x: &mut i32 = &mut x;
    *ref_x = 20;
    println!("x: {x}");

    // // Bad Code | (Example) Will Crash during Build
    // let ref_y: &i32;
    // {
    //     let y: i32 = 10;
    //     ref_y = &y;
    // }
    // println!("ref_y: {ref_y}"); // Broken Reference: `rustc --explain E0597`

    let r: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("r: {r:?}");

    let s: &[i32] = &r[2..4];
    println!("s: {s:?}");

    let s1: &str = "Hello";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");
}
