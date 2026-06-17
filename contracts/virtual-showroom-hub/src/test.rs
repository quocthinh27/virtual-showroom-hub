#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_add_car() {
    let env = Env::default();

    let contract_id = env.register(VirtualShowroomHub, ());
    let client = VirtualShowroomHubClient::new(&env, &contract_id);

    client.add_car(
        &1,
        &String::from_str(&env, "Model S"),
        &String::from_str(&env, "Tesla"),
        &50000,
    );

    let car = client.get_car(&1);

    assert_eq!(car.id, 1);
    assert_eq!(car.price, 50000);
}

#[test]
fn test_update_price() {
    let env = Env::default();

    let contract_id = env.register(VirtualShowroomHub, ());
    let client = VirtualShowroomHubClient::new(&env, &contract_id);

    client.add_car(
        &1,
        &String::from_str(&env, "Civic"),
        &String::from_str(&env, "Honda"),
        &20000,
    );

    client.update_price(&1, &25000);

    let car = client.get_car(&1);

    assert_eq!(car.price, 25000);
}

#[test]
fn test_total_cars() {
    let env = Env::default();

    let contract_id = env.register(VirtualShowroomHub, ());
    let client = VirtualShowroomHubClient::new(&env, &contract_id);

    client.add_car(
        &1,
        &String::from_str(&env, "Camry"),
        &String::from_str(&env, "Toyota"),
        &30000,
    );

    assert_eq!(client.total_cars(), 1);
}