use std::slice;

static mut COUNTER: u32 = 0;

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    raw_pointer();
    arbitrary_address_raw_pointer();
    unsafe {dangerous()}

    let mut v = vec![0, 1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = split_at_mut(r, r.len() >> 1);

    assert_eq!(a, &mut[0, 1, 2]);
    assert_eq!(b, &mut[3, 4, 5]);

    unsafe {println!("Absolute value of -3 according to C: {}", abs(-3))}
    
    static HELLO_WORLD: &str = "Hello, World!";
    println!("name is {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {println!("COUNTER: {}", COUNTER)}
}

fn raw_pointer() {
    let mut num1 = 5;
    let mut num2 = 0;

    let r1 = &num1 as *const i32;
    let mut r2 = &mut num1 as *mut i32;
    println!("const raw-pointer: {:?}", r1);
    println!("original mut raw-pointer: {:?}", r2);

    r2 = &mut num2;
    unsafe {println!("i32 being pointed to: {}", *r2)}
    println!("mutated mut raw-pointer: {:?}", r2);

    unsafe {
        *r2 = 10;
        println!("i32 being pointed to: {}", *r2);
    }

    println!("mutated mut raw-pointer: {:?}", r2);
}

fn arbitrary_address_raw_pointer() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

fn deref_raw_pointer() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust funciton from C!");
}

fn add_to_count(inc: u32) {
    unsafe {COUNTER += inc}
}
