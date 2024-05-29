#![cfg(test)]

use super::*;
// use soroban_sdk::{symbol_short, vec, Env};
use soroban_sdk::{contract, contractimpl, symbol_short,  Vec, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);

    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}
#[test]
// fn test_set_get_location() {
// 	let env = Env::default();
// 	let contract_id = env.register_contract(None, GISContract);
//
// 	let key = Symbol::from("location1");
// 	let latitude = 12345678;
// 	let longitude = 87654321;
//
// 	// Set the location
// 	GISContract::set_location(env.clone(), key.clone(), latitude, longitude);
//
// 	// Get the location and verify the values
// 	let result = GISContract::get_location(env.clone(), key.clone());
// 	assert_eq!(result, Some((latitude, longitude)));
//
// 	// Overwrite the location with new values
// 	let new_latitude = 87654321;
// 	let new_longitude = 12345678;
// 	GISContract::set_location(env.clone(), key.clone(), new_latitude, new_longitude);
//
// 	// Get the location and verify the updated values
// 	let result = GISContract::get_location(env.clone(), key.clone());
// 	assert_eq!(result, Some((new_latitude, new_longitude)));
// }
