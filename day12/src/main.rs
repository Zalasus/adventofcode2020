
extern crate cgmath;

type IVec2 = cgmath::Vector2<i32>;

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'N' => Self::North,
            'E' => Self::East,
            'S' => Self::South,
            'W' => Self::West,
             _  => panic!("Invalid direction character: {}", c)
        }
    }

    pub fn from_degrees(degrees: i32) -> Self {
        match ((degrees % 360) + 360) % 360 {
             90 => Self::North,
              0 => Self::East,
            270 => Self::South,
            180 => Self::West,
            _   => panic!("Invalid degrees: {}", degrees)
        }
    }

    pub fn to_vector(&self) -> IVec2 {
        match *self {
            Self::North => IVec2::new( 0,  1),
            Self::East  => IVec2::new( 1,  0),
            Self::South => IVec2::new( 0, -1),
            Self::West  => IVec2::new(-1,  0)
        }
    }

    pub fn to_degrees(&self) -> i32 {
        match *self {
            Self::North => 90,
            Self::East  => 0,
            Self::South => 270,
            Self::West  => 180
        }
    }

    pub fn rotate(&self, degrees: i32) -> Self {
        Self::from_degrees(self.to_degrees() + degrees)
    }
}


pub fn run_actions(actions: &str) -> IVec2 {
    let mut facing = Direction::East;
    let mut position = IVec2::new(0, 0);
    for action in actions.split('\n').filter(|l| !l.is_empty()) {
        let value : i32 = action[1..].parse().unwrap();
        let action_code = action.chars().next().unwrap();
        match action_code {
            'F' => position += facing.to_vector()*value,
            'N' | 'W' | 'S' | 'E' => position += Direction::from_char(action_code).to_vector()*value,
            'L' => facing = facing.rotate(value),
            'R' => facing = facing.rotate(-value),
            _ => panic!("Invalid action code: {}", action_code)
        }
    }
    position
}


fn rotate_vector(v: IVec2, degrees: i32) -> IVec2 {
    match ((degrees % 360) + 360) % 360 {
         90 => IVec2::new(-v.y,  v.x),
          0 => IVec2::new( v.x,  v.y),
        270 => IVec2::new( v.y, -v.x),
        180 => IVec2::new(-v.x, -v.y),
        _   => panic!("Invalid degrees: {}", degrees)
    }
}


pub fn run_waypoint_actions(actions: &str) -> IVec2{
    let mut waypoint = IVec2::new(10, 1);
    let mut position = IVec2::new(0, 0);
    for action in actions.split('\n').filter(|l| !l.is_empty()) {
        let value : i32 = action[1..].parse().unwrap();
        let action_code = action.chars().next().unwrap();
        match action_code {
            'F' => position += waypoint*value,
            'N' | 'W' | 'S' | 'E' => waypoint += Direction::from_char(action_code).to_vector()*value,
            'L' => waypoint = rotate_vector(waypoint, value),
            'R' => waypoint = rotate_vector(waypoint, -value),
            _ => panic!("Invalid action code: {}", action_code)
        }
    }
    position
}


pub fn l1_norm(v: IVec2) -> i32 {
    v.x.abs() + v.y.abs()
}


fn main() {
    let input = std::fs::read_to_string("day12/input.txt").unwrap();
    let target1 = run_actions(&input);
    println!("After executing the instructions, the ship is at {}/{} (L1 = {})", target1.x, target1.y, l1_norm(target1));

    let target2 = run_waypoint_actions(&input);
    println!("After executing the instructions with a waypoint, the ship is at {}/{} (L1 = {})", target2.x, target2.y, l1_norm(target2));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn simple_actions() {
        let pos = run_actions("F10\nN3\nF7\nR90\nF11");
        assert_eq!(pos, IVec2::new(17, -8));
        assert_eq!(l1_norm(pos), 25);
    }

    #[test]
    fn rotations() {
        assert_eq!(rotate_vector(IVec2::new(1, 0),  90), IVec2::new( 0,  1));
        assert_eq!(rotate_vector(IVec2::new(1, 0), -90), IVec2::new( 0, -1));
        assert_eq!(rotate_vector(IVec2::new(1, 0),   0), IVec2::new( 1,  0));
        assert_eq!(rotate_vector(IVec2::new(1, 0), 180), IVec2::new(-1,  0));
    }

    #[test]
    fn waypoint_actions() {
        let pos = run_waypoint_actions("F10\nN3\nF7\nR90\nF11");
        assert_eq!(pos, IVec2::new(214, -72));
        assert_eq!(l1_norm(pos), 286);
    }

}
