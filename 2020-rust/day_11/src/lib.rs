use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Seat {
    pub x: usize,
    pub y: usize,
    pub sittable: bool,
    pub occupied: bool
}
#[derive(Debug, PartialEq, Eq)]
pub struct WaitingRoom {
    total_sittable_seats: usize,
    state: Vec<Vec<Seat>>,
    occupied: Vec<Seat>,
    unoccupied: Vec<Seat>
}
impl WaitingRoom {
    pub fn new(input: &str) -> WaitingRoom {
        let mut grid: Vec<Vec<Seat>> = Vec::new();
        let mut unoccupied: Vec<Seat> = Vec::new();
        let mut occupied: Vec<Seat> = Vec::new();
        let lines: Vec<&str> = input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
        let mut sittable_seat_counter = 0;
        let mut line_counter = 0;
        for line in lines {
            let mut seat_counter = 0;
            let mut this_row: Vec<Seat> = Vec::new();
            let seats = line.chars();
            for seat in seats {
                let mut seat_object = Seat {
                    y: line_counter,
                    x: seat_counter,
                    sittable: seat != '.',
                    occupied: seat == '#'
                };
                if seat_object.sittable {
                    unoccupied.push(seat_object.clone());
                    sittable_seat_counter = sittable_seat_counter + 1;
                }
                if seat_object.occupied {
                    occupied.push(seat_object.clone());
                }
                this_row.push(seat_object);
                seat_counter = seat_counter + 1;
            };
            grid.push(this_row);
            line_counter = line_counter + 1;
        }
        return WaitingRoom {
            unoccupied: unoccupied,
            total_sittable_seats: sittable_seat_counter,
            state: grid,
            occupied: occupied
        }
    }
    pub fn print_state(&self) {
        println!("{}", WaitingRoom::get_state(&self));
    }
    pub fn get_state(&self) -> String {
        let mut state_string = String::new();
        for row in &self.state {
            let mut this_row = String::new();
            for col in row {
                if !col.sittable {
                    this_row.push('.');
                } else if col.occupied {
                    this_row.push('#');
                } else {
                    this_row.push('L');
                }
            }
            this_row.push('\n');
            state_string.push_str(&this_row);
        }
        return state_string;
    }
    pub fn is_full(&self) -> bool {
        return &self.occupied.len() == &self.total_sittable_seats;
    }
    pub fn get_empty(&self) -> &[Seat] {
        return &[Seat{
            x: 0,
            y: 0,
            occupied: false,
            sittable: false
        }]
    }
    pub fn get_adjacent_occupied_count_2(&self, seat: &Seat) -> usize {
        let mut count = 0;
        &[1, 2, 3, 4, 5, 6, 7, 8].iter().for_each(|d|{
            let candidate: Option<Seat> = self.get_first_seat_in_direction(seat, *d);
            if candidate.is_some() {
                //println!("First seat in direction: {}: {:#?}", d, candidate.unwrap());
            }
            if candidate.is_some() && candidate.unwrap().occupied {
                count = count + 1;
            }
        });
        return count;
    }
    pub fn get_first_seat_in_direction(&self, seat: &Seat, direction: usize) -> Option<Seat>{
        //
        //  1   2   3
        //  8   x   4
        //  7   6   5
        //
        if direction == 1 {
            // Find first top left diag
            let mut x = seat.x;
            let mut y = seat.y;
            while x > 0 && y > 0 {
                x = x - 1;
                y = y - 1;
                let candidate = &self.state[y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 2 {
            //Find first straight up
            let mut y = seat.y;
            while y > 0 {
                y = y - 1;
                let candidate = &self.state[y][seat.x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 3 {
            //Find first top right diag
            let mut x = seat.x;
            let mut y = seat.y;
            while x < &self.state[0].len() -1 && y > 0 {
                x = x + 1;
                y = y - 1;
                let candidate = &self.state[y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 4 {
            //Find first directly right
            let mut x = seat.x;
            while x < &self.state[0].len() -1 {
                x = x + 1;
                let candidate = &self.state[seat.y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 5 {
            //Find first bottom right diag
            let mut x = seat.x;
            let mut y = seat.y;
            while x < &self.state[0].len() - 1 && y <  &self.state.len() - 1 {
                x = x + 1;
                y = y + 1;
                let candidate = &self.state[y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 6 {
            //Find first straight down
            let mut y = seat.y;
            while y < self.state.len() - 1 {
                y = y + 1;
                let candidate = &self.state[y][seat.x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 7 {
            // Find first bottom left diag
            let mut x = seat.x;
            let mut y = seat.y;
            while x > 0 && y < self.state.len() - 1 {
                x = x - 1;
                y = y + 1;
                let candidate = &self.state[y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        if direction == 8 {
            //Find first directly left
            let mut x = seat.x;
            while x > 0 {
                x = x - 1;
                let candidate = &self.state[seat.y][x];
                if candidate.sittable {
                    return Some(*candidate);
                };
            };
        };
        return None;
    }
    pub fn get_adjacent_occupied_count(&self, target: &Seat) -> usize {
        let mut count = 0;
        //println!("Target: {:#?}", target);
        //Get adjacent items
        if target.x > 0 && &self.state[target.y][target.x - 1].occupied == &true { 
            //println!("Occupied: {:#?}", &self.state[target.y][target.x - 1]);
            count = count + 1;
        };
        if target.x < self.state[0].len() -1 && &self.state[target.y][target.x + 1].occupied == &true {
            //println!("Occupied: {:#?}",  &self.state[target.y][target.x + 1]);
            count = count + 1;
        }
        if target.y > 0 && &self.state[target.y -1][target.x].occupied == &true { 
            //println!("Occupied: {:#?}", &self.state[target.y -1][target.x]);
            count = count + 1;
        };
        if target.y < self.state.len() -1 && &self.state[target.y + 1][target.x].occupied == &true {
            //println!("Occupied: {:#?}",&self.state[target.y + 1][target.x]);
            count = count + 1;
        }
        if target.y > 0 && target.x > 0 && &self.state[target.y -1][target.x -1].occupied == &true { 
            //println!("Occupied: {:#?}", &self.state[target.y -1][target.x -1]);
            count = count + 1;
        };
        if target.y < self.state.len() -1 && target.x < self.state[0].len() -1 && &self.state[target.y + 1][target.x + 1].occupied == &true {
            //println!("Occupied: {:#?}", &self.state[target.y + 1][target.x + 1]);
            count = count + 1;
        }
        if target.x > 0 && target.y < self.state.len() -1 && &self.state[target.y + 1][target.x - 1].occupied == &true { 
            //println!("Occupied: {:#?}", &self.state[target.y + 1][target.x - 1]);
            count = count + 1;
        };
        if target.x < self.state[0].len() - 1 && target.y > 0 && &self.state[target.y - 1][target.x + 1].occupied == &true {
            //println!("Occupied: {:#?}", &self.state[target.y - 1][target.x + 1]); 
            count = count + 1;
        };
        return count;
    }

    pub fn count_occupied(&self) -> usize{
        let mut count = 0;
        for row in &self.state {
            for col in row {
                if col.occupied {
                    count = count + 1;
                }
            }
        }
        return count;
    }

    pub fn update(&mut self) {
        let mut occupied_staged: Vec<Seat> = Vec::new();
        let mut unoccupied_staged: Vec<Seat> = Vec::new();
        let mut state_copy = self.state.clone();
        //For each empty seat, check if there are adjacent occupied seats
        &self.unoccupied.iter().for_each(|s|{
            //If not, it becomes occupied
            if WaitingRoom::get_adjacent_occupied_count(&self, s) == 0 {
                occupied_staged.push(s.clone());
            }
        });
        //For each occupied seat, check if there are adjacent occupied seats
        &self.occupied.iter().for_each(|s|{
            //If not, it becomes occupied
            if WaitingRoom::get_adjacent_occupied_count(&self, s) >= 4 {
                unoccupied_staged.push(s.clone());
            }
        });
        //Update grid
        occupied_staged.iter().for_each(|s|{
            state_copy[s.y][s.x].occupied = true;
        });
        //Update grid
        unoccupied_staged.iter().for_each(|s|{
            state_copy[s.y][s.x].occupied = false;
        });
        self.state = state_copy;
        self.occupied = occupied_staged;
        self.unoccupied = unoccupied_staged;
    }

    pub fn update_2(&mut self) {
        let mut occupied_staged: Vec<Seat> = Vec::new();
        let mut unoccupied_staged: Vec<Seat> = Vec::new();
        let mut state_copy = self.state.clone();
        //For each empty seat, check if there are adjacent occupied seats
        &self.unoccupied.iter().for_each(|s|{
            //If not, it becomes occupied
            if WaitingRoom::get_adjacent_occupied_count_2(&self, s) == 0 {
                occupied_staged.push(s.clone());
            }
        });
        //For each occupied seat, check if there are adjacent occupied seats
        &self.occupied.iter().for_each(|s|{
            //If not, it becomes occupied
            if WaitingRoom::get_adjacent_occupied_count_2(&self, s) >= 5 {
                unoccupied_staged.push(s.clone());
            }
        });
        //Update grid
        occupied_staged.iter().for_each(|s|{
            state_copy[s.y][s.x].occupied = true;
        });
        //Update grid
        unoccupied_staged.iter().for_each(|s|{
            state_copy[s.y][s.x].occupied = false;
        });
        self.state = state_copy;
        self.occupied = occupied_staged;
        self.unoccupied = unoccupied_staged;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_adjacent_occupied_count_2(){
        let input = "
        .......#.
        ...#.....
        .#.......
        .........
        ..#L....#
        ....#....
        .........
        #........
        ...#.....";
        let room = WaitingRoom::new(&input);
        let target_seat = Seat {
            x: 3,
            y: 4,
            sittable: true,
            occupied: false
        };
        assert_eq!(room.get_adjacent_occupied_count_2(&target_seat), 8);
    }
    #[test]
    fn test_get_adjacent_occupied_true(){
        let input = "
        L#L#.LL.LL
        L###LLLL.LL
        L.#.L..L..
        LL#L.LL.LL
        L.#L.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let room = WaitingRoom::new(&input);
        let target_seat = Seat {
            x: 2,
            y: 0,
            sittable: true,
            occupied: false
        };
        assert_eq!(room.get_adjacent_occupied_count(&target_seat), 5);
    }
    #[test]
    fn test_get_adjacent_occupied_count_true_2(){
        let input = "
        #.LL.L#.##
        #LLLLLL.L#
        L.L.L..L..
        #LLL.LL.L#
        #.LL.LL.LL
        #.LLLL#.##
        ..L.L.....
        #LLLLLLLL#
        #.LLLLLL.L
        #.#LLLL.##";
        let room = WaitingRoom::new(&input);
        let target_seat = Seat {
            x: 5,
            y: 0,
            sittable: true,
            occupied: false
        };
        assert_eq!(room.get_adjacent_occupied_count(&target_seat), 1);
    }
    #[test]
    fn test_get_adjacent_occupied_count_false(){
        let input = "
        L.LL.LL.LL
        LL#LLLL.LL
        L.#.L..L..
        LL#L.LL.LL
        L.#L.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let room = WaitingRoom::new(&input);
        let target_seat = Seat {
            x: 5,
            y: 0,
            sittable: true,
            occupied: false
        };
        assert_eq!(room.get_adjacent_occupied_count(&target_seat), 0);
    }
    #[test]
    fn test_get_adjacent_occupied_count_out_of_bounds(){
        let input = "
        L.LL.LL.LL
        LL#LLLL.LL
        L.#.L..L..
        LL#L.LL.LL
        L.#L.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let room = WaitingRoom::new(&input);
        let target_seat = Seat {
            x: 0,
            y: 8,
            sittable: true,
            occupied: false
        };
        assert_eq!(room.get_adjacent_occupied_count(&target_seat), 0);
    }

    /*#[test]
    fn test_update(){
        let input = "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let mut room = WaitingRoom::new(&input);
        let mut prev_state = String::new();
        loop {
            prev_state = room.get_state();
            room.update();
            room.print_state();
            if room.get_state() == prev_state {
                println!("{:#?}", room);
                println!("{}", room.count_occupied());
                break
            }
            prev_state = room.get_state();
        }

    }*/

    #[test]
    fn test_update_2(){
        let input = "
        L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";
        let mut room = WaitingRoom::new(&input);
        let mut prev_state = String::new();
        loop {
            prev_state = room.get_state();
            room.update_2();
            room.print_state();
            if room.get_state() == prev_state {
                println!("{:#?}", room);
                println!("{}", room.count_occupied());
                break
            }
            prev_state = room.get_state();
        }

    }
}
