pub struct Message {
    pub text: String,
    pub date: i64,
}

pub trait Log {
    fn console(&self);
    fn log(a: &str);
}

impl Log for Message {
    fn console(&self) {
        println!("{}", self.text);
    }

    fn log(a: &str) {
        let message= Message {
            text: String::from(a),
            date: 324243,
        };
        message.console();
    }
}