#![no_std]
// use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, Map};
use soroban_sdk::{contract, contractimpl, Env, Vec, Symbol, Map, symbol_short, vec};

#[contract]
pub struct KeyValueContract;
#[contractimpl]
impl KeyValueContract {
    // Set a key-value pair
    pub fn set_value(env: Env, key: Symbol, value: i32) {
        env.storage().set(&key, &value);
    }

    // Get a value by key
    pub fn get_value(env: Env, key: Symbol) -> Option<i32> {
        env.storage().get(&key)
    }
}

// #[contract]
// pub struct GISContract;
//
// #[contractimpl]
// impl GISContract {
//     pub fn set_location(env: Env, key: Symbol, latitude: i32, longitude: i32) {
//         // Create a composite key by appending "_lat" and "_lon" to the main key
//
// 		let lat_key = Symbol::from(key.to_string_lossy() + "_lat");
// 		let lon_key = Symbol::from(key.to_string_lossy() + "_lon");
//
//         // Store latitude and longitude in the environment's storage
//         env.storage().set(&lat_key, &latitude);
//         env.storage().set(&lon_key, &longitude);
//     }
//
//     pub fn get_location(env: Env, key: Symbol) -> Option<(i32, i32)> {
//         // Create a composite key by appending "_lat" and "_lon" to the main key
// 		let lat_key = Symbol::from(key.to_string_lossy() + "_lat");
// 		let lon_key = Symbol::from(key.to_string_lossy() + "_lon");
//
//         // Retrieve latitude and longitude from the environment's storage
//         let latitude: Option<i32> = env.storage().get(&lat_key);
//         let longitude: Option<i32> = env.storage().get(&lon_key);
//
//         // Return the coordinates if both latitude and longitude exist
//         if let (Some(lat), Some(lon)) = (latitude, longitude) {
//             Some((lat, lon))
//         } else {
//             None
//         }
//     }
// }
// #[contract]
// pub struct GeoCoordinate {
//     latitude: i64, // Fixed-point representation
//     longitude: i64, // Fixed-point representation
// }
//
// impl GeoCoordinate {
//     pub fn new(latitude_deg: f64, longitude_deg: f64) -> Self {
//         let scale_factor = 1_000_000.0; // Scale factor for precision
//         let latitude = (latitude_deg * scale_factor) as i64;
//         let longitude = (longitude_deg * scale_factor) as i64;
//
//         Self { latitude, longitude }
//     }
//
//     // Methods to convert back to decimal degrees
//     pub fn to_decimal_degrees(&self) -> (f64, f64) {
//         let scale_factor = 1_000_000.0;
//         let latitude_deg = self.latitude as f64 / scale_factor;
//         let longitude_deg = self.longitude as f64 / scale_factor;
//
//         (latitude_deg, longitude_deg)
//     }
// }
//
// #[contract]
// pub struct GisData {
//     entries: Vec<GeoCoordinate>,
// }
//
// impl GisData {
//     pub fn new() -> Self {
//         Self { entries: Vec::new() }
//     }
//
//     pub fn add_entry(&mut self, entry: GeoCoordinate) {
//         self.entries = vec![entry];
//     }
//
//     // Returning the entire vector might not be practical or supported in the same way as Rust's Vec<T>.
//     // Consider adjusting this method based on how you intend to use the data outside of the contract.
//     pub fn get_entries(&self) -> Vec<GeoCoordinate> {
//         self.entries.clone()
//     }
// }

mod test;
