use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

pub fn count_questions_for_group(input: &str) -> usize {
    let c: HashSet<char> = input.trim()
    .chars().filter(|c| c!= &'\n' && c!=&' ')
    .collect();
    println!("{:#?}", c);
    return c.len();
}

pub fn count_questions_for_group_all_yes(input: &str) -> i32 {
    let member_count = input.lines().count();
    let char_counts = input.chars().filter(|c| c!= &'\n' && c!=&' ')
    .fold(HashMap::new(), |mut freqs, value| {
        *freqs.entry(value).or_insert(0) += 1;
        freqs
    });
    let mut count = 0;
    for (_key, value) in char_counts {
        if value == member_count {
            count = count + 1;
        }
    }
    return count;
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

    #[test]
    fn test_count_questions_for_group_all_yes(){
        let input = "a
        a
        a";
        assert_eq!(count_questions_for_group_all_yes(input), 1);
    }
}
