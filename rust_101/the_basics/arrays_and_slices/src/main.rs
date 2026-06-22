fn main() {
    let mut x: [i32; 500] = [0; 500];

    x[0] = 1;

    println!("x[0]: {}", x[0]);

    let f: Option<&i32> = x.get(501);

    println!("f: {:?}", f);
}
