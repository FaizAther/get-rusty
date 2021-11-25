
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub auth:       String,
    pub headline:   String,
    pub content:    String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        return format!("Article:\n\t{}\n\t{}\n\t{}\n", self.auth, self.headline, self.content);
    }
}

pub struct Tweet {
    pub username:   String,
    pub content:    String,
    pub reply:      bool,
    pub retweet:    bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        return format!("Tweet:\n\t{}\n\t{}\n\t{}\n\t{}\n", self.username, self.content, self.reply, self.retweet);
    }
}

fn main() {
    let article = Article {
        auth: String::from("John"),
        headline: String::from("Book 1"),
        content: String::from("The word, ..."),
    };

    let tweet = Tweet {
        username: String::from("Peter"),
        content: String::from("Book 2"),
        reply: false,
        retweet: true,
    };

    println!("{}\n{}", article.summarize(), tweet.summarize());
}
