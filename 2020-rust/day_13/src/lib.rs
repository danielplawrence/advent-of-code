use std::collections::HashMap;

/// Find the nearest multiple of 'multiple' to 'target'
pub fn find_nearest_multiple(target: usize, multiple: usize) -> usize {
    let mut increment = 0;
    loop {
        let res = target + increment;
        if res % multiple == 0 {
            return res;
        }
        increment = increment + 1;
    }
}
/// Given an array of multiples and target minutes
/// Find x, where x + index % multiple = 0 for all pairs
pub fn find_consecutive_buses(input: Vec<&str>) -> i64 {

    let mut residues = vec![];
    let mut valid_buses = vec![];

    for (i, bus) in input.iter().enumerate() {
        if bus == &"x" {
            continue;
        }

        let bus_id = bus.parse::<i64>().unwrap();

        residues.push(bus_id - i as i64);
        valid_buses.push(bus_id);
    }

    match chinese_remainder(&residues, &valid_buses) {
        Some(result) => result,
        None => panic!("No valid value can be found"),
    }
}
/// Do first and second form a pair such that 
/// First + offset is a multiple of second?
fn compare(first: usize, second: usize, offset: usize) -> bool {
    return first + offset % second == 0;
}

pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

pub fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * p * mod_inv(p, modulus)?;
    }

    Some(sum % prod)
}

/// Finds the modular multiplicative inverse
/// https://en.wikipedia.org/wiki/Modular_multiplicative_inverse
pub fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);

    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_nearest_multiple(){
        let target = 100;
        let multiple = 7;
        assert_eq!(find_nearest_multiple(target, multiple), 105);
    }
    #[test]
    fn test_find_consecutive_buses(){
        let input = vec!["7","13","x","x","59","x","31","19"];
        assert_eq!(find_consecutive_buses(input), 1068781);
    }
}