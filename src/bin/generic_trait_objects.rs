#![allow(unused_variables)]

trait Item {
    fn add<T>(&self, t: &T)
    where
        Self: Sized;
}

struct A();

impl Item for A {
    fn add<T: Sized>(&self, t: &T) {}
}

struct B();

impl Item for B {
    fn add<T: Sized>(&self, t: &T) {}
}

struct ItemFactory();

impl ItemFactory {
    pub fn create(&self, _type: &str) -> Box<dyn Item> {
        Box::from(A())
    }
}

fn main() {
    let a = ItemFactory().create("as");
    // the `add` method cannot be invoked on a trait object. You need `&dyn Item` instead of `&mut dyn Item`
    // the `add` method cannot be invoked on a trait object. You need `&mut dyn Item` instead of `&dyn Item`
    // the `add` method cannot be invoked on a trait object
    // a.add("asdasd");
}
