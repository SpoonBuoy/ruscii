### Rusci
#### Rust-based Heuristic Approach for Segment 14 Display

This Rust program employs a heuristic approach to display strings on the terminal in a Segment 14 display fashion. The program takes input strings and converts them into an Character art representation using the Segment 14 display style.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/SpoonBuoy/ruscii.git

   cd ruscii

   cargo build

   cargo run 
 
    ``````
2. It will create an interactive session which will listen for commands 

   ```
     [size of each character] [characters on each line] [spacing between characters] [highlight character which will be used to print string] [input string] 
   ``````
    ``` 
    "quit" to exit the program
    ```
3. Example 
    ```
    Input : 4 5 2 + abc

    Output:
    +++++  +++++  +++++  
    +   +    + +  +      
    +++++    +++  +      
    +   +    + +  +      
    +   +  +++++  +++++

    ```