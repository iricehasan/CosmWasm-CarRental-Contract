use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient Balance(balance {balance:?}")]
    InsufficientBalance { balance: u64 },

    #[error("User Already Exists(address {address:?}")]
    UserAlreadyExists { address: Addr },

    #[error("User With This ID Does Not Exist(address {address:?}")]
    UserDoesNotExist { address: Addr },

    #[error("Car With This ID Does Not Exist(car_id {car_id:?}")]
    CarDoesNotExist { car_id: u64 },

    #[error("Car Already Exists(car_id {car_id:?}")]
    CarAlreadyExists { car_id: u64 },

    #[error("Car Is Not Available For Rent(car_id {car_id:?}")]
    CarIsNotAvailable{ car_id: u64 },

    #[error("Car Is Not Rented Yet(car_id {car_id:?}")]
    CarIsNotRentedYet { car_id: u64},

    #[error("Invalid rent dates")]
    InvalidRentDates {},

    #[error("Rent does not exist(rent_id {rent_id:?}")]
    RentDoesNotExist { rent_id: u64 },
}




