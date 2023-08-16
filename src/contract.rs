use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, Response,
};

use crate::error::ContractError;
use crate::state::{User, Rent, Car, USER, CAR, RENTS, RENT_SEQ};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};

pub const RENT_PERIOD: u64 = 60;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response, ContractError> {

    RENT_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response, ContractError> {
    match msg {
        AddCar { id, name, model, rentfee, depositfee } => add_car(deps, _info, id, model, rentfee) ,
        AddUser { name, lastname } => add_user(deps, info, name, lastname),
        Deposit { amount } => deposit(deps, info, amount),
        Witdhraw { amount } => withdraw(deps, info, amount),
        Rent { car_id, start_date, end_date } => rent( deps, info, car_id, start_date, end_date),
        EndRent { rent_id } => end_rent( deps, info, car_id, start_date, end_date),
    }
}


#[entry_point]
pub fn query(
    deps: DepsMut,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary, ContractError> {
    match msg {
        UserBalance { user_address } => user_balance(deps, user_address),
        RentById { rent_id } => rent_by_id(deps, rent_id),
    }
}
// functions for query

fn user_balance(deps:Deps, user_address: Addr) -> StdResult<Binary> {

    // check if user exists
    let user_key = user_address.as_bytes();
    if !(USER.may_load(deps.storage, user_key)?).is_some() {
        // user does not exist
        return Err(ContractError::UserDoesNotExist { id: user_key, address: user_address });
    }

    let resp = UserBalanceResponse {
        balance: user.balance,
    };    

    to_binary(&resp)
}

fn rent_by_id(deps:Deps, rent_id: u64) -> StdResult<Binary> {
    // check if the rent exists

    let rent_key = rent_id.as_bytes();

    let rent = match RENTS.may_load(deps.storage, rent_key)? {
        Some(rent) => Some(rent),
        None => return Err(StdError::generic_err("Rent does not exist"))
    }
    .unwrap(); 

    let resp = RentResponse {
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

pub fn add_car(deps:DepsMut, _info: MessageInfo, id: u64, name:String , model: String, rentfee: u64, depositfee: u64) -> Result<Response, ContractError> { 
    let car = CAR {
        id,
        name,
        model,
        rentfee,
        car_status: Status::Available,
        depositfee,
    };

    // check if car exists
    let key = car.id.as_bytes();
    if (CAR.may_load(deps.storage, key)?).is_some() {
        // car already exists
        return Err(ContractError::CarAlreadyExists { car_id: id });
    }

    USER.save(deps.storage, key, &car)?;

    Ok(Response::default())
}

pub fn add_user(deps: DepsMut, info: MessageInfo, name: String, lastname: String) -> Result<Response, ContractError>  {

    let user = USER {
        id: info.sender.as_bytes(),
        name,
        lastname,
        address, info.sender,
        balance: 0,
    };

    // check if user exists
    let user_key = user.id;
    if (USER.may_load(deps.storage, user_key)?).is_some() {
        // user already exists
        return Err(ContractError::UserAlreadyExists { id: info.sender.as_bytes() });
    }

    USER.save(deps.storage, user_key, &user)?;

    Ok(Response::default())
}

pub fn deposit(deps: DepsMut, info: MessageInfo, amount: u64) -> Result<Response, ContractError> {

    let user_key = info.sender.as_bytes();
    
    // check if the user exists
    if !(USER.may_load(deps.storage,user_key)?).is_some() {
        return Err(ContractError::UserDoesNotExist {id: user_key});
    }


    let mut user = USER.load(deps.storage, user_key)?;
    user.balance += amount; // increase the user balance

    USER.save(deps.storage, user_key, &user)?;

    Ok(Response::new().add_attribute("action", "deposit"))
}

pub fn withdraw(deps: DepsMut, info: MessageInfo, amount: u64) -> Result<Response, ContractError> {
    let user_key = info.sender.as_bytes();
    
    // check if the user exists
    if !(USER.may_load(deps.storage, user_key)?).is_some() {
        return Err(ContractError::UserDoesNotExist {id: user_key});
    }

    // check if the amount is less than balance 

    if amount > user.balance {
        return Err(ContractError::InsufficientBalance { balance: user.balance });
    }

    user.balance -= amount; // decrease the user balance

    USER.save(deps.storage, user_key, &user)?;

    Ok(Response::new().add_attribute("action", "withdraw"))

}

pub fn rent(deps: DepsMut, info: MessageInfo, car_id: u64, start_date: u64, end_date: u64) -> Result<Response, ContractError> {
    // check if car is available

    let car_key = car_id.as_bytes();

    CAR.update(deps.storage, car_key, |car| {
        if let Some(mut car) = car {
            if car.status != Status::Available {
                return Err(ContractError::CarIsNotAvailable { car_id: car_id });
            }
            car.status = Status::InUse;
            Ok(car)
        } else {
            Err(ContractError::CarDoesNotExist { car_id: car_id });
        }
    })?;

    // check the dates

    if end_date < start_date {
        return Err(ContractError::InvalidRentDates)
    }

    // calculate rent cost 
    let rent_cost = car.deposit_price + car.rent_price * u128::from(end_date - start_date) / RENT_PERIOD; 


    let user_key = info.sender.as_bytes();
    
    // check if the user exists
    if !(USER.may_load(deps.storage, user_key)?).is_some() {
        return Err(ContractError::UserDoesNotExist { id: user_key });
    }

    let mut user = USER.load(deps.storage, user_key)?;

    // check the balance
    if user.balance < rent_cost {
        return Err(ContractError::InsufficientBalance { balance: user.balance });
    }

    // update the balance
    user.balance -= rent_cost; 

    USER.save(deps.storage, user_key, &user)?;

    let rent_id = RENT_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;

    let rent = Rent {
        rent_id: rent_id, 
        user: user,
        car_status: car.status,
        car_id: car_id,
        rent_cost: rent_cost,
        start_date: start_date,
        end_date, end_date,
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
    
    let car_key = rent.car_id.as_bytes();
    let car = CAR.load(deps.storage, car_key)?;

    // check the car status

    if car.status != Status::InUse {
        return Err(ContractError::CarIsNotRentedYet { car_id: rent.car_id });
    } 

    // change the car status

    car.status = Status::Available;

    CAR.save(deps.storage, car_key)?;
    RENTS.save(deps.storage, rent_id)?;

    Ok(Response::default())
}

