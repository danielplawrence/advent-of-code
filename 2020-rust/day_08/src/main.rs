use ::day_08;

fn main () {
    let input = include_str!("input.txt");
    let res = day_08::find_loop(input);
    println!("{}", res);
    let target_instructions = day_08::find_instructions(input, &["jmp".to_string(), "nop".to_string()]);
    for instruction in target_instructions {
        let r = day_08::find_loop_replace_instruction(input, &instruction);
        if (!r.1){
            println!("{}", r.0);
        }
    }
}

