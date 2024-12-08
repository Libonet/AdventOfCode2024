use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string, hash::Hash, io};

fn part1() -> Result<(), io::Error>{
    let contents = read_to_string("input/day05.txt")?;

    let (rules,updates) = contents.split_once("\n\n").unwrap();

    let rule_map = process_rules(rules);

    let result = process_updates(updates, rule_map);

    println!("result = {result}");

    Ok(())
}

fn process_rules(rules: &str) -> HashMap<i32,HashSet<i32>> {
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in rules.lines() {
        let required = rule[..2].parse::<i32>().unwrap();
        let requiree = rule[3..].parse::<i32>().unwrap();

        let rule = rule_map.entry(required).or_default();
        rule.insert(requiree);
    }

    rule_map
}

fn process_updates(updates: &str, rule_map: HashMap<i32,HashSet<i32>>) -> i32 {
    updates.lines()
        .map(|update| {
            let mut appears: HashSet<i32> = HashSet::new();
            let update: Vec<i32> = update.split(',')
                .map(|num| {
                    let num = num.parse::<i32>().unwrap();
                    appears.insert(num);
                    num
                })
                .collect();

            let local_rules: HashMap<i32, HashSet<i32>> = update.iter()
                .map(|num| {
                    let default = HashSet::new();
                    (*num, rule_map.get(num).unwrap_or(&default).intersection(&appears).copied().collect())
                })
                .collect();

            let mut appears_after: HashSet<i32> = HashSet::with_capacity(update.len());
            for start_node in &update {
                appears_after.remove(start_node);
                for end_node in local_rules.get(start_node).unwrap() {
                    appears_after.insert(*end_node);
                }
            }
            let valid = appears_after.is_empty();

            if valid {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part2() -> Result<(), io::Error>{
    let contents = read_to_string("input/day05.txt")?;

    let (rules,updates) = contents.split_once("\n\n").unwrap();

    let rule_map = process_rules(rules);

    let result = fix_updates(updates, rule_map);

    println!("result = {result}");
    
    Ok(())
}

#[derive(Debug, Default)]
struct Node<T> {
    in_count: u32,
    children: Vec<T>,
}

#[derive(Default)]
struct TopoGraph<T> {
    nodes: HashMap<T, Node<T>>,
}

impl<T> TopoGraph<T> 
where 
    T: Default + PartialEq + Eq + Hash + Clone,
{
    fn new() -> Self {
        Self::default()
    }

    fn add_edge(&mut self, start: T, end: T) {
        let start_node = self.nodes.entry(start).or_default();
        start_node.children.push(end.clone());
        let end_node = self.nodes.entry(end).or_default();
        end_node.in_count += 1;
    }

    fn kahn_algorithm(mut self) -> Vec<T> {
        let mut q: VecDeque<_> = self.nodes
            .iter()
            .filter_map(|(val, node)| if node.in_count == 0 { Some(val.clone()) } else { None })
            .collect();

        let mut result = Vec::new();

        while let Some(val) = q.pop_front() {
            let node = self.nodes.get_mut(&val).unwrap();
            for child in std::mem::take(&mut node.children) {
                let child_node = self.nodes.get_mut(&child).unwrap();
                child_node.in_count -= 1;
                if child_node.in_count == 0 {
                    q.push_back(child);
                }
            }
            result.push(val);
        }

        result
    }
}

fn build_topograph(local_rules: &HashMap<i32, HashSet<i32>>, update: &[i32]) -> TopoGraph<i32> {
    let mut result: TopoGraph<i32> = TopoGraph::new();

    let default: HashSet<i32> = HashSet::new();
    for a in update {
        let a_rules = local_rules.get(a).unwrap_or(&default);
        for b in a_rules {
            result.add_edge(*a, *b);
        }
    }

    result
}

fn fix_updates(updates: &str, rule_map: HashMap<i32,HashSet<i32>>) -> i32{
    updates.lines()
        .map(|update| {
            let mut appears: HashSet<i32> = HashSet::new();
            let update: Vec<i32> = update.split(',')
                .map(|num| {
                    let num = num.parse::<i32>().unwrap();
                    appears.insert(num);
                    num
                })
                .collect();

            let local_rules: HashMap<i32, HashSet<i32>> = update.iter()
                .map(|num| {
                    let default = HashSet::new();
                    (*num, rule_map.get(num).unwrap_or(&default).intersection(&appears).copied().collect())
                })
                .collect();

            let mut appears_after: HashSet<i32> = HashSet::with_capacity(update.len());
            for start_node in &update {
                appears_after.remove(start_node);
                for end_node in local_rules.get(start_node).unwrap() {
                    appears_after.insert(*end_node);
                }
            }
            let valid = appears_after.is_empty();

            if !valid {
                let sorted = build_topograph(&local_rules, &update).kahn_algorithm();
                sorted[sorted.len() / 2]
            } else {
                0
            }
        })
        .sum()
}



/*
* this isnt working, for some reason
* an implementation of kahn's algorithm
fn topological_sort(update: &[i32], adj: &HashMap<i32, HashSet<i32>>) -> Option<Vec<i32>> {
    let mut indegree: HashMap<i32, i32> = HashMap::with_capacity(update.len());
    for start_node in update {
        indegree.insert(*start_node, 0);
        for end_node in adj.get(start_node).unwrap() {
            indegree.entry(*end_node).and_modify(|v| *v += 1);
        };
    }

    let mut q: VecDeque<i32> = VecDeque::new();
    for num in update {
        if *indegree.get(num).unwrap() == 0 {
            q.push_back(*num);
        }
    }

    let mut result: Vec<i32> = Vec::with_capacity(update.len());
    while !q.is_empty() {
        let node: i32 = q.pop_front().unwrap();
        result.push(node);

        // decrease indegree of adjacent vertices as the current node is in topological order
        for num in adj.get(&node).unwrap() {
            indegree.entry(*num).and_modify(|v| *v -= 1);

            if *indegree.get(num).unwrap() == 0 {
                q.push_back(*num);
            }
        }

    }
    // check for cycle
    if result.len() != update.len() {
        return None;
    }

    Some(result)
}
*/

pub fn answer() -> Result<(), io::Error>{
    println!("Part1:");
    part1()?;

    println!("Part2:");
    part2()?;

    Ok(())
}

#[cfg(test)]
mod test_topo {
    use super::*;

    #[test]
    fn fix_updates_with_topo() {
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
97,13,75,29,47";

        let (rules, _updates) = input.split_once("\n\n").unwrap();

        let rules = process_rules(rules);

        println!("rules = {rules:#?}");

        let updates = 
"75,97,47,61,53
61,13,29
97,13,75,29,47";

        let result = fix_updates(updates, rules);

        assert_eq!(result, 123);
    }
}
