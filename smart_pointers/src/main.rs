enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use crate::{
    List::{Cons, Nil},
    RcList::{RcCons, RcNil},
};

use std::rc::Rc;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));

    let b = RcCons(3, Rc::clone(&a));
    
    let c = RcCons(4, Rc::clone(&a));
    
    let d = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let f = CustomSmartPointer {
        data: String::from("early drop"),
    };

    println!("CustomSmartPointers created.");

    drop(f);
    
    hello("brown");
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
