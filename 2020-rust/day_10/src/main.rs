use ::day_10;

fn main () {
    let input = include_str!("input.txt");
    // Part 1
    let items: Vec<usize> = input
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
    let res = day_10::get_differences(&items);
    println!("{:?}", res);

    let items: Vec<i64> = input
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
    let res = day_10::count_valid_combinations(&items);
    println!("{:?}", res);
}