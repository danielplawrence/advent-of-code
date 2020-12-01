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

/// Given a set of values and a target value n,
/// find any three entries which sum to n
pub fn three_sum(input: &HashSet<u32>, target: &u32) -> (u32, u32, u32) {
    for first_entry in input {
        for second_entry in input {
            if first_entry + second_entry > *target {
                continue;
            }

            let diff = target - first_entry - second_entry;

            if input.contains(&diff) {
                return (*first_entry, *second_entry, diff)
            }
        }
    }
    panic!("No matching triplet found");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let data: HashSet<u32> = vec![1721, 979, 366, 299, 675, 1456].into_iter().collect();
        let result: (u32, u32) = two_sum(&data, &2020);
        let sum = result.0 + result.1;
        assert_eq!(sum, 2020);
    }

    #[test]
    fn test_three_sum() {
        let data: HashSet<u32> = vec![1720, 979, 366, 299, 675, 1456].into_iter().collect();
        let result: (u32, u32, u32) = three_sum(&data, &2020);
        let sum = result.0 + result.1 + result.2;
        assert_eq!(sum, 2020);
    }

}