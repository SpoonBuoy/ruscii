// char_map.rs

use std::collections::HashMap;
use crate::char_seg_map;

pub fn create_char_map() -> HashMap<char, Vec<u8>> {
    let mut char_map = HashMap::new();
    let mut seg_map = HashMap::new();
    let seg_char_vector = char_seg_map::create_seg_char_map();


    let mut idx = 0;
    for char in 'A'..='Z'{
        seg_map.insert(char, seg_char_vector[idx]);
        idx = idx + 1;
    }

    for (char, str) in seg_map{
        let mut v = vec![0;14];
        //println!("{} {:?}", char, str);
        for character in str.chars(){
            let dif = (character as u32 - 'a' as u32) as usize;
            v[dif] = 1;
        }
        //println!("{:?}", v);
        char_map.insert(char, v);
    }
    // Map characters to their respective segment vectors (1 and 0)
    //abcdefg12hijklm
    // char_map.insert('A', vec![
    //     1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0
    // ]);

    char_map
}
