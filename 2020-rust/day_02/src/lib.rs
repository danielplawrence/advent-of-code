/// Represents a password policy --
/// target character and min/max occurrances permitted
pub struct Policy {
    pub character: char,
    pub min: usize,
    pub max: usize
}

/// Given a string and a policy, return true iff the string
/// conforms to the policy
pub fn check_pass(input: &String, policy: &Policy) -> bool {
    let count = input.matches(policy.character).count();
    if count <= policy.max && count >= policy.min {
        return true;
    }
    return false;
}

/// Given a string and a policy, return true iff the string
/// conforms to the policy -- this time, 'min' and 'max'
/// are indices where exactly one must contain the target
pub fn check_pass_2(input: &String, policy: &Policy) -> bool {
    let first = &input.chars().nth(policy.min - 1).unwrap();
    let second = &input.chars().nth(policy.max - 1).unwrap();
    let first_matches = first == &policy.character;
    let second_matches = second == &policy.character;
    if first_matches && second_matches{
        return false;
    }
    if first_matches {
        return true;
    }
    if second_matches {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pass_true() {
        let password = String::from("abcde");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_pass_false_min() {
        let password = String::from("bcde");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_pass_false_max() {
        let password = String::from("aaaabcde");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_pass_2_true_first_match() {
        let password = String::from("abcd");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_pass_2_true_second_match() {
        let password = String::from("cbad");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_pass_2_false_no_match() {
        let password = String::from("defg");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, false);
    }

    #[test]
    fn test_check_pass_2_false_both_match() {
        let password = String::from("aaaa");
        let policy = Policy {character: 'a', min: 1, max: 3};
        let result = check_pass(&password, &policy);
        assert_eq!(result, false);
    }

}