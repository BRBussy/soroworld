use soroban_sdk::Env;

use crate::storage_types::{DataKey, Coordinates};

pub fn read_current_land_coordinate(e: &Env) -> Coordinates {
    let key = DataKey::CurrentLandCoordinate;
    e.storage().instance().get(&key).unwrap_or(Coordinates{x: 0, y: 0})
}

pub fn read_current_land_depth(e: &Env) -> u128 {
    let key = DataKey::CurrentLandDepth;
    e.storage().instance().get(&key).unwrap_or(0)
}

pub fn increment_current_land_depth(e: &Env) {
    let key = DataKey::CurrentLandDepth;
    e.storage().instance().set(&key, &read_current_land_depth(&e));
}

pub fn increment_current_land_coordinate(e: &Env) -> Coordinates {
    let key = DataKey::CurrentLandCoordinate;

    // get current coordinate
    let mut current_coordinates = read_current_land_coordinate(&e);

    // get current land depth
    let current_land_depth = read_current_land_depth(&e);

   // increment coordinates (and land depth if necessary)
   if increment_coordinates(&mut current_coordinates, current_land_depth) {
    // store updated land depth
    increment_current_land_depth(&e);
   }

    // store updated coordinates
    e.storage().instance().set(&key, &current_coordinates);

    // return updated coordinates
    current_coordinates
}

fn increment_coordinates(coordinates: &mut Coordinates, depth: u128) -> bool {
     // check if depth is even or odd
     if depth % 2 == 0 {
        // depth is even

        // check if corner has been reached
        if coordinates.x == depth {
            // corner reached - decrease y until 0
            if coordinates.y == 0 {
                // increment x and depth
                coordinates.x+=1;
                return true;
            } else {
                // decrement y
                coordinates.y-=1;
            }
        } else {
            // corner not yet reached - increase x
            coordinates.x+=1;
        }
    } else {
        // depth is odd

        // check if corner has been reached
        if coordinates.y == depth {
            // corner reached - decrease x until 0
            if coordinates.x == 0 {
                // increment y and depth and go again
                coordinates.y+=1;
                return true;
            } else {
                // decrement x
                coordinates.x-=1;
            }
        } else {
            // corner not yet reached - increase y
            coordinates.y+=1;
        }
    }

    false
}

#[cfg(test)]
pub fn increment_coordinates_for_test(coordinates: &mut Coordinates, depth: u128) -> bool {
    increment_coordinates(coordinates, depth)
}