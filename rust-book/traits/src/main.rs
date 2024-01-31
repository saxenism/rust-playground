// Traits allow us to define a set of methods that are shared across different types

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
    // This means that, for every type that implements this trait, they should have a summarize method that returns a string.
    // fn summarize(&self) -> String;

    // Now, if we do not expect all the types implementing this trait to implement the body of the function, we can create a default implementation:
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }

    // Default implementations can call other methods inside our trait definition
    fn summarize_author(&self) -> String;
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     return format!("{} by {}", self.headline, self.author);
    // }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        return format!("{}: {}", self.username, self.content);
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@saxenism"),
        content: String::from("Balanced in the macro. Obsessed in the micro. Works for me 10/10."),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling, lol."),
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Article Summary: {}", article.summarize());

}
