use std::collections::HashSet;

pub struct Command {
    command: String,
    arg: i32
}
pub fn find_loop(input: &str) -> i32 {
    let commands: Vec<Command> = input.lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .map(|l|{
        return Command {
            command: String::from(l.split_whitespace().nth(0).unwrap()),
            arg: l.split_whitespace().nth(1).unwrap().parse().unwrap(),
        }
    }).collect();
    let mut counter : i32 = 0;
    let mut acc = 0;
    let mut indices = HashSet::<i32>::new();
    loop {
        if indices.contains(&counter){
            break;
        }
        indices.insert(counter);
        let cmd: &Command = &commands[counter as usize];
        if cmd.command == "nop" {
            counter = counter + 1;
        } else if cmd.command == "acc" {
            acc = acc + cmd.arg;
            counter = counter + 1;
        } else if cmd.command == "jmp" {
            counter = counter + cmd.arg;
        }
    }
    return acc
}
// Flip the no-op/jmp instruction at index 'replace' 
// Return the value of the accumulator directly before an infinite loop (second val of tuple will be 'true')
// ...or before normal termination (second val of tuple will be 'false')
pub fn find_loop_replace_instruction(input: &str, replace: &i32) -> (i32, bool){
    let commands: Vec<Command> = input.lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .map(|l|{
        return Command {
            command: String::from(l.split_whitespace().nth(0).unwrap()),
            arg: l.split_whitespace().nth(1).unwrap().parse().unwrap(),
        }
    }).collect();
    let mut counter : i32 = 0;
    let mut acc = 0;
    let instructions_length = commands.len();
    let mut indices = HashSet::<i32>::new();
    loop {
        if counter >= instructions_length as i32 {
            return (acc, false);
        }
        if indices.contains(&counter){
            return (acc, true);
        }
        indices.insert(counter);
        let cmd: &Command = &commands[counter as usize];
        let mut command = String::from(&cmd.command);
        if counter == *replace {
            if &command == &"nop".to_string(){
                command = String::from("jmp");
            } else if &command == &"jmp".to_string(){
                command = String::from("nop");
            }
        }
        if &command == &"nop".to_string() {
            counter = counter + 1;
        } else if &command == &"acc".to_string() {
            acc = acc + cmd.arg;
            counter = counter + 1;
        } else if &command == &"jmp".to_string() {
            counter = counter + cmd.arg;
        }
    }
}
pub fn find_instructions(input: &str, instructions: &[String]) -> Vec<i32> {
    let mut instructions_set = HashSet::new();
    instructions.into_iter().for_each(|i|{
        instructions_set.insert(i);
    });
    let commands: Vec<Command> = input.lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .map(|l|{
        return Command {
            command: String::from(l.split_whitespace().nth(0).unwrap()),
            arg: l.split_whitespace().nth(1).unwrap().parse().unwrap(),
        }
    }).collect();
    let mut result = Vec::<i32>::new();
    let mut counter = 0;
    for command in commands {
        if instructions_set.contains(&command.command){
            result.push(counter);
        }
        counter = counter + 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_loop(){
        let input = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
        ";
        let res = find_loop(input);
        assert_eq!(res, 5);
    }
    #[test]
    fn test_find_loop_replace_instruction(){
        let input = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
        ";
        let res = find_loop_replace_instruction(input, &100);
        assert_eq!(res, (5, true));

        let input = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
        ";
        let res = find_loop_replace_instruction(input, &0);
        assert_eq!(res, (0, true));

        let input = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        nop -4
        acc +6 
        ";
        let res = find_loop_replace_instruction(input, &4);
        assert_eq!(res, (8, false));
    }
    #[test]
    fn test_find_instructions(){
        let input = "
        nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6
        ";
        let res = find_instructions(input, &["nop".to_string()]);
        assert_eq!(res[0], 0);
        let res = find_instructions(input, &["acc".to_string()]);
        assert_eq!(res[0], 1);
        assert_eq!(res[1], 3);
        assert_eq!(res[2], 5);
        assert_eq!(res[3], 6);
        assert_eq!(res[4], 8);
    }


}
