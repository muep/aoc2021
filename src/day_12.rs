use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Node {
    Start,
    End,
    Small([char; 2]),
    Big([char; 2]),
}

impl Node {
    fn from_name(name: &str) -> Node {
        match name {
            "start" => Node::Start,
            "end" => Node::End,
            cave_name => {
                let name_array = {
                    let mut arr = ['\0'; 2];
                    let mut chars = cave_name.chars();
                    arr[0] = chars.next().unwrap();
                    arr[1] = chars.next().unwrap_or('\0');
                    arr
                };
                if name_array[0].is_ascii_lowercase() {
                    Node::Small(name_array)
                } else {
                    Node::Big(name_array)
                }
            }
        }
    }
}

struct Network {
    ncon: HashMap<Node, HashSet<Node>>,
}

fn load(input: &mut dyn Read) -> Network {
    let mut ncon = HashMap::new();

    ncon.insert(Node::Start, HashSet::new());
    ncon.insert(Node::End, HashSet::new());

    for (node0, node1) in BufReader::new(input).lines().map(|l| {
        let line_text = l.unwrap();
        let mut names = line_text.split('-');
        let node0 = Node::from_name(names.next().unwrap());
        let node1 = Node::from_name(names.next().unwrap());
        (node0, node1)
    }) {
        for (n0, n1) in [(node0, node1), (node1, node0)] {
            if !ncon.contains_key(&n0) {
                ncon.insert(n0, HashSet::new());
            }
            let n0_set: &mut HashSet<Node> = ncon.get_mut(&n0).unwrap();
            n0_set.insert(n1);
        }
    }

    Network { ncon }
}

fn routes(prefix: &[Node], net: &Network, permit_small_twice: bool) -> Vec<Vec<Node>> {
    let here = prefix.last().unwrap();
    if *here == Node::End {
        return vec![prefix.to_vec()];
    }

    let mut p = prefix.to_vec();

    net.ncon[&here]
        .iter()
        .filter_map(|n| match n {
            Node::Start => None,
            Node::Small(_) => {
                if !prefix.contains(n) {
                    Some((permit_small_twice, n))
                } else if permit_small_twice {
                    Some((false, n))
                } else {
                    None
                }
            }
            Node::Big(_) => Some((permit_small_twice, n)),
            Node::End => Some((permit_small_twice, n)),
        })
        .map(|(permit_twice, next)| {
            p.push(*next);
            let res = routes(&p, net, permit_twice);
            p.pop();
            res
        })
        .flatten()
        .collect()
}

fn part1(input: &mut dyn Read) -> u32 {
    let net = load(input);
    routes(&[Node::Start], &net, false).len() as u32
}

fn part2(input: &mut dyn Read) -> u32 {
    let net = load(input);
    routes(&[Node::Start], &net, true).len() as u32
}

pub fn run_part1(input: &mut dyn Read) {
    println!("{}", part1(input));
}

pub fn run_part2(input: &mut dyn Read) {
    println!("{}", part2(input));
}

pub fn run_to_dot(input: &mut dyn Read) {
    println!("graph {{");
    for l in BufReader::new(input).lines().map(|l| l.unwrap()) {
        println!("  {}", l.replace('-', " -- "));
    }
    println!("}}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Seek;

    #[test]
    fn test_sample_1() {
        let mut f = File::open("input/day-12-sample-1.txt").unwrap();
        assert_eq!(part1(&mut f), 10);
        f.rewind().unwrap();
        assert_eq!(part2(&mut f), 36);
    }

    #[test]
    fn test_sample_2() {
        let mut f = File::open("input/day-12-sample-2.txt").unwrap();
        assert_eq!(part1(&mut f), 19);
        f.rewind().unwrap();
        assert_eq!(part2(&mut f), 103);
    }

    #[test]
    fn test_sample_3() {
        let mut f = File::open("input/day-12-sample-3.txt").unwrap();
        assert_eq!(part1(&mut f), 226);
        f.rewind().unwrap();
        assert_eq!(part2(&mut f), 3509);
    }

    #[test]
    fn test_full() {
        let mut f = File::open("input/day-12.txt").unwrap();
        assert_eq!(part1(&mut f), 4749);
        f.rewind().unwrap();
        assert_eq!(part2(&mut f), 123054);
    }
}
