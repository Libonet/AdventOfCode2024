use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use std::{fs::read_to_string, io};
use std::time::Instant;

type Nodes = HashSet<String>;
struct Graph (HashMap<String, Nodes>);
type Input = Graph;

impl Graph {
    fn new() -> Self {
        Self(HashMap::new())
    }

    //fn degree(&self, v: &str) -> Option<usize> {
    //    let node = self.get(v)?;
    //
    //    Some(node.len())
    //}
}

impl Deref for Graph {
    type Target = HashMap<String, Nodes>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day23.txt")?; // input/dayxx.txt
    
    let input = parse(contents);
    
    println!("Part1:");
    let now = Instant::now();
    let part1_res = part1(&input);
    let elapsed = now.elapsed();
    println!("result = {part1_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    println!("Part2:");
    let now = Instant::now();
    let part2_res = part2(&input);
    let elapsed = now.elapsed();
    println!("result = {part2_res:?}");
    println!("Time taken: {:.2?}", elapsed);

    Ok(())
}

fn parse(contents: String) -> Input {
    let mut graph = Graph::new();

    for line in contents.lines() {
        let node1 = line[..2].to_string();
        let node2 = line[3..].to_string();

        let node1_list = graph.entry(node1.clone()).or_default();
        node1_list.insert(node2.clone());

        let node2_list = graph.entry(node2).or_default();
        node2_list.insert(node1);
    }

    graph
}

type Count = usize;
fn part1(input: &Input) -> Count {
    let points = input.keys().cloned().collect();
    let mut clique_list = Vec::new();
    bron_kerbosch_sized(3, input, HashSet::new(), points, HashSet::new(), &mut clique_list);

    let mut count: usize = 0;
    for clique in clique_list {
        if clique.len() == 3 {
            for v in &clique {
                if v.starts_with("t") {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn bron_kerbosch_sized(
    ksize: usize,
    graph: &Graph,
    clique: HashSet<String>,
    points: HashSet<String>,
    exclude: HashSet<String>,
    result: &mut Vec<HashSet<String>>) {
    let mut points = points;
    let mut exclude = exclude;

    if clique.len() == ksize {
        result.push(clique);
        return;
    }

    if points.is_empty() && exclude.is_empty() {
        return;
    }

    let iter: Vec<String> = points.iter().cloned().collect();
    for v in iter {
        let mut new_clique = clique.clone();
        new_clique.insert(v.clone());
        let new_points: HashSet<String> = points.intersection(&graph[&v]).cloned().collect();
        let new_exclude: HashSet<String> = exclude.intersection(&graph[&v]).cloned().collect();
        bron_kerbosch_sized(
            ksize,
            graph,
            new_clique,
            new_points,
            new_exclude,
            result);
        points.remove(&v);
        exclude.insert(v.to_string());
    }
}

fn bron_kerbosch(
    graph: &Graph,
    clique: HashSet<String>,
    points: HashSet<String>,
    exclude: HashSet<String>,
    result: &mut Vec<HashSet<String>>) {
    let mut points = points;
    let mut exclude = exclude;

    if points.is_empty() && exclude.is_empty() {
        result.push(clique);
        return;
    }

    // choose a pivot vertex u in {points U exclude}
    let pivot = points.union(&exclude).next().unwrap();
    //for u in points.union(&exclude) {
    //    if graph.degree(u) > graph.degree(pivot) {
    //        pivot = u;
    //    }
    //}

    let difference: Vec<String> = points.difference(&graph[pivot]).cloned().collect();
    for v in difference {
        let mut new_clique = clique.clone();
        new_clique.insert(v.clone());
        let new_points: HashSet<String> = points.intersection(&graph[&v]).cloned().collect();
        let new_exclude: HashSet<String> = exclude.intersection(&graph[&v]).cloned().collect();
        bron_kerbosch(
            graph,
            new_clique,
            new_points,
            new_exclude,
            result);
        points.remove(&v);
        exclude.insert(v.to_string());
    }
}

fn part2(input: &Input) -> String {
    let points = input.keys().cloned().collect();
    let mut result = Vec::new();
    bron_kerbosch(input, HashSet::new(), points, HashSet::new(), &mut result);

    let max = result.iter().max_by(|x,y| x.len().cmp(&y.len()));

    let mut max = max.unwrap().iter().cloned().collect::<Vec<String>>();
    max.sort();
    let mut iter = max.into_iter();
    let mut ret = String::new();
    ret.push_str(&iter.next().unwrap());

    for next in iter {
        ret.push(',');
        ret.push_str(&next);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let contents = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn".to_string();

        let input = parse(contents);

        let result = part1(&input);

        assert_eq!(result, 7);
    }

    #[test]
    fn test_bron_kerbosch() {
        let contents = "\
si-fo
fo-fi
fo-th
th-tw
fi-tw
tw-on
fi-on
";

        let input = parse(contents.to_string());
        
        let points = input.keys().cloned().collect();
        let mut result = Vec::new();
        bron_kerbosch(
            &input,
            HashSet::new(),
            points,
            HashSet::new(),
            &mut result);

        //println!("bron results = {result:#?}");

        assert_eq!(result.len(), 5);
    }
}
