
use std::collections::HashMap;


struct ElfGame {
    start_sequence: Vec<usize>,
    turn_index: usize,
    last_number: Option<usize>,
    turn_map: HashMap<usize, usize> // key=number, value=turn where it was last spoken
}

impl ElfGame {
    pub fn new(start_sequence: &[usize]) -> Self {
        Self {
            start_sequence: start_sequence.to_vec(),
            turn_index: 0,
            last_number: None,
            turn_map: HashMap::new()
        }
    }

    pub fn reset(&mut self) {
        self.turn_index = 0;
        self.last_number = None;
        self.turn_map.clear();
    }
}

impl Iterator for ElfGame {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {

        let next_number = if self.turn_index < self.start_sequence.len() {
            self.start_sequence[self.turn_index]
        }else{
            if let Some(last_turn) = self.turn_map.get(&self.last_number.unwrap()) {
                self.turn_index - last_turn - 1
            }else{
                0
            }
        };

        if let Some(last_number) = self.last_number {
            self.turn_map.insert(last_number, self.turn_index-1);
        }

        self.last_number = Some(next_number);
        self.turn_index += 1;

        Some(next_number)
    }
}


fn main() {
    let mut game = ElfGame::new(&[7,14,0,17,11,1,2]);
    println!("The 2020th number spoken is {}", game.nth(2020 - 1).unwrap());

    game.reset();
    println!("The 30000000th number spoken is {}", game.nth(30000000 - 1).unwrap());
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn steps() {
        let mut game = ElfGame::new(&[0,3,6]);
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(6));
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(3));
        assert_eq!(game.next(), Some(1));
        assert_eq!(game.next(), Some(0));
        assert_eq!(game.next(), Some(4));
        assert_eq!(game.next(), Some(0));
    }

    #[test]
    fn various_2020th() {
        // note that our indices start at 0, thus, we need the 2019th number
        let n = 2020 - 1;
        assert_eq!(ElfGame::new(&[0,3,6]).nth(n), Some(436));
        assert_eq!(ElfGame::new(&[1,3,2]).nth(n), Some(1));
        assert_eq!(ElfGame::new(&[2,1,3]).nth(n), Some(10));
        assert_eq!(ElfGame::new(&[1,2,3]).nth(n), Some(27));
        assert_eq!(ElfGame::new(&[2,3,1]).nth(n), Some(78));
        assert_eq!(ElfGame::new(&[3,2,1]).nth(n), Some(438));
        assert_eq!(ElfGame::new(&[3,1,2]).nth(n), Some(1836));
    }

    #[test]
    fn various_30000000th() {
        let n = 30000000 - 1;
        assert_eq!(ElfGame::new(&[0,3,6]).nth(n), Some(175594));
        assert_eq!(ElfGame::new(&[1,3,2]).nth(n), Some(2578));
        assert_eq!(ElfGame::new(&[2,1,3]).nth(n), Some(3544142));
        assert_eq!(ElfGame::new(&[1,2,3]).nth(n), Some(261214));
        assert_eq!(ElfGame::new(&[2,3,1]).nth(n), Some(6895259));
        assert_eq!(ElfGame::new(&[3,2,1]).nth(n), Some(18));
        assert_eq!(ElfGame::new(&[3,1,2]).nth(n), Some(362));
    }
}
