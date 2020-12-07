use ::day_06;

fn main () {
    // part 1
    let input = include_str!("input.txt");
    let mut count = 0;
    day_06::split_lines_on_empty_line(input)
    .iter()
    .for_each(|e|{
        let thiscount = day_06::count_questions_for_group(e);
        println!("{} {}", e, thiscount);
        count = count + thiscount;
    });
    println!("{}", count);
    // part 2
    let mut count = 0;
    day_06::split_lines_on_empty_line(input)
    .iter()
    .for_each(|e|{
        let thiscount = day_06::count_questions_for_group_all_yes(e);
        println!("{} {}", e, thiscount);
        count = count + thiscount;
    });
    println!("{}", count);

}

