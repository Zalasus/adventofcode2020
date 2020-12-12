

fn find_differences(mut data: Vec<isize>) -> (usize, usize) {
    data.push(0); // add the outlet so we count that difference, too
    data.sort();
    let result = data.iter()
        .zip(data.iter().skip(1))
        .fold((0,0), |acc, x| {
            let diff = x.1 - x.0;
            if diff == 1 {
                (acc.0+1, acc.1)
            }else if diff == 2 {
                acc
            }else if diff == 3 {
                (acc.0, acc.1+1)
            }else {
                panic!("Unexpected difference: {}-{}={}", x.1, x.0, diff);
            }
        });

    // the difference between the last adapter and the device is always 3, so add one to that count
    (result.0, result.1+1)
}

fn find_permutations(mut data: Vec<isize>) -> usize {
    // we cheat a little by adding two unreachable chargers and the outlet. this allows us use a
    //  sliding window without clamping the range of the look-back
    data.push(-101);
    data.push(-100);
    data.push(0);
    data.sort();

    let mut paths : Vec<usize> = Vec::new();
    paths.resize(data.len(), 0);
    paths[2] = 1;

    for (index, window) in data.windows(4).enumerate() {
        let current = window[3];
        for lookback in 0..3 {
            if current - window[lookback] < 4 {
                // current and previous overlap.
                paths[index+3] += paths[index + lookback];
            }
        }
    }

    *paths.last().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("day10/input.txt").unwrap();
    let data = input.split('\n').filter_map(|l| l.trim().parse::<isize>().ok()).collect::<Vec<isize>>();

    let differences = find_differences(data.clone());
    println!("1-jolt differences={} 3-jolt-differences={} multiplied={}", differences.0, differences.1, differences.0*differences.1);

    let permutations = find_permutations(data.clone());
    println!("number of possible charger permutations: {}", permutations);
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_ARRAY   : &[isize] = &[16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    const TEST_ARRAY_2 : &[isize] = &[28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];

    #[test]
    fn differences() {
        assert_eq!(find_differences(TEST_ARRAY.to_vec()), (7,5));
        assert_eq!(find_differences(TEST_ARRAY_2.to_vec()), (22,10));
    }

    #[test]
    fn permutations() {
        assert_eq!(find_permutations(TEST_ARRAY.to_vec()), 8);
        assert_eq!(find_permutations(TEST_ARRAY_2.to_vec()), 19208);
    }
}
