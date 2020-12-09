
fn do_the_thing(data: &[isize], window_length: usize) -> Option<usize> {
    for (index, number) in data.iter().enumerate().skip(window_length) {
        let mut not_a_sum = true;
        for window_number_1 in &data[(index-window_length)..index] {
            for window_number_2 in &data[(index-window_length)..index] {
                if window_number_1 + window_number_2 == *number {
                    not_a_sum = false;
                    break;
                }
            }
        }
        if not_a_sum {
            return Some(index);
        }
    }
    None
}

fn do_the_other_thing(data: &[isize], target: isize) -> Option<(usize,usize)> {
    for start_index in 0..data.len() {
        let mut sum = 0;
        for (count, number) in data.iter().skip(start_index).enumerate() {
            sum += *number;
            if sum == target {
                return Some((start_index, start_index+count+1));
            }else if sum > target {
                break;
            }
        }
    }
    None
}

fn sum_min_max(data: &[isize], range: (usize, usize)) -> isize {
    let data_range = &data[range.0..range.1];
    data_range.iter().min().unwrap() + data_range.iter().max().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("day9_input.txt").unwrap();
    let data = input.split('\n').filter_map(|l| l.trim().parse::<isize>().ok()).collect::<Vec<isize>>();
    if let Some(index) = do_the_thing(&data, 25) {
        println!("Found {} at index {}", data[index], index);
        if let Some(sum_range) = do_the_other_thing(&data, data[index]) {
            println!("Found summing range {}..{}", sum_range.0, sum_range.1);
            println!("Sum of min and max in that range: {}", sum_min_max(&data, sum_range));
        }else{
            println!("No summing range found");
        }
    }else{
        println!("No matching number found");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_ARRAY : &[isize] = &[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576];

    #[test]
    fn thing() {
        assert_eq!(do_the_thing(TEST_ARRAY, 5), Some(14));
    }

    #[test]
    fn other_thing() {
        assert_eq!(do_the_other_thing(TEST_ARRAY, 127), Some((2,6)));
        assert_eq!(sum_min_max(TEST_ARRAY, (2,6)), 62);
    }
}
