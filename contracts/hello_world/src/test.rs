#![cfg(test)]

use super::*;
// use soroban_sdk::{symbol_short, vec, Env};
// use soroban_sdk::{contract, contractimpl, symbol_short,  Vec, Symbol};
use soroban_sdk::{symbol_short};

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
fn test_geo_coordinate_conversion() {
	// Test data: Known latitude and longitude in decimal degrees
	let lat_deg = 40.7128; // Example latitude
	let lon_deg = -74.0060; // Example longitude

	// Create a GeoCoordinate instance from decimal degrees
	let coord = GeoCoordinate::new(lat_deg, lon_deg);

	// Assert that the internal representation matches expectations
	// Note: Directly accessing private fields for testing purposes is generally discouraged
	// and might not be possible depending on your struct visibility settings.
	// This example assumes a way to access these fields for demonstration purposes.
	assert_eq!(coord.latitude, 40712800); // Expected scaled latitude
	assert_eq!(coord.longitude, -74006000); // Expected scaled longitude

	// Convert back to decimal degrees and assert accuracy
	let (lat_back, lon_back) = coord.to_decimal_degrees();
	assert_eq!(lat_back, lat_deg);
	assert_eq!(lon_back, lon_deg);
}
// use soroban_sdk::{testutils::Env as TestEnv, Env, Symbol};

#[test]
fn test_store_and_retrieve_value() {
	// let env = TestEnv::default();
    let env = Env::default();
	let contract_id = env.register_contract(None, SimpleStorageContract);

	let client = SimpleStorageContractClient::new(&env, &contract_id);

	// let key = symbol!("mykey");
	let key = symbol_short!("mykey");
	// let value = 42;
    let value = vec![&env, (2, 20)];
	// let lat_deg = 40.7128; // Example latitude
	// let lon_deg = -74.0060; // Example longitude
    // let value = vec![&env, (lat_deg, lon_deg)];

	// Store the value
	client.store_value(&key, &value);

	// Retrieve the value and verify it
	let retrieved_value = client.retrieve_value(&key);
	assert_eq!(retrieved_value, Some(value));
}
