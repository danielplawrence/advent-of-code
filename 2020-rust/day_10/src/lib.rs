use std::collections::HashMap;

pub fn get_differences(input: &[usize]) -> HashMap<usize, usize> {

    //Sort by size
    let mut arr: Vec<usize> = input.to_vec();
    arr.sort();
    //Add the initial adapter
    arr.insert(0, 0);
    //Add the final adapter (max + 3)
    let max = input.iter().max().unwrap();
    arr.push(max + 3);
    let mut res: HashMap<usize, usize> = HashMap::new();
    let mut counter = 0;
    for elem in &arr {
        if counter == 0 {
            counter = counter + 1;
            continue;
        }
        let diff = elem - arr[counter-1];
        if res.contains_key(&diff){
            let newval = res.get(&diff).unwrap() + 1;
            res.insert(
                diff,
                newval
            );
        } else {
            res.insert(diff, 1);
        }
        counter = counter + 1;
    }
    return res;
}
pub fn count_valid_combinations(input: &[i64]) -> i64 {
    //Sort by size
    let mut arr: Vec<i64> = input.to_vec();
    arr.sort();
    //Add the initial adapter
    arr.insert(0, 0);
    //Add the final adapter (max + 3)
    let max = input.clone().iter().max().unwrap();
    arr.push(max + 3);
    //Keep a map of the result for each item
    let mut paths: HashMap<i64, i64> = HashMap::new();
    paths.insert(0, 1);
    paths.insert(1, 1);
    for &v in &arr[2..] {
        //The number of paths for item i 
        //is the sum of the number of paths for the previous three items
        let opts = &[1, 2, 3];
        let res: i64 = opts.iter().map(|opt|{
            let val = paths.get(&(v - opt)).unwrap_or(&0);
            if val < &0 {
                return 0;
            }
            return *val;
        }).sum();
        paths.insert(v, res);
    }
    *paths.get(arr.last().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_differences(){
        let input = [
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
        ];
        let mut expected = HashMap::new();
        expected.insert(1, 22);
        expected.insert(3, 10);
        assert_eq!(get_differences(&input), expected);
    }
    #[test]
    fn test_count_valid_combinations(){
        let input = [
            1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19
        ];
        let expected = 8;
        assert_eq!(count_valid_combinations(&input), expected);
    }
}
