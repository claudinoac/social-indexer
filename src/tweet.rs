pub struct Tweet {
    date: String,
    content: String,
    reactions: i32,
    shares: i32,
    comments: i32,
    user_id: i32,
}

impl Default for Tweet {
    fn default() -> Self {
        Tweet {
            date: String::from("today"),
            content: String::from(""),
            reactions: 0,
            shares: 0,
            comments: 0,
            user_id: 0
        }
    }
}

impl Tweet {
    pub fn new(content: String, user_id: i32) -> Tweet {
        Tweet {
            content: content,
            user_id: user_id,
            ..Tweet::default()
        }
    }
}
