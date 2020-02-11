fn main() {
    largest_example();
    news_and_tweet();
}

fn largest_example() {
    let mut number_list = vec![34, 50, 25, 100, 65];
    println!("The largest is: {}", largest(&number_list));

    number_list.push(4);
    println!("The largest is: {}", largest(&number_list));

    number_list.push(400);
    println!("The largest is: {}", largest(&number_list));
}

fn largest_for_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// fn largest(list: &[i32]) -> i32 {
//     largest_for_i32(list)
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    username: String,
    content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Summary for Book {

}

fn news_and_tweet() {
    let tweet = Tweet {
        username: String::from("someone"),
        content: String::from("Lorem ipsum content"),
        reply: false,
        retweet: false,
    };
    println!("Tweet: {}", tweet.summarize());
    notify(tweet);

    let book = Book {
        title: String::from("Little book"),
        author: String::from("John Doe"),
    };
    println!("Book: {}", book.summarize());
    notify(book);
    // notify(String::from("Test")); // fails

    println!("Another tweet: {}", new_reading().summarize());
    let another = new_reading();
    println!("Another another tweet: {}", another.summarize());
}

fn notify(item: impl Summary) {
    println!("New! {}", item.summarize());
}

fn new_reading() -> impl Summary {
    Tweet {
        username: String::from("No one"),
        content: String::from("Oh really?"),
        reply: false,
        retweet: false,
    }
}
