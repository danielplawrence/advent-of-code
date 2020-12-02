use ::day_02;

fn main () {
    // Part 1
    let input = parse_input(include_str!("input.txt"));
    let mut count = 0;
    for line in input {
        if day_02::check_pass(&String::from(line.0), &line.1) {
            count = count + 1;
        }
    }
    println!("Number of valid passwords: {}", count);
    // Part 2
    let input2 = parse_input(include_str!("input.txt"));
    count = 0;
    for line in input2 {
        if day_02::check_pass_2(&String::from(line.0), &line.1) {
            count = count + 1;
        }
    }
    println!("Number of valid passwords: {}", count);
}

pub fn parse_line(line: &str) -> (&str, day_02::Policy){
    let mut chunks = line.split_whitespace();
    let mut min_max = chunks.next().unwrap().split('-');
    let min = min_max.next().unwrap().parse::<usize>().unwrap();
    let max = min_max.next().unwrap().parse::<usize>().unwrap();
    let character: char = chunks.next().unwrap().chars().next().unwrap();
    let password = chunks.next().unwrap();
    return (&password, day_02::Policy{character, min, max})
}
pub fn parse_input(input: &str) -> Vec<(&str, day_02::Policy)> {
    return input
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .map(|l| parse_line(l))
    .collect::<Vec<_>>();
}
