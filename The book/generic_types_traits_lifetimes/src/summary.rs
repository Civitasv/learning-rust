pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Default behaviour")
    }
}
// trait: interface
// crate: lib/executable
// struct: class

pub trait Trait2 {}

pub struct NewArticle<'a> {
    pub headline: &'a str,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl<'a> Summary for NewArticle<'a> {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl<T: Summary> Trait2 for T {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// actually, it is...
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: &(impl Summary + Trait2)) {}

pub fn notify4<T: Summary + Trait2>(item: &T) {}

// syntax where
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Trait2,
    U: Summary + Trait2,
{
    2
}

// return impl
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
