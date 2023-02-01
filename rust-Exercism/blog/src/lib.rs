//博文"状态"的特征定义
pub trait State {
    fn requset_rewiew(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&'a self, post: &'a Post) -> &'a str;
}

//博文“草稿”状态特征方法实现
pub struct Draft {}
impl State for Draft {
    fn requset_rewiew(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}
//博文“待审核”状态特征方法实现
pub struct PendingReview {}
impl State for PendingReview {
    fn requset_rewiew(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}
//博文“已发表”状态特征方法实现
pub struct Published {}
impl State for Published {
    fn requset_rewiew(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}
//博文“拒绝”状态特征方法实现
pub struct Reject {}
impl State for Reject {
    fn requset_rewiew(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}

//博文结构体定义
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        
        self.content.push_str(text)
    }
    pub fn requset_rewiew(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.requset_rewiew())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}
