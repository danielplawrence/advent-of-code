#[derive(Copy, Clone, Debug)]
pub struct Ship {
    pub north: isize,
    pub east: isize,
    pub facing: char,
    pub facing_degrees: isize
}
impl Ship {
    pub fn new() -> Ship {
        return Ship {
            north: 0,
            east: 0,
            facing: 'E',
            facing_degrees: 90
        }
    }
    pub fn move_ship(&mut self, action: char, distance: isize){
        let compass_directions = &['E', 'W', 'N', 'S'];
        if compass_directions.contains(&action){
            self.move_ship_direction(action, distance);
        }
        if action == 'F' {
            let facing = self.facing;
            self.move_ship_direction(facing, distance);
        }
        let turns = &['L', 'R'];
        if turns.contains(&action){
            self.turn_ship_direction(action, distance);
        }
    }
    fn turn_ship_direction(&mut self, direction: char, distance: isize){
        if direction == 'R' {
            if self.facing_degrees + distance > 360 {
                self.facing_degrees = (self.facing_degrees + distance) - 360;
            } else {
                self.facing_degrees = self.facing_degrees + distance;
            }
        }
        if direction == 'L' {
            if self.facing_degrees - distance < 0 {
                self.facing_degrees = self.facing_degrees - distance + 360;
            } else {
                self.facing_degrees = self.facing_degrees - distance;
            }
        }
        if self.facing_degrees == 360 {
            self.facing_degrees = 0;
        }
        if self.facing_degrees == 0 { 
            self.facing = 'N';
        }
        if self.facing_degrees == 90 {
            self.facing = 'E';
        }
        if self.facing_degrees == 180 {
            self.facing = 'S';
        }
        if self.facing_degrees == 270 {
            self.facing = 'W';
        }
    
    }
    fn move_ship_direction(&mut self, direction: char, distance: isize){
        println!("{} {}", direction, distance);
        if direction == 'E' {
            self.east = self.east + distance;
        }
        if direction == 'W' {
            self.east = self.east - distance;
        }
        if direction == 'N' {
            self.north = self.north + distance;
        }
        if direction == 'S' {
            self.north = self.north - distance;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_ship_forward(){
        let mut ship = Ship::new();
        ship.move_ship('F', 10);
        assert_eq!(ship.north, 0);
        assert_eq!(ship.east, 10);
    }
    #[test]
    fn test_move_ship_direction(){
        let mut ship = Ship::new();
        ship.move_ship('E', 10);
        assert_eq!(ship.north, 0);
        assert_eq!(ship.east, 10);
    }
    #[test]
    fn test_move_ship_turn(){
        let mut ship = Ship::new();
        ship.move_ship('L', 180);
        assert_eq!(ship.facing_degrees, 270);
        assert_eq!(ship.facing, 'W');
        ship.move_ship('R', 90);
        assert_eq!(ship.facing_degrees, 0);
        assert_eq!(ship.facing, 'N');
        ship.move_ship('R', 90);
        assert_eq!(ship.facing_degrees, 90);
        assert_eq!(ship.facing, 'E');
        ship.move_ship('R', 90);
        assert_eq!(ship.facing_degrees, 180);
        assert_eq!(ship.facing, 'S');
    }
}
