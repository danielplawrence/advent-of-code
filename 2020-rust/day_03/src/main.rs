use ::day_03;

fn main () {
    let res = day_03::count_elem_offset(include_str!("input.txt"), '#', &3);
    println!("{} trees", res);
}

