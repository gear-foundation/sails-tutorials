#![no_std]
use sails::prelude::*;
mod service;
use service::Token;

#[derive(Default)]
pub struct MyProgram;

#[gprogram]
impl MyProgram {
    pub fn new(name: String) -> Self {
        Token::init(name);
        Self
    }

    pub fn token(&self) -> Token {
        Token::default()
    }
}
