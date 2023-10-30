use oo::Draft___;

fn main() {
    let mut post = Draft___::new_();

    post.add_text("I ate a salad for lunch today");

    let post = post.r_review();
    let post = post.approve_();
    
    assert_eq!("I ate salad for lunch today", post.content_());
}
