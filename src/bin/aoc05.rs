use std::fmt;
use std::fs;
use std::cmp::{Ordering, PartialOrd, PartialEq, Eq};
use std::collections::HashMap;

#[derive(Clone)]
struct MyOrdering {
    lt: HashMap<i16, Vec<i16>>,
    gt: HashMap<i16, Vec<i16>>,

    // Where did we leave off in the input file? Just to help out callers
    // with continuing to read the file.
    line_offset: usize
}

impl MyOrdering {
    pub fn new(string: &str) -> MyOrdering {
        let mut lt: HashMap<i16, Vec<i16>> = HashMap::new();
        let mut gt: HashMap<i16, Vec<i16>> = HashMap::new();
        let mut line_offset: usize = 0;

        for (i, line) in string.lines().enumerate() {
            line_offset = i;
            if line == "" {
                break;
            }
            let parts: Vec<i16> = line.split("|").map(|s| s.parse::<i16>().unwrap()).collect();
            // HashMap.entry returns a Vacant/Occupied Enum
            // .or_insert_with() acts on the Vacant case
            lt.entry(parts[0]).or_insert_with(Vec::new).push(parts[1]);
            gt.entry(parts[1]).or_insert_with(Vec::new).push(parts[0]);
        }
        MyOrdering { lt, gt, line_offset }
    }
}

#[derive(Clone)]
struct MyOrderable<'a> {
    x: i16,
    // n00b guide: &'a means a reference to a value that lives at least as long as 'a
    // (the lifetime of *this* object)
    ordering: &'a MyOrdering
}

impl fmt::Debug for MyOrderable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.x)
    }
}

// Applies to MyOrderables that qualify for comparison
impl Ord for MyOrderable<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ordering.lt.get(&self.x).map_or(false, |values| values.contains(&other.x)) {
            Ordering::Less
        } else if self.ordering.gt.get(&self.x).map_or(false, |values| values.contains(&other.x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

// We're saying that any MyOrderable can be compared to any other MyOrderable
impl PartialOrd<Self> for MyOrderable<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))  // use `cmp` from `Ord`
    }
}

impl PartialEq<Self> for MyOrderable<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

// We're saying that our PartialEq implementation is sufficient and "correct""
// i.e. reflexive
impl Eq for MyOrderable<'_> {}


fn part_a_b(string: &str) -> (i16, i16) {
    let my_ordering = MyOrdering::new(string);

    let mut result_a: i16 = 0;
    let mut result_b: i16 = 0;

    for line in string.lines().skip(my_ordering.line_offset) {
        if line == "" {
            continue;
        }
        let parts: Vec<i16> = line.split(",").map(|s| s.parse::<i16>().unwrap()).collect();
        let mut orderables: Vec<MyOrderable> = Vec::new();
        for x in parts {
            orderables.push(MyOrderable { x, ordering: &my_ordering });
        }
        if orderables.iter().is_sorted() {
            result_a += orderables[orderables.len() / 2].x;
        } else {
            let mut sorted_orderables = orderables.clone();
            sorted_orderables.sort();
            result_b += sorted_orderables[sorted_orderables.len() / 2].x;
        }
    }
    (result_a, result_b)
}

fn main() {
    let content = fs::read_to_string("input5_1.txt").expect("Could not read file");
    let result = part_a_b(content.as_str());
    print!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input: &str = "\
47|53
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
97,13,75,29,47";
        let result = part_a_b(input);
        assert_eq!(result.0, 143);
    }

    #[test]
    fn part_b_works() {
        let input: &str = "\
47|53
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
97,13,75,29,47";
        let result = part_a_b(input);
        assert_eq!(result.1, 123);
    }
}