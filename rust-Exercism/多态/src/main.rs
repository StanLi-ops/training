trait Draw {
    fn draw(&self);
}

struct Screen {
    pub commponents: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for commponent in self.commponents.iter() {
            commponent.draw();
        }
    }
}

struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {}
}

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {}
}

fn main() {
    let screen = Screen {
        commponents: vec![
            Box::new(SelectBox {
                width: 10,
                height: 15,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 10,
                height: 15,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
