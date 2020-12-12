
#[derive(PartialEq, Clone, Copy, Debug)]
enum Cell {
    Floor,
    Seat,
    OccupiedSeat
}


type Coord = (isize, isize);


struct Automaton {
    width: usize,
    height: usize,
    current_state: Vec<Cell>,
    next_state: Vec<Cell>
}


trait Ruleset {
    fn step_cell(&mut self, automaton: &Automaton, cell: Cell, p: Coord) -> Cell;
}


impl Automaton {
    pub fn new(s: &str) -> Self {
        let mut height = 0;
        let mut width = None;
        let mut map = Vec::new();

        for line in s.split('\n').map(|l| l.trim()).filter(|l| !l.is_empty()) {

            let this_width = line.len();
            if let Some(some_width) = width {
                if some_width != this_width {
                    panic!("Width mismatch");
                }
            }else{
                width = Some(this_width);
            }

            height += 1;

            map.extend(line.chars().map(|c| {
                match c {
                    '.' => Cell::Floor,
                    'L' => Cell::Seat,
                    '#' => Cell::OccupiedSeat,
                     _  => panic!("Unrecognized character")
                }
            }));
        }

        Self{
            width: width.unwrap(),
            height,
            current_state: map.clone(),
            next_state: map
        }
    }

    pub fn get(&self, p: Coord) -> Option<Cell> {
        if p.0 < 0 || p.0 >= (self.width as isize) || p.1 < 0 || p.1 >= (self.height as isize) {
            None
        }else {
            Some(self.current_state[(p.0 as usize) + (p.1 as usize)*self.width])
        }
    }

    // returns true if the state changed, false if not
    pub fn step<R: Ruleset>(&mut self, ruleset: &mut R) -> bool {
        let mut changed = false;

        for y in 0..self.height {
            for x in 0..self.width {
                let coord = (x as isize, y as isize);
                let index = y*self.width + x;
                let cell = self.current_state[index];
                let next_cell = ruleset.step_cell(self, cell, coord);
                if next_cell != cell {
                    self.next_state[index] = next_cell;
                    changed = true;
                }
            }
        }

        if changed {
            self.current_state.clone_from_slice(&self.next_state);
        }

        changed
    }

    pub fn count_total_occupied_seats(&self) -> usize {
        self.current_state.iter().filter(|c| **c == Cell::OccupiedSeat).count()
    }
}

impl PartialEq for Automaton {
    fn eq(&self, other: &Automaton) -> bool {
           self.width == other.width
        && self.height == other.height
        && self.current_state == other.current_state
    }
}

impl std::fmt::Debug for Automaton {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.current_state[y*self.width + x] {
                    Cell::Seat => formatter.write_str("L")?,
                    Cell::OccupiedSeat => formatter.write_str("#")?,
                    Cell::Floor => formatter.write_str(".")?
                }
            }
            formatter.write_str("\n")?
        }
        Ok(())
    }
}


struct AdjacencyRuleset {
}

impl AdjacencyRuleset {
    pub fn new() -> Self {
        Self{}
    }

    fn count_adjacent_occupied_seats(automaton: &Automaton, p: Coord) -> usize {
        let mut result = 0;
        let offsets = [(1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1)];
        for offset in &offsets {
            let op = (p.0 + offset.0, p.1 + offset.1);
            if let Some(Cell::OccupiedSeat) = automaton.get(op) {
                result += 1;
            }
        }
        result
    }
}

impl Ruleset for AdjacencyRuleset {
    fn step_cell(&mut self, automaton: &Automaton, cell: Cell, p: Coord) -> Cell {
        match cell {
            Cell::Seat => {
                if AdjacencyRuleset::count_adjacent_occupied_seats(automaton, p) == 0 {
                    Cell::OccupiedSeat
                }else{
                    Cell::Seat
                }
            },
            Cell::OccupiedSeat => {
                if AdjacencyRuleset::count_adjacent_occupied_seats(automaton, p) >= 4 {
                    Cell::Seat
                }else{
                    Cell::OccupiedSeat
                }
            },
            Cell::Floor => Cell::Floor
        }
    }
}


struct SightlineRuleset {
}

impl SightlineRuleset {
    pub fn new() -> Self {
        Self{}
    }

    fn count_visible_occupied_seats(automaton: &Automaton, start: Coord) -> usize {
        let mut result = 0;
        let slopes = [(1,0), (1,-1), (0,-1), (-1,-1), (-1,0), (-1,1), (0,1), (1,1)];
        for slope in &slopes {
            if let Some(Cell::OccupiedSeat) = Self::walk(automaton, start, *slope) {
                result += 1;
            }
        }
        result
    }

    /// Walks along the specified line and returns the first non-floor cell it encounters, or None
    ///  if it walks out of the seating area.
    fn walk(automaton: &Automaton, start: Coord, slope: Coord) -> Option<Cell> {
        let mut p = start;
        loop {
            p.0 += slope.0;
            p.1 += slope.1;
            match automaton.get(p) {
                Some(Cell::Floor) => {},
                Some(not_floor) => { return Some(not_floor); },
                None => { return None; }
            }
        }
    }
}

impl Ruleset for SightlineRuleset {
    fn step_cell(&mut self, automaton: &Automaton, cell: Cell, p: Coord) -> Cell {
        match cell {
            Cell::Seat => {
                if SightlineRuleset::count_visible_occupied_seats(automaton, p) == 0 {
                    Cell::OccupiedSeat
                }else{
                    Cell::Seat
                }
            },
            Cell::OccupiedSeat => {
                if SightlineRuleset::count_visible_occupied_seats(automaton, p) >= 5 {
                    Cell::Seat
                }else{
                    Cell::OccupiedSeat
                }
            },
            Cell::Floor => Cell::Floor
        }
    }
}


fn main() {
    let input = std::fs::read_to_string("day11/input.txt").unwrap();

    {
        let mut automaton = Automaton::new(&input);
        let mut ruleset = AdjacencyRuleset::new();
        while automaton.step(&mut ruleset) {
        }
        println!("Final state with adjacency ruleset has {} occupied seats", automaton.count_total_occupied_seats());
    }

    {
        let mut automaton = Automaton::new(&input);
        let mut ruleset = SightlineRuleset::new();
        while automaton.step(&mut ruleset) {
        }
        println!("Final state with sightlines ruleset has {} occupied seats", automaton.count_total_occupied_seats());
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adjacency() {
        let automaton = Automaton::new(".###
                                        #L.#
                                        .L.#");
        assert_eq!(automaton.get((0,0)), Some(Cell::Floor));
        assert_eq!(automaton.get((1,0)), Some(Cell::OccupiedSeat));
        assert_eq!(automaton.get((2,0)), Some(Cell::OccupiedSeat));
        assert_eq!(automaton.get((0,1)), Some(Cell::OccupiedSeat));
        assert_eq!(automaton.get((1,1)), Some(Cell::Seat));
        assert_eq!(automaton.get((3,1)), Some(Cell::OccupiedSeat));
        assert_eq!(automaton.get((-1,1)), None);
        assert_eq!(automaton.get((6,9)), None);
        assert_eq!(AdjacencyRuleset::count_adjacent_occupied_seats(&automaton, (0,0)), 2);
        assert_eq!(AdjacencyRuleset::count_adjacent_occupied_seats(&automaton, (1,1)), 3);
        assert_eq!(AdjacencyRuleset::count_adjacent_occupied_seats(&automaton, (2,1)), 5);
        assert_eq!(AdjacencyRuleset::count_adjacent_occupied_seats(&automaton, (3,1)), 3);
        assert_eq!(AdjacencyRuleset::count_adjacent_occupied_seats(&automaton, (3,2)), 1);
    }

    #[test]
    fn simulate_adjacency() {
        let mut ruleset = AdjacencyRuleset::new();
        let mut automaton = Automaton::new("L.LL.LL.LL
                                            LLLLLLL.LL
                                            L.L.L..L..
                                            LLLL.LL.LL
                                            L.LL.LL.LL
                                            L.LLLLL.LL
                                            ..L.L.....
                                            LLLLLLLLLL
                                            L.LLLLLL.L
                                            L.LLLLL.LL");
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);

        let current_state = Automaton::new("#.##.L#.##
                                            #L###LL.L#
                                            L.#.#..#..
                                            #L##.##.L#
                                            #.##.LL.LL
                                            #.###L#.##
                                            ..#.#.....
                                            #L######L#
                                            #.LL###L.L
                                            #.#L###.##");
        assert_eq!(automaton, current_state);

        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), false);

        let final_state = Automaton::new("#.#L.L#.##
                                          #LLL#LL.L#
                                          L.#.L..#..
                                          #L##.##.L#
                                          #.#L.LL.LL
                                          #.#L#L#.##
                                          ..L.L.....
                                          #L#L##L#L#
                                          #.LLLLLL.L
                                          #.#L#L#.##");
        assert_eq!(automaton, final_state);

        assert_eq!(automaton.count_total_occupied_seats(), 37);
    }

    #[test]
    fn simulate_sightlines() {
        let mut ruleset = SightlineRuleset::new();
        let mut automaton = Automaton::new("L.LL.LL.LL
                                            LLLLLLL.LL
                                            L.L.L..L..
                                            LLLL.LL.LL
                                            L.LL.LL.LL
                                            L.LLLLL.LL
                                            ..L.L.....
                                            LLLLLLLLLL
                                            L.LLLLLL.L
                                            L.LLLLL.LL");
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);

        let current_state = Automaton::new("#.L#.##.L#
                                            #L#####.LL
                                            L.#.#..#..
                                            ##L#.##.##
                                            #.##.#L.##
                                            #.#####.#L
                                            ..#.#.....
                                            LLL####LL#
                                            #.L#####.L
                                            #.L####.L#");
        assert_eq!(automaton, current_state);

        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), true);
        assert_eq!(automaton.step(&mut ruleset), false);

        let final_state = Automaton::new("#.L#.L#.L#
                                          #LLLLLL.LL
                                          L.L.L..#..
                                          ##L#.#L.L#
                                          L.L#.LL.L#
                                          #.LLLL#.LL
                                          ..#.L.....
                                          LLL###LLL#
                                          #.LLLLL#.L
                                          #.L#LL#.L#");
        assert_eq!(automaton, final_state);

        assert_eq!(automaton.count_total_occupied_seats(), 26);
    }
}
