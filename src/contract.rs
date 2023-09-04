use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Response, Addr, StdError
};

use crate::error::ContractError;
use crate::state::{User, Rent, Car, Status, USER, CAR, RENTS, RENT_SEQ};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, UserBalanceResponse, RentResponse};

pub const RENT_PERIOD: u64 = 60;

use std::ops::Add;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {

    RENT_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddCar { id, name, model, rentfee, depositfee } => add_car(deps, id, name, model, rentfee, depositfee) ,
        ExecuteMsg::AddUser { name, lastname } => add_user(deps, info, name, lastname),
        ExecuteMsg::Deposit { amount } => deposit(deps, info, amount),
        ExecuteMsg::Witdhraw { amount } => withdraw(deps, info, amount),
        ExecuteMsg::Rent { car_id, start_date, end_date } => rent( deps, info, car_id, start_date, end_date),
        ExecuteMsg::EndRent { rent_id } => end_rent( deps, info, rent_id),
    }
}


#[entry_point]
pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserBalance { user_address } => user_balance(deps, user_address),
        QueryMsg::RentById { rent_id } => rent_by_id(deps, rent_id),
    }
}
// functions for query

fn user_balance(deps:Deps, user_address: Addr) -> StdResult<Binary> {

    // check if user exists
    if !(USER.may_load(deps.storage, user_address.clone())?).is_some() {
        // user does not exist
        return Err(StdError::generic_err("User does not exist"));
    }

    let user = USER.load(deps.storage, user_address.clone())?;
    let resp = UserBalanceResponse {
        balance: user.balance,
    };    

    to_binary(&resp)
}

fn rent_by_id(deps:Deps, rent_id: u64) -> StdResult<Binary> {
    // check if the rent exists

    let rent_key = rent_id;

    let rent = match RENTS.may_load(deps.storage, rent_key)? {
        Some(rent) => Some(rent),
        None => return Err(StdError::generic_err("Rent does not exist"))
    }
    .unwrap(); 

    let resp = RentResponse {
        id: rent_id,
        user: rent.user,
        car_id: rent.car_id,
        car_status: rent.car_status,
        start_date: rent.start_date,
        end_date: rent.end_date,
        rent_cost: rent.rent_cost,
    };

    to_binary(&resp)
}


// Functions for execute entry point

pub fn add_car(deps:DepsMut, id: u64, name:String , model: String, rentfee: u64, depositfee: u64) -> Result<Response, ContractError> { 
    let car = Car {
        id,
        name,
        model,
        rentfee,
        car_status: Status::Available,
        depositfee,
    };

    // check if car exists
    let key = car.id.to_be_bytes();
    if (CAR.may_load(deps.storage, &key)?).is_some() {
        // car already exists
        return Err(ContractError::CarAlreadyExists { car_id: id });
    }

    CAR.save(deps.storage, &key, &car)?;

    Ok(Response::default())
}

pub fn add_user(deps: DepsMut, info: MessageInfo, name: String, lastname: String) -> Result<Response, ContractError>  {

    let user = User {
        address: info.sender.clone(),
        name,
        lastname,
        balance: 0,
    };

    // check if user exists
    if (USER.may_load(deps.storage, info.sender.clone())?).is_some() {
        // user already exists
        return Err(ContractError::UserAlreadyExists { address: info.sender.clone() });
    }

    USER.save(deps.storage, info.sender.clone(), &user)?;

    Ok(Response::default())
}

pub fn deposit(deps: DepsMut, info: MessageInfo, amount: u64) -> Result<Response, ContractError> {
    
    // check if the user exists
    if !(USER.may_load(deps.storage, info.sender.clone())?).is_some() {
        return Err(ContractError::UserDoesNotExist { address: info.sender.clone()});
    }


    let mut user = USER.load(deps.storage, info.sender.clone())?;
    user.balance += amount; // increase the user balance

    USER.save(deps.storage, info.sender.clone(), &user)?;

    Ok(Response::new().add_attribute("action", "deposit"))
}

pub fn withdraw(deps: DepsMut, info: MessageInfo, amount: u64) -> Result<Response, ContractError> {
    
    // check if the user exists
    if !(USER.may_load(deps.storage, info.sender.clone())?).is_some() {
        return Err(ContractError::UserDoesNotExist { address: info.sender.clone()});
    }

    let mut user = USER.load(deps.storage, info.sender.clone())?;
    // check if the amount is less than balance 

    if amount > user.balance {
        return Err(ContractError::InsufficientBalance { balance: user.balance });
    }

    user.balance -= amount; // decrease the user balance

    USER.save(deps.storage, info.sender.clone(), &mut user)?;

    Ok(Response::new().add_attribute("action", "withdraw"))

}

pub fn rent(deps: DepsMut, info: MessageInfo, car_id: u64, start_date: u64, end_date: u64) -> Result<Response, ContractError> {
    // check if car is available

    let car_key = car_id.to_be_bytes();

    CAR.update(deps.storage, &car_key, |car| {
        if let Some(mut car) = car {
            if car.car_status != Status::Available {
                return Err(ContractError::CarIsNotAvailable { car_id: car_id });
            }
            car.car_status = Status::InUse;
            Ok(car)
        } else {
            Err(ContractError::CarDoesNotExist { car_id: car_id })
        }
    })?;

    // check the dates

    if end_date < start_date {
        return Err(ContractError::InvalidRentDates {})
    }

    let car = CAR.load(deps.storage, &car_key)?;

    // calculate rent cost 
    let rent_cost = car.depositfee+ car.rentfee * u64::from(end_date - start_date) / RENT_PERIOD; 

    
    // check if the user exists
    if !(USER.may_load(deps.storage, info.sender.clone())?).is_some() {
        return Err(ContractError::UserDoesNotExist { address: info.sender.clone() });
    }

    let mut user = USER.load(deps.storage, info.sender.clone())?;

    // check the balance
    if user.balance < rent_cost {
        return Err(ContractError::InsufficientBalance { balance: user.balance });
    }

    // update the balance
    user.balance -= rent_cost; 

    USER.save(deps.storage, info.sender.clone(), &user)?;

    let rent_id = RENT_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(&1u64)))?;

    let rent = Rent {
        id: rent_id, 
        user,
        car_status: car.car_status,
        car_id,
        rent_cost,
        start_date,
        end_date,
    };
    
    // save the rent info
    RENTS.save(deps.storage, rent_id, &rent)?;

    Ok(Response::new().add_attribute("action","rent"))

}

pub fn end_rent(deps: DepsMut,_info: MessageInfo, rent_id: u64) -> Result<Response, ContractError> {

    // get the rent details from the rend_id, throw an error if it does not exist

    // then get the start date from the rent 

    // check the start date be less than end date, throw an error

    // check if the car status is in use

    // change the car status to available


    // check if the rent with id exists
    if !(RENTS.may_load(deps.storage, rent_id)?).is_some() {
        return Err(ContractError::RentDoesNotExist { rent_id: rent_id});
    }

    let rent = RENTS.load(deps.storage, rent_id)?;
    
    let car_key = rent.car_id.to_be_bytes();
    let mut car = CAR.load(deps.storage, &car_key)?;

    // check the car status

    if car.car_status != Status::InUse {
        return Err(ContractError::CarIsNotRentedYet { car_id: rent.car_id });
    } 

    // change the car status

    car.car_status = Status::Available;

    CAR.save(deps.storage, &car_key, &car)?;
    RENTS.save(deps.storage, rent_id, &rent)?;

    Ok(Response::default())
}

