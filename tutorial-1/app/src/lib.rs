#![no_std]
use sails::prelude::*;
mod service;
use service::HelloWorld;

#[derive(Default)]
pub struct MyProgram;

#[gprogram]
impl MyProgram {
    pub fn hello_world(&self) -> HelloWorld {
        HelloWorld::default()
    }
}
