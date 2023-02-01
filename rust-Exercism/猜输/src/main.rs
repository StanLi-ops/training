use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜输！");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("secret number is : {}",secret_number);
    loop {
        let mut guess = String::new();
        println!("输入你猜测的数！");
        io::stdin().read_line(&mut guess).expect("读取失败!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
