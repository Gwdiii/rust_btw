// THIS WILL NOT COMPILE DUE TO LIFETIME ELISION EXAMPLES AT THE BOTTOM
// IN ORDER TO COMPILE< COMMENT OUT ALL CODE AFTER LINE 110.

use std::fmt::Display;

// `ImportantExcerpt` has a lifetime requirement that it can only exist as
// long as the value for `part` exists.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// lifetime annotations are required after `impl` and after `ImportantExcerpt`
// because they are part of `ImportantExcerpt`s type. No other lifetime
// annotations are required because the first rule assigns a lifetime to `self`
// and the third rule assigns `self`s lifetime to the return type.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {3}
}

// Like the last example, lifetime annotations are required after `impl` and
// `ImportantExcerpt` because they are part of the struct's type. No other
// annotations are needed because the first and third elision rules are
// satisfied, and the return has the same lifetime as `&self`
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let string1 = String::from("xyz");

    {
        let string2 = String::from("xyz");
        let result  = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // `novel` is created here
    let novel = String::from("Call me Ishmael. Some years ago...");

    // a string slice, which is a reference to novel is created here
    let first_sentence = novel
        .split('.')
        .next()
        .expect("Could not find a '.'");

    // `i` of type `ImportantExcerpt` is created here. `ImportantExcerpt` has
    // a lifetime that must be no longer than the lifetime of `part`. Since
    // `i.part` is assign a string slice, which references `novel`, `i` can only
    // exist as long as `novel` exists. Since `novel` is created before `i`, and
    // `novel` and `i` both exist until the end of the same scope, this works.
    let i = ImportantExcerpt { part: first_sentence };

    // `'static` annotation specifies that the reference should live for the
    // entirety of the program. All string literal assignments have `'static`
    // lifetimes, because their data exists in the binary, and never changes.
    let s: &'static str = "I have a static lifetime";
}

// Lifetime syntax defined here establishes that the returned reference
// is valid as long as both parameters are valid. For some lifetime `'a`,
// the function takes two parameters, both of which are string slices that
// live at least as long as lifetime `'a`. The string slice returned will
// also live as long as lifetime `'a`. The borrwo checker should reject any
// values that don't adhere to these constraints. It doesn't need to know
// exactly how long `x` and `y` will live, only that some scope can be
// substituted for `'a` that will satisfy the signature.
//
// When concrete references are passed to `longest` the concrete lifetime
// substituted for `'a` is the overlapping scope of x and y. This means,
// effectively that the lifetime will end up being equivalent to the shorter
// of the lifetimes of `x` and `y`. Because the return is annotated with
// lifetime `'a`, it's lifetime is also equivalent to the shorter of lifetimes
// of `x` and `y`.
//
// In the case of any function which has a lifetime annotated on the return
// type, that lifetime must be the same as at least one of the parameters.
// Otherwise, the reference would have to be to some value created within
// the function, and that reference would necessarily go out of scope when
// the function exits.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function had three parameters, one of which is a generic. It returns
// a string slice with a lifetime that lasts no longer `x` or `y`, whichever
// ends first. The parameter `ann` can be any type which implements the
// trait `Display`
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
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

// COMMENT OUT ALL CODE AFTER THIS POINT TO COMPILE

// Lifetimes on parameters are referred to as input lifetimes.
// Lifetimes on returns are referred to as output lifetimes.

// Adding lifetimes according to lifetime elision rules
fn first_word(s: &str) -> &str {}
// All paramaters are given a lifetime:
fn first_word1<'a>(s: &'a str) -> &str {}
// If there is only one parameter, that lifetime is assigned to the return
// The third rule, doesn't apply, because their are no input parameters
// that are `self`
fn first_word2<'a>(s: &'a str) -> &'a str {}

// Adding lifetimes according to lifetime elision rules
fn longest1(x: &str, y: &str) -> &str {}
// All parameters are given a lifetime.
// There is more than one parameter, so the second rule doesn't apply.
// None of the parameters are `self`, so the third rule doesn't apply.
// In this case, lifetime elision rules don't provide a lifetime for
// the return, so this would not work without explicitly annotating
// lifetimes for the parameters and the return.
fn longest2<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
