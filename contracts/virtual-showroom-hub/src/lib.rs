#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol};

#[derive(Clone)]
#[contracttype]
pub struct Car {
    pub id: u32,
    pub name: String,
    pub brand: String,
    pub price: u32,
}

const CAR_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct VirtualShowroomHub;

#[contractimpl]
impl VirtualShowroomHub {

    pub fn add_car(
        env: Env,
        id: u32,
        name: String,
        brand: String,
        price: u32,
    ) {
        let car = Car {
            id,
            name,
            brand,
            price,
        };

        env.storage().persistent().set(&id, &car);

        let count: u32 = env
            .storage()
            .persistent()
            .get(&CAR_COUNT)
            .unwrap_or(0);

        env.storage().persistent().set(&CAR_COUNT, &(count + 1));
    }

    pub fn get_car(env: Env, id: u32) -> Car {
        env.storage()
            .persistent()
            .get(&id)
            .unwrap()
    }

    pub fn update_price(
        env: Env,
        id: u32,
        new_price: u32,
    ) {
        let mut car: Car = env
            .storage()
            .persistent()
            .get(&id)
            .unwrap();

        car.price = new_price;

        env.storage().persistent().set(&id, &car);
    }

    pub fn remove_car(env: Env, id: u32) {
        env.storage().persistent().remove(&id);

        let count: u32 = env
            .storage()
            .persistent()
            .get(&CAR_COUNT)
            .unwrap_or(1);

        env.storage().persistent().set(&CAR_COUNT, &(count - 1));
    }

    pub fn total_cars(env: Env) -> u32 {
        env.storage()
            .persistent()
            .get(&CAR_COUNT)
            .unwrap_or(0)
    }
}