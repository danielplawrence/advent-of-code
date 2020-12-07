
pub fn get_seat_id(input: &str) -> i32 {
    return get_row(input) * 8 + get_col(input);
}

fn get_row(input: &str) -> i32 {
    let mut seats: Vec<i32> = (0..128).collect();
    let input_iter = String::from(input);
    input_iter.chars().for_each(|c|{
        if c == 'F' {
            seats = seats.split_at(seats.len()/2).0.to_vec();
        } else if c == 'B' {
            seats = seats.split_at(seats.len()/2).1.to_vec();
        }
    });
    return seats[0];
}
fn get_col(input: &str) -> i32 {
    let mut seats: Vec<i32> = (0..8).collect();
    let input_iter = String::from(input);
    input_iter.chars().for_each(|c|{
        if c == 'L' {
            seats = seats.split_at(seats.len()/2).0.to_vec();
        } else if c == 'R' {
            seats = seats.split_at(seats.len()/2).1.to_vec();
        }
    });
    return seats[0];
}
pub fn find_missing(input: &Vec<i32>) -> i32 {
    let mut prev = input[0];
    for item in input {
        if *item == prev {
            continue;
        }
        if *item != prev + 1 {
            return prev + 1;
        } else {
            prev = *item;
        }
    }
    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_missing() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let input = [1, 2, 4].to_vec();
        assert_eq!(find_missing(&input), 3);
    }
    #[test]
    fn test_get_row() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let input = &"BFFFBBFRRR";
        assert_eq!(get_row(input), 70);
        let input = &"FFFBBBFRRR";
        assert_eq!(get_row(input), 14);
        let input = &"BBFFBBFRLL";
        assert_eq!(get_row(input), 102);
    }
    #[test]
    fn test_get_col() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let input = &"BFFFBBFRRR";
        assert_eq!(get_col(input), 7);
        let input = &"FFFBBBFRRR";
        assert_eq!(get_col(input), 7);
        let input = &"BBFFBBFRLL";
        assert_eq!(get_col(input), 4);
    }
    #[test]
    fn test_get_seat_id() {
        // BFFFBBFRRR: row 70, column 7, seat ID 567.
        // FFFBBBFRRR: row 14, column 7, seat ID 119.
        // BBFFBBFRLL: row 102, column 4, seat ID 820.
        let input = &"BFFFBBFRRR";
        assert_eq!(get_seat_id(input), 567);
        let input = &"FFFBBBFRRR";
        assert_eq!(get_seat_id(input), 119);
        let input = &"BBFFBBFRLL";
        assert_eq!(get_seat_id(input), 820);
    }
}
