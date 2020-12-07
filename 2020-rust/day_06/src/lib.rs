use std::collections::HashSet;
use regex::Regex;

pub fn count_questions_for_group(input: &str) -> usize{
    let c: HashSet<char> = input.trim()
    .chars().filter(|c| c!= &'\n' && c!=&' ')
    .collect();
    println!("{:#?}", c);
    return c.len();
}

pub fn split_lines_on_empty_line(input: &str) -> Vec<&str> {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let result: Vec<&str> = re.split(input).into_iter().collect();
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_questions_for_group(){
        let input = "
        fzmwqgcjylr
        goqewcrzfjm
        caqgpmrwz
        ";
        assert_eq!(count_questions_for_group(input), 15);
        let input = "e
        y
        r
        e
        y
        r";
        assert_eq!(count_questions_for_group(input), 3);
    }
}
