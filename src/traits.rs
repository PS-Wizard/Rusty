use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    // Default Thing:
    fn summarize(&self) -> String {
        "Default Implementation".to_owned()
    }
    // fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{} wrote {}, with contents {}",
            self.author, self.headline, self.content
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        if self.retweet {
            format!("{} retweeted {}", self.username, self.content)
        } else if self.reply {
            format!("{} replied {}", self.username, self.content)
        } else if self.reply && self.retweet {
            format!("{} retweeted and replied {}", self.username, self.content)
        } else {
            format!("{} says {}", self.username, self.content)
        }
    }
}

struct Somethingelse {}
impl Summary for Somethingelse {}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking News: {}", item.summarize());
// }
//

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking News: {}", item.summarize());
}

// pub fn notifyonsteroids<T, U>(item: &T, item2: &U) {}
pub fn notifyonsteroids<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Summary,
{
    todo!("The use of where with generics");
}
pub fn traits() {
    let newtweet = Tweet {
        username: "GigaChad".to_owned(),
        content: "Lorem Ipusm Random Thingy".to_owned(),
        reply: true,
        retweet: true,
    };

    let newnews = NewsArticle {
        author: "Some Author".to_owned(),
        headline: "Some Catchy Headline".to_owned(),
        content: "Some Random Content".to_owned(),
    };

    let newsomething = Somethingelse {};
    println!("{}", newnews.summarize());
    println!("{}", newtweet.summarize());
    println!("{}", newsomething.summarize());
    notify(&newsomething);
}
