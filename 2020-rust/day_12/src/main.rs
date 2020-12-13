use ::day_12;
use regex::Regex;

fn main () {
 let input = include_str!("input.txt");
 let mut ship = day_12::Ship::new();
 input
 .lines()
 .map(|l| l.trim())
 .filter(|l| !l.is_empty())
 .for_each(|l: &str|{
     let cmd_pattern = Regex::new(r"[A-Z]").unwrap();
     let digit_pattern = Regex::new(r"\d+").unwrap();
     let cmd = cmd_pattern.captures(l).unwrap().get(0).unwrap().as_str().chars().nth(0).unwrap();
     let val: isize = digit_pattern.captures(l).unwrap().get(0).unwrap().as_str().parse().unwrap();
     ship.move_ship(cmd, val);
 });
 println!("{:#?}", ship);

}