use soroban_sdk::Env;

use crate::storage_types::{DataKey, Coordinates};

pub fn write_current_land_coordinate(e: &Env, coordinates: &Coordinates) {
    let key = DataKey::CurrentLandCoordinate;
    e.storage().instance().set(&key, coordinates);
}

pub fn read_current_land_coordinate(e: &Env) -> Coordinates {
    let key = DataKey::CurrentLandCoordinate;
    e.storage().instance().get(&key).unwrap()
}

pub fn increment_current_land_coordinate(e: &Env, coordinates: &Coordinates) {
    let key = DataKey::CurrentLandCoordinate;
    e.storage().instance().set(&key, coordinates);
}