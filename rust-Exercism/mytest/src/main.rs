// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point{
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// impl<T, U> Point<T, U> {
//     fn chr(self) -> Point<U, T> {
//         Point{
//             x: self.y,
//             y: self.x,
//         }
//     }
// }
// #[derive(Debug)]
// enum Option<T>{
//     Some(T),
//     None,
// }

use mytest::NewsArticle;
use mytest::Summary;
use mytest::Tweet;

fn main() {
    // let p1 = Point { x: 1, y: "hello" };
    // println!("{:?},{:?}", p1.x, p1.y);
    // let p2 = Point { x: "hello", y: 0.567 };
    // println!("{:?},{:?}", p2.x, p2.y);
    // let p3 = p1.mixup(p2);
    // println!("{:?},{:?}", p3.x, p3.y);
    // let p4 = p3.chr();
    // println!("{:?},{:?}", p4.x, p4.y);
    // let a1 = vec![Option::Some(23),Option::None];
    // let a3 = vec![Option::Some(String::from("some")),Option::None];
    // println!("{:#?}",a1);
    // println!("{:#?}",a3);

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
