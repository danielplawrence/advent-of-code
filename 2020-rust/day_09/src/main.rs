use ::day_09;

fn main () {
    let input = include_str!("input.txt");
    let items: Vec<i64> = input
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
    let res = day_09::find_invalid_number(&items, 25);
    println!("{:?}", res);
}