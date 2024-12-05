use std::fs;

pub(crate) fn day5(input_filename: &str) {
    let contents = fs::read_to_string(input_filename).expect("could not read file");
    let valid_updates = get_valid_updates(contents);

    let sum_of_middle_pages: u64 = valid_updates.iter().map(|update| update[update.len() / 2]).sum();
    println!("sum of middle pages of valid updates: {}", sum_of_middle_pages);
}

fn get_valid_updates(input: String) -> Vec<Vec<u64>> {
    let rules = input
        .lines()
        .take_while(|&line| !line.is_empty())
        .map(|rule_line| {
            let rule_split = rule_line
                .split('|')
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            Rule(rule_split[0], rule_split[1])
        })
        .collect::<Vec<_>>();

    let updates = input
        .lines()
        .skip_while(|&line| !line.is_empty())
        .skip(1)
        .map(|update_line| {
            update_line
                .split(',')
                .map(|e| e.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("num rules: {}, num updates: {}", rules.len(), updates.len());

    let valid_updates = updates
        .iter()
        .filter(|update| rules.iter().all(|rule| rule.is_update_valid(update)))
        .map(|update| update.to_owned())
        .collect::<Vec<_>>();

    valid_updates
}

#[derive(Debug)]
struct Rule(u64, u64);

impl Rule {
    fn is_update_valid(&self, update: &Vec<u64>) -> bool {
        let mut seen_first = false;
        let mut seen_second = false;
        let mut seen_second_before_first = false;
        for &page_num in update {
            if page_num == self.1 && !seen_first {
                seen_second_before_first = true;
            }

            if page_num == self.0 {
                seen_first = true;
            } else if page_num == self.1 {
                seen_second = true;
            }
        }
        return if seen_first && seen_second {
            !seen_second_before_first
        } else {
            true
        };
    }
}

mod tests {
    use super::{get_valid_updates, Rule};

    #[test]
    fn check_example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
        assert_eq!(get_valid_updates(input.to_string()).len(), 3);
    }

    #[test]
    fn test_rule() {
        let r = Rule(47, 53);
        let update = vec![47, 53];
        assert!(r.is_update_valid(&update));
    }
}
