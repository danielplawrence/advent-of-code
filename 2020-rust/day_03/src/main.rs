use ::day_03;

fn main () {

    //Part 1
    let res = day_03::count_elem_offset(include_str!("input.txt"), '#', &3, &1);
    println!("{} trees", res);

    //Part 2
    let r_1_d_1 = day_03::count_elem_offset(include_str!("input.txt"), '#', &1, &1);
    let r_3_d_1 = day_03::count_elem_offset(include_str!("input.txt"), '#', &3, &1);
    let r_5_d_1 = day_03::count_elem_offset(include_str!("input.txt"), '#', &5, &1);
    let r_7_d_1 = day_03::count_elem_offset(include_str!("input.txt"), '#', &7, &1);
    let r_1_d_2 = day_03::count_elem_offset(include_str!("input.txt"), '#', &1, &2);

    println!("Answer: {}", r_1_d_1 * r_3_d_1 * r_5_d_1 * r_7_d_1 * r_1_d_2);

}

