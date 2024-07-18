use sails::prelude::*;

#[derive(Default)]
pub struct HelloWorld(());

#[gservice]
impl HelloWorld {
    pub fn hello_world(&mut self) -> &'static str {
        "Hello, world!"
    }
}
