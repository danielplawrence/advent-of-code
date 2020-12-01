use std::collections::HashSet;

/// Given a set of values and a target value n,
/// find any two entries which sum to n
pub fn two_sum(input: &HashSet<u32>, target: &u32) -> (u32, u32) {
    for entry in input {
        let diff = target - entry;
        if input.contains(&diff) {
            return (*entry, diff);
        }
    }
    panic!("No entry found");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let data: HashSet<u32> = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
        let result: (u32, u32) = two_sum(&data, &2020);
        assert_eq!(result.0, 299);
        assert_eq!(result.1, 1721);
    }

}