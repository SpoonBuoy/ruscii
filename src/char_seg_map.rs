
//maps the character to each on segment in 14 segment display
//       aaaaaaa
//   f  i  j  k   b
//   f   i j k    b
//       ggghhh
//   e   l m n    c
//   e  l  m  n   c
//       ddddddd      dp


pub fn create_seg_char_map() -> Vec<&'static str>{
    let seg_char_vector = vec![
         "abcfegh", "abcdjmh", "afed", "abcdjm", "afedgh", "afegh", "afedch", "febcgh",
         "adjm", "bcde", "knjm", "fed", "febcik", "febcin", "abcdef", "abfegh", "abcdef",
        "abfehn", "afcdgh", "ajm", "fedbc", "ik", "fedbcjm", "inkl", "ikm", "adkl"
    ];
    seg_char_vector
}