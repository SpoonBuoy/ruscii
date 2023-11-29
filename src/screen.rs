pub struct Screen{
    height: usize,
    width: usize,
    data: Vec<Vec<char>>,
}

impl Screen{
    pub fn new(height:usize, width:usize)-> Screen{
        Screen{
            height,
            width, 
            data: vec![vec![]; height]
        }
    }
    pub fn put(&mut self, character:Vec<Vec<char>>, spacing:usize, delimeter:char){
        for i in 0..self.height{
            self.data[i].extend(character[i].clone());
            for _ in 0..spacing{
                self.data[i].push(delimeter);
            }
        }
    }
    pub fn put_char(&mut self, character:char, spacing:usize) -> Option<()>{
        let rows = self.height;
        let cols = self.data[rows - 1].len();
        self.data[rows - 1][cols - spacing] = character;
        // for i in 0..self.height{
        //     if i < self.height - 1{
        //         self.data[i].push(' ');
        //     }else{
        //         self.data[i].push(character);
        //     }
            
        // }
        Some(())
    }
    pub fn fill(&self, width:usize) -> Option<()>{
        let actual_width = self.data[0].len() as usize;
        let mut width_start = 0 as usize;
        let mut width_end = std::cmp::min(actual_width, width);
        loop {
            for row in 0..self.height{
                for col in width_start..std::cmp::min(actual_width, width_end){
                    print!("{}", self.data[row][col]);
                }
                println!();
            }
            println!();
            if width_end > actual_width {break;}
            width_start = width_end;
            width_end = width_end + width;
        }
        Some(())
    }
}