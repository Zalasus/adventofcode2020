
use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, Copy)]
struct Bitmask {
    dont_care_mask: u64,
    force_mask: u64
}

impl Bitmask {
    pub fn from_str(s: &str) -> Self {
        let mut dont_care_mask = 0;
        let mut force_mask = 0;
        for c in s.chars() {
            dont_care_mask <<= 1;
            force_mask <<= 1;
            match c {
                'X' => dont_care_mask |= 1,
                '1' => force_mask |= 1,
                '0' => {},
                 _  => panic!("Invalid character in bitmask: {}", c)
            }
        }
        Self {
            dont_care_mask,
            force_mask
        }
    }

    /// Applies the mask to the provided value (like a type-1 chip).
    pub fn apply_to(&self, u: u64) -> u64 {
        (u & self.dont_care_mask) | self.force_mask
    }

    /// Creates an iterator over all addresses created by masking the provided address (like a
    ///  type-2 chip).
    pub fn address_iter(&self, addr: u64) -> FloatingAddresses {
        FloatingAddresses {
            fixed: (self.force_mask | addr) & !self.dont_care_mask,
            floating: self.dont_care_mask,
            end: 1 << self.dont_care_mask.count_ones(),
            next: 0
        }
    }
}


struct FloatingAddresses {
    fixed: u64,
    floating: u64,
    end: u64,
    next: u64
}

impl Iterator for FloatingAddresses {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.end {
            // distribute the bits in next across the positions that are floating
            let mut floating = self.floating;
            let mut next_bit = 0;
            let mut output = 0;
            while floating != 0 {
                let bit = floating.trailing_zeros();
                if ((1 << next_bit) & self.next) != 0 {
                    output |= 1 << bit;
                }
                next_bit += 1;
                floating &= !(1 << bit);
            }
            self.next += 1;
            Some(output | self.fixed)
        }else{
            None
        }
    }
}


#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(Bitmask),
    Mem(u64, u64)
}

impl Instruction {
    pub fn from_str(s: &str) -> Self {
        if let Some(mask) = s.strip_prefix("mask = ") {
            Instruction::Mask(Bitmask::from_str(mask))
        }else if let Some(mem) = s.strip_prefix("mem[") {
            let mut parts = mem.split("] = ");
            let addr = parts.next().unwrap();
            let value = parts.next().unwrap();
            Instruction::Mem(addr.parse().unwrap(), value.parse().unwrap())
        }else{
            panic!("Invalid instruction string: {}", s);
        }
    }
}


fn run_program_type1(program: &Vec<Instruction>) -> u64 {
    // a 36-bit address space is to big to account for every cell, so we store it sparsely in a map
    let mut memory = HashMap::<u64, u64>::new();
    let mut mask = None;
    for instruction in program {
        match instruction {
            Instruction::Mask(m) => { mask = Some(*m); },
            Instruction::Mem(addr, value) => { memory.insert(*addr, mask.unwrap().apply_to(*value)); }
        }
    }

    // sum up all the cells
    memory.iter().map(|c| c.1).sum::<u64>()
}


fn run_program_type2(program: &Vec<Instruction>) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut mask = None;
    for instruction in program {
        match instruction {
            Instruction::Mask(m) => { mask = Some(*m); },
            Instruction::Mem(addr, value) => {
                for masked_addr in mask.unwrap().address_iter(*addr) {
                    memory.insert(masked_addr, *value);
                }
            }
        }
    }
    memory.iter().map(|c| c.1).sum::<u64>()
}


fn parse_program(s: &str) -> Vec<Instruction> {
    s.split('\n')
     .filter(|l| !l.is_empty())
     .map(|l| Instruction::from_str(l))
     .collect::<Vec<Instruction>>()
}


fn main() {
    let input = std::fs::read_to_string("day14/input.txt").unwrap();
    let program = parse_program(&input);

    let sum1 = run_program_type1(&program);
    println!("Sum of all non-zero cells (type-1 chip): {}", sum1);

    let sum2 = run_program_type2(&program);
    println!("Sum of all non-zero cells (type-2 chip): {}", sum2);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bitmasks() {
        let mask = Bitmask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.apply_to(11), 73);
        assert_eq!(mask.apply_to(101), 101);
        assert_eq!(mask.apply_to(0), 64);
    }

    #[test]
    fn instructions() {
        assert_eq!(Instruction::from_str("mask = X100X00XX1100X111111001001000X00X110"), Instruction::Mask(Bitmask::from_str("X100X00XX1100X111111001001000X00X110")));
        assert_eq!(Instruction::from_str("mem[43083] = 105622"), Instruction::Mem(43083, 105622));
    }

    #[test]
    fn address_iteration() {
        {
            let mask = Bitmask::from_str("000000000000000000000000000000X1001X");
            let mut addr = mask.address_iter(42);
            assert_eq!(addr.next(), Some(26));
            assert_eq!(addr.next(), Some(27));
            assert_eq!(addr.next(), Some(58));
            assert_eq!(addr.next(), Some(59));
            assert_eq!(addr.next(), None);
        }
        {
            let mask = Bitmask::from_str("00000000000000000000000000000000X0XX");
            let mut addr = mask.address_iter(26);
            assert_eq!(addr.next(), Some(16));
            assert_eq!(addr.next(), Some(17));
            assert_eq!(addr.next(), Some(18));
            assert_eq!(addr.next(), Some(19));
            assert_eq!(addr.next(), Some(24));
            assert_eq!(addr.next(), Some(25));
            assert_eq!(addr.next(), Some(26));
            assert_eq!(addr.next(), Some(27));
            assert_eq!(addr.next(), None);
        }
    }

    #[test]
    fn example_program_type2() {
        let program = parse_program("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1");
        let sum = run_program_type2(&program);
        assert_eq!(sum, 208);
    }
}
