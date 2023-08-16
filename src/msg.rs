use crate::state::{User, Rent, Status};
use cosmwasm_std::Addr;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
}

#[cw_serde]
pub enum ExecuteMsg {
    AddUser { name: String, lastname: String },
    AddCar { id: u64, name: String, model: String, rentfee: u64, depositfee: u64 },
    Deposit { amount: u64 },
    Witdhraw { amount: u64 },
    Rent { car_id: u64,  start_date: u64, end_date: u64 },
    EndRent { rent_id: u64 }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(UserBalanceResponse)]
    UserBalance { user_address: Addr},
    #[returns(RentResponse)]
    RentById {},
}

#[cw_serde]
pub struct UserBalanceResponse {
    pub balance: u64,
}

#[cw_serde]
pub struct RentResponse {
    pub id: u64,
    pub user: User,
    pub car_id: u64,
    pub car_status: Status,
    pub start_date: u64,
    pub end_date: u64,
    pub rent_cost: u128,
}

impl From<Rent> for RentResponse {
    fn from(rent: Rent) -> RentResponse {
        RentResponse {
            id: rent.id,
            user: rent.user,
            car_id: rent.car_id,
            car_status: rent.car_status,
            start_date: rent.start_date,
            end_date: rent.end_date,
            rent_cost: rent.rent_cost,
        }
    }
}
