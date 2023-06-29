use std::fmt::Display;

// trait `Summary` can be implemented by any type that has the `summarize`
// method matching the signature defined in the trait definition. It is also
// public, so that any crate depending upon this crate can use it.
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format! ("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub content:  String,
    pub author:   String,
}


// `NewsArticle` uses the default definition of method `summarize`, defined
// in the definition for trait `Summary`
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content:  String,
    pub retweet:    bool,
    pub reply:      bool,
}

// method definition for `summarize_author` matches that which is defined in
// the `Summary` trait definition, so the Summary trait can be implemented on
// the type `Tweet`
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub struct Pair<T> {
    x: T,
    y: T,
}

// `Pair<T>` always implements the method `new` to return a new instance of
// `Pair<T>`
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// `Pair<T>` only implements the `cmp_display` mehtod if inner typer `T`
// implements the `PartialOrd` trait that enables comparison, and the
// `Display` trait that enables printing
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content:  String::from("of course, as you probably already know, people"),
        retweet:  false,
        reply:    false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Champlionship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author:   String::from("Iceburgh"),
        content:  String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available {}", article.summarize());

    notify(&article);
    notify_bound(&article);
}

// Instead of a concrete data type, `item` is defined as implementing
// the trait `Summary`. This is required to call the `summarize`
// method on `item`
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// This is functionally equivalent to `notify`, the syntax is just more verbose
pub fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// `impl_trait_ex` allows for `item1` and `item2` to have different types
// as long as they both implement the trait `Summary`
pub fn impl_trait_ex(item1: &impl Summary, item2: &impl Summary) {}

// `bound_trait_ex` requires that `item1` and `item2` are of the same concrete
// type, and that type must implement the `Summary` trait
pub fn bound_trait_ex<T: Summary>(item1: &T, item2: &T) {}

// The `+` operator can be used to specify that a parameter must implement
// multiple traits, in this case `item` can be any concrete type that
// implements both `Summary` and `Display` traits
pub fn impl_multi_trait_ex(item: &(impl Summary + Display)) {}

/// `bound_multi_trait_ex` is functionally equivalent to
/// `impl_multi_trait_ex`, just using bound syntax
pub fn bound_multi_trait_ex<T: Summary + Display>(item: &T) {}

// Another way to define multiple trait bounds is to use the where syntax:
pub fn where_multi_trait_ex<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Summary,
{0}

// `returns_summarizable` requires the a concrete type which implements
// the trait `Summary` is returned. In this case, that is a `Tweet`.
// Note that `impl ${trait}` only works for returns if the function is
// only able to return one concrete type. For a function that could return
// one type implementing `Summary` or another, this would not work.
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content:  String::from(
            "of course, as you probably already know, people"   
        ),
        retweet: false,
        reply:   false,
    }
}
