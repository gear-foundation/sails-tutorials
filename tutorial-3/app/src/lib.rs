#![no_std]
use sails_rs::prelude::*;
use vft::{Service as BaseVftService, Storage};

#[derive(Encode, Decode, TypeInfo)]
pub enum Events {
    Minted { to: ActorId, value: U256 },
    Burned { from: ActorId, value: U256 },
}

#[derive(Clone)]
pub struct VftService {
    vft: BaseVftService,
}

static mut ADMIN: Option<ActorId> = None;

fn set_admin(admin: ActorId) {
    unsafe {
        ADMIN = Some(admin);
    }
}

fn get_admin() -> &'static ActorId {
    unsafe {
       ADMIN.as_ref().expect("Contract is not initialized")
    }
}

impl VftService {
    pub fn init(name: String, symbol: String, decimals: u8, admin: ActorId) -> Self {
        set_admin(admin);
        VftService {
            vft: <BaseVftService>::seed(name, symbol, decimals),
        }
    }
}
#[service(extends = BaseVftService, events = Events)]
impl VftService {
    pub fn new() -> Self {
        Self {
            vft: BaseVftService::new(),
        }
    }
    pub fn mint(&mut self, to: ActorId, value: U256) {
        self.only_admin();
        let balances = Storage::balances();
        let total_supply = Storage::total_supply();

        let new_total_supply = total_supply
            .checked_add(value)
            .unwrap_or_else(|| panic!("Numeric overflow occurred"));

        balances
            .entry(to)
            .and_modify(|a| *a = *a + value)
            .or_insert(value);

        *total_supply = new_total_supply;
        let _ = self.notify_on(Events::Minted { to, value });
    }

    pub fn burn(&mut self, from: ActorId, value: U256) {
        self.only_admin();
        let balances = Storage::balances();
        let total_supply = Storage::total_supply();

        let new_total_supply = total_supply
            .checked_sub(value)
            .unwrap_or_else(|| panic!("Numeric unferflow occurred"));

        let balance = balances.get(&from).expect("Account has no balance");

        let new_balance = balance
            .checked_sub(value)
            .unwrap_or_else(|| panic!("Insufficient balance"));

        if !new_balance.is_zero() {
            balances.insert(from, new_balance);
        } else {
            balances.remove(&from);
        }

        *total_supply = new_total_supply;

        let _ = self.notify_on(Events::Burned { from, value });
    }

    pub fn change_admin(&mut self, new_admin: ActorId) {
        self.only_admin();
        set_admin(new_admin);
    }

    pub fn admin_address(&self) -> ActorId {
        get_admin().clone()
    }

    fn only_admin(&self) {
        let admin = get_admin();
        assert_eq!(*admin, gstd::msg::source(), "Unauthorized access");
    }

}

impl AsRef<BaseVftService> for VftService {
    fn as_ref(&self) -> &BaseVftService {
        &self.vft
    }
}

pub struct MyProgram;

#[program]
impl MyProgram {
    pub fn new(name: String, symbol: String, decimals: u8, admin: ActorId) -> Self {
        VftService::init(name, symbol, decimals, admin);
        Self
    }

    pub fn vft(&self) -> VftService {
        VftService::new()
    }
}
