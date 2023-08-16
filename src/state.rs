use cosmwasm_std::Addr;
use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct User {
    pub address: Addr,
    pub id: u64,
    pub name: String,
    pub lastname: String,
    pub balance: u64,
}

#[cw_serde]
pub struct Car {
    pub id: u64,
    pub name: String,
    pub model: String,
    pub status: Status, // make this status an enum
    pub rentfee: u64,
    pub depositfee: u64,
}

#[cw_serde]
pub struct Rent{
    pub id: u64,
    pub user: User,
    pub car_id: u64,
    pub car_status: Status,
    pub start_date: u64,
    pub end_date: u64,
    pub rent_cost: u128,
}

#[cw_serde]
pub enum Status {
    InUse {}, 
    Available {},
}

pub const USER: Map<&[u8], User> = Map::new("user");
pub const CAR: Map<&[u8], Car> = Map::new("car");
pub const RENT_SEQ: Item<u64> = Item::new("rent_seq");
pub const RENTS: Map<u64, Rent> = Map::new("rent");

