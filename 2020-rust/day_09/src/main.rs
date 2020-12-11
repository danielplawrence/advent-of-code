use ::day_09;

fn main () {
    let input = include_str!("input.txt");
    // Part 1
    let items: Vec<i64> = input
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();
    let res = day_09::find_invalid_number(&items, 25);
    println!("{:?}", res);
    // Part 2
    let mut res = day_09::find_subarray_which_sums_to_n(&items, 542529149).unwrap();
    let max = res.iter().max().unwrap();
    let min = res.iter().min().unwrap();
    let final_result = max + min;
    println!("{:?}", final_result);
}