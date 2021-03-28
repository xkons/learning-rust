#![allow(dead_code)]

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// any methods of the Trait that are not defined within the curly braces will use the default implementation
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}, (Read more from {}...)", self.content, self.summarize_author())
    }
}

// impl traitname is syntax sugar for the trait bound below
fn notify(item: &impl Summary) {
    println!("Breaking news with trait as parameter! {}", item.summarize());
}

// trait bound style
fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news with trait as parameter using the bound syntax! {}", item.summarize());
}

fn notify_bound2<T: Summary>(item1: &T, item2: &T) {
    println!("Two things want to be summarized!");
    println!("1: {}", item1.summarize());
    println!("2: {}", item2.summarize());
}

fn notify_bound3<T: Summary + Display>(item: &T) {
    println!("Display: {}, Summary: {}", item.display(), item.summarize())
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());

    notify(&article);
}
