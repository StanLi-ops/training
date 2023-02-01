pub fn run() {
    let mut hello = String::from("Hello ");
    // println!("{}",hello)

    //Get length
    println!("Length: {}", hello.len());
    //push char
    hello.push('W');
    //push string
    hello.push_str("orld!");
    //capacity inbytes
    println!("Capacity: {}", hello.capacity());
    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World' {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    assert_eq!(2, s.len());
    assert_eq!(11, s.capacity());
    println!("{}", s);
}
