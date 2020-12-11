use std::collections::HashSet;

pub fn find_invalid_number(items: &[i64], preamble_size: usize) -> Option<&i64> {
    let mut index = 0;
    for number in items {
        if index <= preamble_size {
            index = index + 1;
            continue;
        }
        let preamble = &items[index - preamble_size..index];
        let has_two_sum = two_sum(number, preamble);
        if has_two_sum == false {
            return Some(number);
        }
        index = index + 1;
    }
    return None;
}
fn two_sum(target: &i64, items: &[i64]) -> bool {
    let mut items_set = HashSet::new();
    items.iter().for_each(|i|{
        items_set.insert(i);
    });
    for item in items {
        let complement = target - item;
        if complement != *item && items_set.contains(&complement){
            return true;
        }
    }
    return false;
}
pub fn find_subarray_which_sums_to_n(array: &[i64], n: i64) -> Option<&[i64]> {
    let mut curr_sum = 0;
    let mut left_index = 0;
    let mut right_index = 0;
    for (i, x) in array.iter().enumerate() {
        while curr_sum > n && left_index < i - 1 { 
            curr_sum = curr_sum - array[left_index]; 
            left_index = left_index + 1;
        } 
        if curr_sum == n {
            return Some(&array[left_index..right_index]);
        }
        if curr_sum < n {
            curr_sum = curr_sum + array[right_index];
            right_index = right_index + 1;
        }
    }
    return None;
}
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_two_sum(){
        let input = &[
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        assert_eq!(two_sum(&182, &[65, 95, 102, 117, 150]), true);
        assert_eq!(two_sum(&127, &[182, 150, 117, 102, 95]), false);
    }
    #[test]
    fn test_find_invalid_number(){
        let input = &[
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        assert_eq!(find_invalid_number(input, 5).unwrap(), &127);
    }
    #[test]
    fn test_find_subarray_which_sums_to_n(){
        let input = &[
            35,
            20,
            15,
            25,
            47,
            40,
            62,
            55,
            65,
            95,
            102,
            117,
            150,
            182,
            127,
            219,
            299,
            277,
            309,
            576
        ];
        let res = find_subarray_which_sums_to_n(input, 70);
        assert_eq!(res.unwrap(), &[35, 20, 15]);
    }

}
