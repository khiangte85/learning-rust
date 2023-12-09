use std::fmt::Display;

pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String;
}

pub struct Article {
    author: String,
    title: String,
    content: String,
}

pub struct Tweet {
    username: String,
    tweet: String,
}

impl Summary for Article {
    fn author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{} - {}", self.title, self.content)
    }
}

impl Summary for Tweet {
    fn author(&self) -> String {
        format!("{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}", self.tweet)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("{} - {}", item.summarize(), item.author());
}

pub fn notify1<T>(item: &T)
where
    T: Summary + Display,
{
    println!("{} - {}", item.summarize(), item.author());
}

fn main() {
    let article = Article {
        author: String::from("Awmtea"),
        title: String::from("Learning Rust"),
        content: String::from("First day of learning Rust"),
    };

    let tweet = Tweet {
        username: String::from("khiangte"),
        tweet: String::from("My first tweet"),
    };

    println!("{}", article.summarize());
    notify(&article);

    println!("{}", tweet.summarize());
    notify(&tweet);
}
