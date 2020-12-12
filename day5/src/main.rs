

const ROWS : usize = 128;
const COLS : usize = 8;

fn bsp_to_linear(bsp: &str, front_token: char, back_token: char, range: usize) -> usize {
    if (1 << bsp.len()) != range {
        panic!("Invalid BSP string length: 2^{} =/= {}", bsp.len(), range);
    }

    let mut start = 0;
    let mut length = range;
    for c in bsp.chars() {
        let half = length / 2;
        if c == front_token {
            length = half;
        }else if c == back_token {
            length = half;
            start += half;
        }else{
            panic!("Invalid character in BSP string: {}", c);
        }
    }

    if length != 1 {
        panic!("Did not converge. start={} length={}", start, length);
    }

    start
}

fn seat_decode(bsp: &str) -> (usize, usize, usize) {

    if bsp.len() != 10 {
        panic!("Invalid BSP string length");
    }

    let row = bsp_to_linear(&bsp[..7], 'F', 'B', ROWS);
    let col = bsp_to_linear(&bsp[7..], 'L', 'R', COLS);

    (row, col, row*8+col)
}

fn main() {
    let passes = std::fs::read_to_string("day5/input.txt").unwrap();

    let mut seat_ids = Vec::new();
    seat_ids.reserve(passes.split('\n').count());

    for pass in passes.split('\n').filter(|l| !l.is_empty()) {
        let (_row, _col, seat_id) = seat_decode(pass);
        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    println!("Maximum seat ID: {:?}", seat_ids.last());

    for (left, right) in seat_ids.iter().zip(seat_ids.iter().skip(1)) {
        if *left+1 != *right {
            println!("My seat is probably {}", *left+1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coords() {
        assert_eq!(seat_decode("FBFBBFFRLR"), (44, 5, 357));
        assert_eq!(seat_decode("BFFFBBFRRR"), (70, 7, 567));
        assert_eq!(seat_decode("FFFBBBFRRR"), (14, 7, 119));
        assert_eq!(seat_decode("BBFFBBFRLL"), (102, 4, 820));
    }
}
