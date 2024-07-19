#![no_std]
use sails::prelude::*;
mod service;
use service::ExtendedService;

#[derive(Default)]
pub struct MyProgram;

#[gprogram]
impl MyProgram {
    pub fn new(name: String, symbol: String, decimals: u8) -> Self {
        ExtendedService::init(name, symbol, decimals);
        Self
    }

    pub fn vft(&self) -> ExtendedService {
        ExtendedService::new()
    }
}
