// use std::fs::File;
// use std::io::ErrorKind;
// fn main() {
//     let f = File::open("hello.txt");
//     match f{
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => panic!("Problem opening the file: {:?}", other_error),
//         }
//     };
// }
//高级写法
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }


use std::error::Error;
use std::fs::File;

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}