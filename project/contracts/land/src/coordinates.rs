use soroban_sdk::Env;

use crate::storage_types::{DataKey, Coordinates};

// pub fn read_coordinates(e: &Env) -> Coordinates {
//     let key = DataKey::Coordinates;
//     e.storage().instance().get::<_, Coordinates>(&key).unwrap()
// }

pub fn write_coordinates(e: &Env, coordinates: &Coordinates) {
    let key = DataKey::Coordinates;
    e.storage().instance().set(&key, coordinates);
}