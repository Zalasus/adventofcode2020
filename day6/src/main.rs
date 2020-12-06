

// as an extra challenge, this was implemented without any loops. yaay, coding is fun! :3


trait Count {
    fn new() -> Self;
    fn count(&mut self, s: &str) -> usize;
}


// counts unique characters that appear in any line.
//  note that this breaks if a character appears more than once per line
struct AnyCounter {
    buffer: Vec<char>
}

impl Count for AnyCounter {
    fn new() -> Self {
        Self{
            buffer: Vec::new()
        }
    }

    fn count(&mut self, s: &str) -> usize {
        self.buffer.clear();
        self.buffer.extend(s.chars().filter(|c| *c != '\n'));
        self.buffer.sort();
        self.buffer.dedup();
        self.buffer.len()
    }
}


// counts unique characters that appear in all lines.
//  note that this breaks if a character appears more than once per line
struct AllCounter {
    buffer: Vec<char>
}

impl Count for AllCounter {
    fn new() -> Self {
        Self{
            buffer: Vec::new()
        }
    }

    fn count(&mut self, s: &str) -> usize {
        let line_count = s.trim().split('\n').count();

        self.buffer.clear();
        self.buffer.extend(s.chars().filter(|c| *c != '\n'));

        // edge case: only one line means all characters are unique and appear on all lines
        if line_count == 1 {
            return self.buffer.len();
        }

        self.buffer.sort();

        self.buffer.iter()
                   .zip(self.buffer.iter().skip(1))
                   .fold((0,0), |acc, x| {
                        // acc is a tuple, first is the number of consecutive equal elements
                        // encountered, second is the number of times that count reached the
                        // number of lines
                        if x.0 == x.1 {
                            if acc.0+2 == line_count {
                                (acc.0+1, acc.1+1)
                            }else{
                                (acc.0+1, acc.1)
                            }
                        }else{
                            (0, acc.1)
                        }
                   }).1
    }
}


fn sum_group_answers<C: Count>(group_answers: &str) -> usize {
    let mut counter = C::new();
    group_answers.split("\n\n")
                 .filter(|l| !l.is_empty())
                 .map(|l| counter.count(l))
                 .sum()
}


fn main() {
    let group_answers = std::fs::read_to_string("day6_input.txt").unwrap();

    let any_sum = sum_group_answers::<AnyCounter>(&group_answers);
    println!("The sum of the count of answers thay appear *anywhere* is: {}", any_sum);

    let all_sum = sum_group_answers::<AllCounter>(&group_answers);
    println!("The sum of the count of answers thay appear *everywhere* is: {}", all_sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_any() {
        let mut counter = AnyCounter::new();
        assert_eq!(counter.count("abc"), 3);
        assert_eq!(counter.count("a\nb\nc"), 3);
        assert_eq!(counter.count("ab\nac"), 3);
        assert_eq!(counter.count("a\na\na\na"), 1);
        assert_eq!(counter.count("b"), 1);
    }

    #[test]
    fn count_all() {
        let mut counter = AllCounter::new();
        assert_eq!(counter.count("abc"), 3);
        assert_eq!(counter.count("a\nb\nc"), 0);
        assert_eq!(counter.count("ab\nac"), 1);
        assert_eq!(counter.count("a\na\na\na"), 1);
        assert_eq!(counter.count("b"), 1);

        assert_eq!(counter.count("abfg\ndean\nblakd\nopena\nxantch\nmark\nacut"), 1);
        assert_eq!(counter.count("abfg\ndeaf\nblafd\nofena\nxafnch\nmafk\nafcu"), 2);
        assert_eq!(counter.count("obfg\ndeaf\nblasd\nofena\nxafnch\nmafk\nafcu"), 0);
    }

    const TEST_GROUPS : &str = concat!("abc\n",
                                        "\n",
                                        "a\n",
                                        "b\n",
                                        "c\n",
                                        "\n",
                                        "ab\n",
                                        "ac\n",
                                        "\n",
                                        "a\n",
                                        "a\n",
                                        "a\n",
                                        "a\n",
                                        "\n",
                                        "b\n");

    #[test]
    fn summing_any() {
        assert_eq!(sum_group_answers::<AnyCounter>(&TEST_GROUPS), 11);
    }

    #[test]
    fn summing_all() {
        assert_eq!(sum_group_answers::<AllCounter>(&TEST_GROUPS), 6);
    }
}
