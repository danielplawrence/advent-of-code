use ::day_04;

fn main () {
    let input = include_str!("input.txt");
    let mut count = 0;
    day_04::split_lines_on_empty_line(input)
    .iter()
    .for_each(|l| {
        if day_04::validate_record(l){
            count = count + 1;
        }
    });
    println!("{}", count);
    let input2 = include_str!("input.txt");
    let count2 = day_04::split_lines_on_empty_line(input2)
    .iter()
    .filter(|l|{
        let i = day_04::Record::new(l);
        return i.is_valid();
    }).count();
    println!("{}", count2);
}

