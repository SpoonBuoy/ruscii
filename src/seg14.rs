use crate::char_map;
use crate::integer_map;

pub struct Seg14{
    rows: usize, 
    cols: usize,
    highlight: char,
    data: Vec<Vec<char>>,
}
#[derive(Debug)]
pub struct Segment{
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Seg14{
    pub fn new(rows: usize, cols: usize, default_char: char, highlight:char) -> Seg14{
        Seg14{
            rows,
            cols,
            highlight,
            data: vec![vec![default_char; cols + 1]; rows + 1],
        }
    }
    pub fn draw(&mut self, placeholder: char) -> Option<()>{
        let char_mapping = char_map::create_char_map();
        let int_mapping = integer_map::create_integer_map();

        if let Some(segs) = char_mapping.get(&placeholder){
            for(index, &value) in segs.iter().enumerate(){
                if value==1{
                    //println!("{}", index);
                    //this index segment has to be diplayed
                    let v = &int_mapping[&index];
                    let cur_seg = Segment{
                        x1: v[0],
                        y1: v[1],
                        x2: v[2],
                        y2: v[3],
                    };
                    self.draw_segment(cur_seg);

                }
            }
        }
        Some(())
    }
    pub fn get_seg(&self) -> Vec<Vec<char>>{
        self.data.clone()
    }

    pub fn display(&self){
        print!("Displaying.. \n");
        for row in &self.data {
            //print!("{:?}", row);
            for &char in row {
                print!("{}", char);
            }
            println!();
        }
    }
    fn draw_segment(&mut self, seg: Segment){
        if seg.x1 == seg.x2{
            // Draw vertical segment
            for y in (seg.y1*(self.cols as f32)) as usize..=(seg.y2*(self.cols as f32)) as usize {
                self.data[(seg.x1*(self.rows as f32)) as usize][y] = self.highlight;
            }
        }else if seg.y1 == seg.y2{
            // Draw vertical segment
            for x in (seg.x1*(self.rows as f32))as usize..=(seg.x2*(self.rows as f32))as usize {
                self.data[x][(seg.y1*(self.cols as f32)) as usize] = self.highlight;
            }
        }else{
            
            let add : bool = if (2.0*seg.y2) as i32 > (2.0*seg.y1) as i32 {true} else {false};
            let mut y = (seg.y1*(self.cols as f32)) as i32;
            for x in (seg.x1*(self.rows as f32))as usize..=(seg.x2*(self.rows as f32))as usize {
                self.data[x][y as usize] = self.highlight;
               // println!("{}", y);
               //if x == y as usize {break;}
                if add{
                    y = std::cmp::min(self.cols as i32, y + 1);
                }else{
                    y = std::cmp::max(0, y - 1);
                }
            }
        }
    }
}