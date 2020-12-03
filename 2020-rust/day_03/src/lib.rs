/// Traverse an input grid in steps defined
/// by column_offset and line_offset.
/// Return an integer representing the 
/// number of times the target char
/// was encountered during traversal.
pub fn count_elem_offset(
    input: &str,
    target: char, 
    column_offset: &usize,
    line_offset: &usize) -> u32 {

    let mut column_pos = 0;
    let mut tree_count = 0;
    
    input
    .lines()
    .map(|l| l.trim())
    .filter(|l| !l.is_empty())
    .step_by(*line_offset)
    .for_each(|l|{
        if column_pos >= l.chars().count() {
            column_pos = column_pos - l.chars().count()
        }
        let c = l.chars().nth(column_pos);
        if c.is_some() && c.unwrap() == target {
            tree_count = tree_count + 1;
        } else {
            println!("Found no trees in line");
        }
        column_pos = column_pos + column_offset;
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
        let count = count_elem_offset(&input, '#', &3, &1);
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
        let count = count_elem_offset(&input, '#', &3, &1);
        assert_eq!(count, 4);
    }
    #[test]
    fn test_count_elem_empty() {
        let input = String::from("
        ...............................
        ...............................
        ...............................
        ");
        let count = count_elem_offset(&input, '#', &3, &1);
        assert_eq!(count, 0);
    }
    #[test]
    fn test_count_elem_line_skip() {
        let input = String::from("
        ...............................
        ##############################
        ...............................
        ");
        let count = count_elem_offset(&input, '#', &3, &2);
        assert_eq!(count, 0);
    }
}