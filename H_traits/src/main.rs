
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        return format!("Read more... @{}", self.summarize_author());
    }
}

pub struct Article {
    pub auth:       String,
    pub headline:   String,
    pub content:    String,
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        return format!("{}", self.auth);
    }

    fn summarize(&self) -> String {
        return format!("Article:\n\t{}\n\t{}\n\t{}\n", self.auth,
                       self.headline, self.content);
    }
}

pub struct Tweet {
    pub username:   String,
    pub content:    String,
    pub reply:      bool,
    pub retweet:    bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("{}", self.username);
    }

    fn summarize(&self) -> String {
        return format!("Tweet:\n\t{}\n\t{}\n\t{}\n\t{}\n", self.username,
                       self.content, self.reply, self.retweet);
    }
}

pub struct Post {
    pub name:   String,
    pub data:   String,
}

impl Summary for Post {
    fn summarize_author(&self) -> String {
        return format!("{}", self.name);
    }
}

// item is a anything that implements Summary
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyy<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn foo(i1: &impl Summary, i2: &impl Summary) {
    println!("foo {}{}", i1.summarize(), i2.summarize());
}

pub fn bar<T: Summary>(i1: &T, i2: &T) {
    println!("bar {}{}", i1.summarize(), i2.summarize());
}

use core::fmt::Display;
use core::fmt::Debug;

pub fn moo<T: Display + Clone, U: Clone + Debug>(i1: &T, i2: &U) {
    println!("{},{:?},", i1, i2);
}

pub fn caz<T, U>(i1: &T, i2: &U)
    where T: Display + Clone,
          U: Clone + Debug
{
    println!("{},{:?},", i1, i2);
}

pub fn gen_summarizable() -> impl Summary {
    return Post {
        name: String::from("name"),
        data: String::from("data"),
    }
}

struct Pair<T> {
    x:  T,
    y:  T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        return Self {
            x:  x,
            y:  y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("large x={}", self.x);
        } else {
            println!("large y={}", self.y);
        }
    }
}

fn main() {
    let article = Article {
        auth:       String::from("John"),
        headline:   String::from("Book >1"),
        content:    String::from("The word, ..."),
    };

    let tweet = Tweet {
        username:   String::from("Peter"),
        content:    String::from("Book 2"),
        reply:      false,
        retweet:    true,
    };

    let post = Post {
        name:       String::from("Book"),
        data:       String::from("content"),
    };

    println!("{}\n{}\n{}", article.summarize(), tweet.summarize(),
        post.summarize());

    notify(&article);
    notifyy(&tweet);

    foo(&post, &article);
    //bar(&post, &article); has to be same type
    bar(&post, &post);

    let summary = gen_summarizable();
    println!("{}", summary.summarize());

    let pair = Pair::new(1,2);
    pair.cmp_display();
}
