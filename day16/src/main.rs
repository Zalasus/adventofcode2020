

#[derive(Debug)]
struct FieldRule {
    name: String,
    start: (usize, usize),
    end: (usize, usize) // inclusive!
}

impl FieldRule {
    pub fn new_from_str(name: &str, s1: &str, s2: &str) -> Self {
        let mut parts1 = s1.split('-');
        let mut parts2 = s2.split('-');
        let begin1 = parts1.next().unwrap().parse().unwrap();
        let end1 = parts1.next().unwrap().parse().unwrap();
        let begin2 = parts2.next().unwrap().parse().unwrap();
        let end2 = parts2.next().unwrap().parse().unwrap();
        Self {
            name: name.into(),
            start: (begin1, begin2),
            end: (end1, end2)
        }
    }

    pub fn contains(&self, v: usize) -> bool {
           ((self.start.0 <= v) && (v <= self.end.0))
        || ((self.start.1 <= v) && (v <= self.end.1))
    }
}


struct Input {
    rules: Vec<FieldRule>,
    my_ticket: Vec<usize>,
    tickets: Vec<Vec<usize>>
}

impl Input {
    pub fn parse(s: &str) -> Self {
        let mut parts = s.split("\n\n");

        let rules_str = parts.next().unwrap();
        let mut rules = Vec::new();
        for rule in rules_str.split('\n') {
            let mut parts = rule.split(": ");
            let field_name = parts.next().unwrap();
            let mut range = parts.next().unwrap().split(" or ");
            let rule = FieldRule::new_from_str(field_name, range.next().unwrap(), range.next().unwrap());
            rules.push(rule);
        }

        let mut tickets = Vec::new();
        let my_ticket = parts.next().unwrap().strip_prefix("your ticket:\n").unwrap()
                             .split(',')
                             .map(|f| f.parse::<usize>().unwrap())
                             .collect::<Vec<usize>>();
        assert_eq!(my_ticket.len(), rules.len());

        let other_ticket_lines = parts.next().unwrap()
                                      .strip_prefix("nearby tickets:\n").unwrap()
                                      .split('\n')
                                      .filter(|l| !l.is_empty());
        for other_ticket_line in other_ticket_lines {
            let ticket = other_ticket_line.split(',')
                                          .map(|f| f.parse::<usize>().unwrap())
                                          .collect::<Vec<usize>>();
            tickets.push(ticket);
        }

        Self {
            rules,
            my_ticket,
            tickets
        }
    }

    pub fn remove_invalid_tickets(&mut self) -> usize {
        let mut error_rate = 0;
        let mut valid_tickets = Vec::with_capacity(self.tickets.len());
        for ticket in self.tickets.iter() {
            let mut ticket_valid = true;
            for field in ticket {
                if self.rules.iter().all(|rule| !rule.contains(*field)) {
                    error_rate += *field;
                    ticket_valid = false;
                }
            }
            if ticket_valid {
                valid_tickets.push(ticket.clone());
            }
        }

        self.tickets = valid_tickets;

        error_rate
    }

    fn check_rule(&self, rule: &FieldRule, field_index: usize) -> bool {
        self.tickets.iter().all(|t| rule.contains(t[field_index]))
    }

    pub fn find_field_order(&self) -> Vec<usize> {
        let mut candidates = Vec::new();
        for (rule_index, rule) in self.rules.iter().enumerate() {
            let mut cand = Vec::new();
            for field_index in 0..self.rules.len() {
                if self.check_rule(&rule, field_index) {
                    cand.push(field_index);
                }
            }
            candidates.push((rule_index, cand));
        }

        candidates.sort_by_key(|a| a.1.len());

        for start in 0..candidates.len() {
            assert_eq!(candidates[start].1.len(), 1);
            let unique = *candidates[start].1.first().unwrap();
            for c in candidates.iter_mut().skip(start+1) {
                c.1.retain(|i| *i != unique);
            }
        }

        candidates.sort_by_key(|a| a.0);

        candidates.iter().map(|c| *c.1.first().unwrap()).collect()
    }
}


fn main() {
    let input_str = std::fs::read_to_string("day16/input.txt").unwrap();
    let mut input = Input::parse(&input_str);

    let error_rate = input.remove_invalid_tickets();
    println!("Scanning error rate: {}", error_rate);

    let field_order = input.find_field_order();

    let mut result = 1;
    for (rule_index, rule) in input.rules.iter().enumerate() {
        let field_index = field_order[rule_index];
        if rule.name.starts_with("departure") {
            println!("{} is field {}. It has the value {}", rule.name, field_index, input.my_ticket[field_index]);
            result *= input.my_ticket[field_index];
        }
    }
    println!("Product of all 'departure' fields in my ticket: {}", result);
}


#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT : &str = concat!("class: 1-3 or 5-7\n",
                                      "row: 6-11 or 33-44\n",
                                      "seat: 13-40 or 45-50\n",
                                      "\n",
                                      "your ticket:\n",
                                      "7,1,14\n",
                                      "\n",
                                      "nearby tickets:\n",
                                      "7,3,47\n",
                                      "40,4,50\n",
                                      "55,2,20\n",
                                      "38,6,12\n");

    #[test]
    fn parse() {
        let mut input = Input::parse(TEST_INPUT);
        let error_rate = input.remove_invalid_tickets();
        assert_eq!(error_rate, 71);
    }

    const TEST_INPUT_2 : &str =
"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn field_order() {
        let mut input = Input::parse(TEST_INPUT_2);
        input.remove_invalid_tickets();
        let field_order = input.find_field_order();
        assert_eq!(field_order[0], 1);
        assert_eq!(field_order[1], 0);
        assert_eq!(field_order[2], 2);
    }
}
