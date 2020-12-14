use std::f32::consts::PI;

/// 2D rotation of a point
pub fn rotation_2d(x: isize, y: isize, angle: isize) -> (isize, isize) {
    let radians = angle as f32 * PI / 180_f32;

    let nx = (x as f32 * radians.cos() - y as f32 * radians.sin()).round();
    let ny = (y as f32 * radians.cos() + x as f32 * radians.sin()).round();

    (nx as isize, ny as isize)
}
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
    pub fn move_by_waypoint(&mut self, waypoint: &Waypoint, value: &isize) {
        self.east += waypoint.x * value;
        self.north += waypoint.y * value;
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct Waypoint {
    x: isize,
    y: isize,
}

impl Waypoint {
    pub fn north(&mut self, step: &isize) {
        self.y += step;
    }

    pub fn south(&mut self, step: &isize) {
        self.y -= step;
    }

    pub fn east(&mut self, step: &isize) {
        self.x += step;
    }

    pub fn west(&mut self, step: &isize) {
        self.x -= step;
    }

    pub fn rotate(&mut self, angle: &isize) {
        let (nx, ny) = rotation_2d(self.x, self.y, *angle);

        self.x = nx;
        self.y = ny;
    }

    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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
