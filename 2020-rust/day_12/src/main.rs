use ::day_12;
use regex::Regex;

fn main () {
 // Part 1
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
 // Part 2
 let mut ship = day_12::Ship::new();
 let mut waypoint = day_12::Waypoint::new(10, 1);
 input
 .lines()
 .map(|l| l.trim())
 .filter(|l| !l.is_empty())
 .for_each(|l: &str|{
     let cmd_pattern = Regex::new(r"[A-Z]").unwrap();
     let digit_pattern = Regex::new(r"\d+").unwrap();
     let cmd = cmd_pattern.captures(l).unwrap().get(0).unwrap().as_str().chars().nth(0).unwrap();
     let val: isize = digit_pattern.captures(l).unwrap().get(0).unwrap().as_str().parse().unwrap();
     if cmd == 'F' {
        ship.move_by_waypoint(&waypoint, &val);
     }
     if cmd == 'R' {
        waypoint.rotate(&-val);
     }
     if cmd == 'L' {
        waypoint.rotate(&val);
     }
     if cmd == 'N' {
        waypoint.north(&val);
     }
     if cmd == 'S' {
        waypoint.south(&val);
     }
     if cmd == 'E' {
        waypoint.east(&val);
     }
     if cmd == 'W' {
        waypoint.west(&val);
     }
 });
 println!("{:#?}", ship);
}