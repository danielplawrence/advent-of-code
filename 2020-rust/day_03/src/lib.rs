pub fn count_elem_offset(input: &str, target: char, offset: &usize) -> u32 {
    let mut pos = 0;
    let mut tree_count = 0;
    input
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .for_each(|l|{
        if pos >= l.chars().count() {
            pos = pos - l.chars().count()
        }
        let c = l.chars().nth(pos);
        if c.is_some() && c.unwrap() == target {
            tree_count = tree_count + 1;
        } else {
            println!("Found no trees in line");
        }
        pos = pos + offset;
    });
    return tree_count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_count_elem() {
        let input = String::from("
        .##.....#....#....#..#.#...#.##
        .###........#.##....#......#..#
        #..#..#.....#...#....#.#.......
        ");
        let count = count_elem_offset(&input, '#', &3);
        assert_eq!(count, 2);
    }
    #[test]
    fn test_count_elem_wrap() {
        //We need to 'wrap around' when the
        //index goes off the right-hand side
        //of the map
        let input = String::from("
        .##.....
        ..##....
        #..#..#.
        ########
        ########
        ");
        let count = count_elem_offset(&input, '#', &3);
        assert_eq!(count, 4);
    }
    #[test]
    fn test_count_elem_empty() {
        let input = String::from("
        ...............................
        ...............................
        ...............................
        ");
        let count = count_elem_offset(&input, '#', &3);
        assert_eq!(count, 0);
    }
}