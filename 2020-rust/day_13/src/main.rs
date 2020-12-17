use ::day_13;

fn main () {
   // Part 1
   let target = 1000677;
   let multiples = &[29, 41, 661, 13, 17, 23, 521, 37, 19];
   let mut curr_min = 0;
   let mut res = multiples.iter().map(|i|{
      return (day_13::find_nearest_multiple(target, *i), i);
   }).min().unwrap();
   println!("{:#?}", res);
   // Part 2
   let input = include_str!("input.txt");
   let buses = input.lines().nth(1).unwrap().split(',').collect();
   let res = day_13::find_consecutive_buses(buses);
   println!("{:#?}", res);
}