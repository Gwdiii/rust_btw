pub trait Draw { fn draw(&self); }

pub struct Screen { pub components: Vec<Box<dyn Draw>> }
// pub struct Screen<T: Draw> { pub components: Vec<T> }

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() { component.draw(); }
    }
}

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() { component.draw(); }
//     }
// }

pub struct Button {
    pub height: u32,
    pub label: String,
    pub width: u32,
}

impl Draw for Button {
    fn draw(&self) {
    // TODO
    }
}
