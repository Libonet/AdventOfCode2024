use std::{fs::read_to_string, io, cmp::Ordering};

fn part1(contents: String) -> u128 {
    let mut identified = translate(contents);

    compact(&mut identified);
    
    let mut pos: u128 = 0;
    let result: u128 = identified.iter()
        .fold(0, |acc,node| {
            if node.kind == NodeKind::File {
                let mut sum = 0;
                for _i in 0..node.size {
                    sum += node.id.unwrap() * pos;
                    pos += 1;
                }

                acc + sum
            } else {
                acc
            }
        });
    
    println!("result = {result}");

    result
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum NodeKind {
    File,
    Space,
}

#[derive(Debug, Clone, Copy)]
struct Node {
    kind: NodeKind,
    id: Option<u128>,
    size: u32,
}

fn translate(contents: String) -> Vec<Node> {
    let mut id = 0;
    let mut res: Vec<Node> = Vec::new();

    let mut iter = contents.chars();
    while let Some(val) = iter.next() {
        let size = val.to_digit(10).unwrap();
        
        res.push(Node { kind: NodeKind::File, id: Some(id), size });
        id += 1;

        if let Some(space) = iter.next() {
            let size = space.to_digit(10).unwrap();

            res.push(Node { kind: NodeKind::Space, id: None, size });
        }
    }

    res
}

fn compact(segment: &mut Vec<Node>){
    let mut i = 0;
    let mut j = segment.len()-1;

    while i < j {
        if segment[i].kind == NodeKind::File {
            i += 1;
            continue;
        }
        if segment[j].kind == NodeKind::Space {
            j -= 1;
            continue;
        }
        
        switch_nodes(segment, i, j);
    }
}

fn switch_nodes(segment: &mut Vec<Node>, i: usize, j: usize){
    let space = segment[i];
    let file = segment[j];

    match file.size.cmp(&space.size) {
        Ordering::Less => {
            segment[i].size -= file.size;
            segment[j] = space;
            segment[j].size -= segment[i].size;
            segment.insert(i, file);
        },
        Ordering::Greater => {
            segment[i] = Node { kind: NodeKind::File, id: file.id, size: space.size };
            segment[j].size -= space.size;
        },
        Ordering::Equal => {
            segment[j] = space;
            segment[i] = file;
        },
    }
}

fn part2(contents: String) -> u128 {
    let mut identified = translate(contents);

    compact_whole_file(&mut identified);

    let mut pos: u128 = 0;
    let result: u128 = identified.iter()
        .fold(0, |acc,node| {
            if node.kind == NodeKind::File {
                let mut sum = 0;
                for _i in 0..node.size {
                    sum += node.id.unwrap() * pos;
                    pos += 1;
                }

                acc + sum
            } else {
                pos += node.size as u128;
                acc
            }
        });

    println!("result = {result}");

    result
}

fn compact_whole_file(segment: &mut Vec<Node>){
    let mut file_pointer = (segment.len()-1) as isize;
    let mut i = 0;

    while file_pointer >= 0 {
        while file_pointer >= 0 {
            let j = file_pointer as usize;
            if segment[j].kind == NodeKind::File {
                break;
            } else {
                file_pointer -= 1;
            }
        }

        let j = file_pointer as usize;
        while i < j {
            if segment[i].kind == NodeKind::Space {
                if segment[i].size >= segment[j].size {
                    switch_nodes(segment, i, j);
                    break;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        i = 0;
        file_pointer -= 1;
    }
}

pub fn answer() -> Result<(), io::Error>{
    let contents = read_to_string("input/day09.txt")?;
    let contents = contents.replace("\n", "");

    println!("Part1:");
    let _ = part1(contents.clone());

    println!("Part2:");
    let _ = part2(contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let input = "2333133121414131402".to_string();

        let result = part1(input);

        assert_eq!(result, 1928);
    }

    #[test]
    fn part2_test() {
        let input = "2333133121414131402".to_string();

        let result = part2(input);

        assert_eq!(result, 2858);
    }
}
