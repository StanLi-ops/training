#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
use std::mem::drop;
fn main() {
    let a = CustomSmartPointer {
        data: String::from("a"),
    };
    {
        let b = CustomSmartPointer {
            data: String::from("b"),
        };
        drop(b);
        drop(a);
    }
    let c = CustomSmartPointer {
        data: String::from("c"),
    };
}
