fn main() {
    let a = do_twice(add_one, 1);
    println!("{}", a);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
    let list_of_numbers2 = vec![1, 2, 3];
    let list_of_strings2: Vec<String> = list_of_numbers2.iter().map(ToString::to_string).collect();
}

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
