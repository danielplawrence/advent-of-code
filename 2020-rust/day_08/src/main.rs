use ::day_08;

fn main () {
    let input = include_str!("input.txt");
    let res = day_08::find_loop(input);
    println!("{}", res);
}

