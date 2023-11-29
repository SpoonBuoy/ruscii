// integer_map.rs

use std::collections::HashMap;

pub fn create_integer_map() -> HashMap<usize, Vec<f32>> {
    let mut integer_map = HashMap::new();

    // Map integers to their respective vector of f32 values
    //in the order abcdefg1g2hijklm
    integer_map.insert(0, vec![0.0, 0.0, 0.0, 1.0]);
    integer_map.insert(1, vec![0.0, 1.0, 1.0 / 2.0, 1.0]);
    integer_map.insert(2, vec![1.0 / 2.0, 1.0, 1.0, 1.0]);
    integer_map.insert(3, vec![1.0, 0.0, 1.0, 1.0]);
    integer_map.insert(4, vec![1.0 / 2.0, 0.0, 1.0, 0.0]);
    integer_map.insert(5, vec![0.0, 0.0, 1.0 / 2.0, 0.0]);
    integer_map.insert(6, vec![1.0 / 2.0, 0.0, 1.0 / 2.0, 1.0 / 2.0]);
    integer_map.insert(7, vec![1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0, 1.0]);
    integer_map.insert(8, vec![0.0, 0.0, 1.0 / 2.0, 1.0 / 2.0]);
    
    integer_map.insert(9, vec![0.0, 1.0 / 2.0, 1.0 / 2.0, 1.0 / 2.0]);
    
    integer_map.insert(10, vec![0.0, 1.0, 1.0 / 2.0, 1.0 / 2.0]);
    integer_map.insert(11, vec![1.0 / 2.0, 1.0 / 2.0, 1.0, 0.0]);
    integer_map.insert(12, vec![1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0 / 2.0]);
    integer_map.insert(13, vec![1.0 / 2.0, 1.0 / 2.0, 1.0, 1.0]);

    // Add more integer mappings as needed

    integer_map
}
