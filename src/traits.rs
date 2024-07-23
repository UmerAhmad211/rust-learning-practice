pub trait summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from .....{}", self.summarize_author())
    }
}
struct news_article {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl summary for news_article {
    fn summarize_author(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

//if in mod in another file i.e lib.rs then struct vars should be pub ?? and struct should be pub ??

impl summary for tweet {
    // fn summarize(&self) -> String {
    //   format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let t1 = tweet {
        username: String::from("Neil Halstead"),
        content: String::from("allison im lost"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", t1.summarize());
}
