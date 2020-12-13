
extern crate num;

use num::integer::{lcm, Integer};

fn parse_input(s: &str) -> (isize, Vec<Option<isize>>) {
    let mut lines = s.split('\n');
    let departure = lines.next().unwrap().trim().parse().unwrap();
    let busses = lines.next().unwrap()
                      .trim()
                      .split(',')
                      .map(|b| b.parse::<isize>().ok())
                      .collect();
    (departure, busses)
}

// returns a tuple of the next depature time and the bus ID
fn find_earliest_bus(earliest_dep: isize, busses: &Vec<Option<isize>>) -> (isize, isize) {
    busses.iter()
          .filter_map(|bus| *bus)
          .map(|bus| ((earliest_dep/bus+1)*bus, bus) )
          .min_by_key(|i| i.0 - earliest_dep).unwrap()
}

fn find_common_timestamp(busses: &Vec<Option<isize>>) -> isize {
    // if b_i is the bus ID at index i, what we want to solve is this system of congruencies:
    //    t ≡ (b_i - i) mod b_i  ∀  i
    // which, apparently, is called the chinese remainder theorem. wish i knew that beforehand...

    // first, we need the least common multiple of the bus IDs
    let id_lcm = busses.iter()
                       .filter_map(|b| *b)
                       .fold(1, |acc, b| lcm(acc, b));

    // now, some weird number theory stuff i copied from wikipedia
    let t = busses.iter()
                      .enumerate()
                      .filter_map(|b| if let Some(id) = b.1 {
                          let i = b.0 as isize;
                          let m = id_lcm/id;
                          let bezout_coeff = isize::extended_gcd(id, &m);
                          assert_eq!(bezout_coeff.gcd, 1);
                          Some((id-i)*bezout_coeff.y*m)
                      }else{
                          None
                      })
                      .sum::<isize>();

    // t + k*lcm(b) is a solution for any integer k. to make sure we get the smallest possible t,
    //  we can simply take t modulo lcm(B)
    ((t % id_lcm) + id_lcm) % id_lcm
}

fn main() {
    let input = std::fs::read_to_string("day13/input.txt").unwrap();
    let (departure, busses) = parse_input(&input);
    let (earliest_bus_time, earliest_bus_id) = find_earliest_bus(departure, &busses);
    let delay = earliest_bus_time - departure;
    println!("The earliest bus after {} is bus ID {}, which arrives at {}, causing {} min of delay", departure, earliest_bus_id, earliest_bus_time, delay);
    println!("Bus ID times delay: {}", earliest_bus_id*delay);

    let ts = find_common_timestamp(&busses);
    println!("The earliest timestamp at which the described pattern occurs is at {}", ts);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn earliest_bus() {
        let (departure, busses) = parse_input("939\n7,13,x,x,59,x,31,19");
        let earliest_bus = find_earliest_bus(departure, &busses);
        assert_eq!(earliest_bus, (944, 59));
    }

    #[test]
    fn common_timestamp() {
        {
            let (_, busses) = parse_input("939\n7,13,x,x,59,x,31,19");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 1068781);
        }
        {
            let (_, busses) = parse_input("939\n67,7,59,61");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 754018);
        }
        {
            let (_, busses) = parse_input("939\n67,x,7,59,61");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 779210);
        }
        {
            let (_, busses) = parse_input("939\n67,7,x,59,61");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 1261476);
        }
        {
            let (_, busses) = parse_input("939\n17,x,13,19");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 3417);
        }
        {
            let (_, busses) = parse_input("939\n1789,37,47,1889");
            let common_ts = find_common_timestamp(&busses);
            assert_eq!(common_ts, 1202161486);
        }
    }
}
