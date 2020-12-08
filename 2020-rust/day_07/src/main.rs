use ::day_07;

fn main () {
    let input = include_str!("input.txt");
    let mut bags = day_07::Bags::new();
    input.lines().for_each(|i|{
        let res = day_07::parse_rule(i);
        bags.add_bag(res.clone(), &res.name);
    });
    let res = bags.get_bags_containing("shiny gold bag");
    println!("{}", res.len());
    let res2 = bags.count_number_of_bags_in_tree("shiny gold bag", &0);
    println!("{}", res2);
}

