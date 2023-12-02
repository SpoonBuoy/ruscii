mod seg14;  
mod screen;       // Import the Seg14 module from seg14.rs
mod char_map;      // Import the char_map module from char_map.rs
mod integer_map;   // Import the integer_map module from integer_map.rs
mod char_seg_map;
use std::io;

use seg14::Seg14; 
use screen::Screen; // Use the Seg14 struct


fn display(str: &str, size: usize, chars_on_each_line: usize, spacing: usize, highlight: char ) -> Option<()>{
     let mut screen = Screen::new(size + 1, 4);
    for char in str.chars(){
        let mut d = Seg14::new(size, size, ' ', highlight);
        d.draw(char).unwrap();
        let v = d.get_seg();
        screen.put(v, spacing, ' ');
        if char == 'Q'{
            screen.put_char(highlight, spacing);
        }
        
    }
    //width of each char = cols + spacing + 1
    let width = chars_on_each_line * (size + 1 + spacing);
    screen.fill(width);

    Some(())
}
fn main() {
    let mut size: usize = 8;
    let mut chars_on_each_line: usize = 20;
    let mut spacing:usize = 2;
    let mut highlight:char = '@';
    let str = "RUSCI";
   
    display(str, size, chars_on_each_line, spacing, highlight);

     //width = chars_on_each_line * (size + 1) + spacing;
    println!("[Size] : {} [Chars on each line] : {}, [Spacing] : {}, [Highlight] : {}, [String] : {}",
            size, chars_on_each_line, spacing, highlight, str);
    
    println!("Enter the command in that order");
    println!("ex : 6 2 @ example");

    
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input_values: Vec<&str> = input.trim().split_whitespace().collect();

        if input_values[0] == "quit"{
            break;
        }

        size = input_values[0].parse().expect("Invalid Size");
        chars_on_each_line = input_values[1].parse().expect("Invalid Number for Chars on each line");
        spacing = input_values[2].parse().expect("Invalid Spacing");
        highlight = input_values[3].chars().next().expect("Invalid highlight character");
        let str = input_values[4].to_uppercase();
        display(&str, size, chars_on_each_line, spacing, highlight);
    }

    
}