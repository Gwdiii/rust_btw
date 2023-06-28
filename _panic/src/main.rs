// In some cases, such as when the binary needs to be as small as possible,
// the program can abort on panic instead of unwinding, leaving it to the
// OS to clean up the memory. To abort on panic in release mode, put the
// following in the respective `Cargo.toml` file:
// [profile.release]
// panic = 'abort'

// This is supposed to panic
fn main() {
    // panic!("crash and burn")
    
    // To see backtrace, use: `$ RUST_BACKTRACE=1 cargo run` 
    let v = vec![1, 2, 3];
    v[99];
}
