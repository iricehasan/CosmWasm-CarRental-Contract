use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Insufficient Balance(balance {balance:?}")]
    InsufficientBalance { balance: u64 },

    #[error("User Already Exists(id {id:?}")]
    UserAlreadyExists { id: u64 },

    #[error("Car With This ID Does Not Exist(car_id {car_id:?}")]
    CarDoesNotExist { car_id: u64 },

    #[error("Car Already Exists(car_id {car_id:?}")]
    CarAlreadyExists { car_id: u64 },

    #[error("Car Is Not Available For Rent(car_id {car_id:?}")]
    CarIsNotAvailable{ car_id: u64 },

    #[error("Car Is Not Rented Yet(car_id {car_id:?}")]
    CarIsNotRentedYet { car_id: u64},
}




