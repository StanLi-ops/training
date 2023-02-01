pub fn run() {
    //Default is "i32"
    let _x = 1;
    //Default is "f64"
    let _y = 2.5;
    //Add explicit type
    let _z: i64 = 4545454545;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_actice = true;
    //Get boolean from expression
    let is_greater = 10 < 5;
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (_x, _y, _z, is_actice, is_greater, a1, face));
}
