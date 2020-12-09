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
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_rule(){
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

}
