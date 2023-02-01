#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name:&str)-> String{
    format!("Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greeting() {
        let myname = String::from("Wukai");
        let result = greeting(&myname);
        assert!(result.contains("Wukai"),"Greeting didn`t contain name,value was ");
    }

    // #[test]
    // fn test_can_hold() {
    //     let larger = Rectangle{
    //         length: 8,
    //         width: 7,
    //     };
    //     let smaller = Rectangle{
    //         length: 5,
    //         width: 1,
    //     };
    //     assert_eq!(larger.can_hold(&smaller),true);
    //     assert_eq!(true,larger.can_hold(&smaller));
    //     assert_ne!(larger.can_hold(&smaller),false);
    //     assert_eq!(2,1);
    //     assert_ne!(5,3);
    // }
}