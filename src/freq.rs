use std::collections::HashMap;

// Create a hashmap of frequencies in a byte map.
pub fn gen_frequency(bytes: &Vec<u8>) -> HashMap::<u8, usize> {
    let frequency = bytes.iter().fold(HashMap::<u8, usize>::new(), | mut map, curr | {
        if !map.contains_key(curr) {
            map.insert(curr.clone(), 0);
        }
        map.insert(curr.clone(), map.get(curr).unwrap() + 1);
        map
    });
    frequency
}
