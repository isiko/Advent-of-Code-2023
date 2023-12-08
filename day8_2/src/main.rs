use std::collections::{BTreeSet, HashMap};

use regex::Regex;

fn main() {
    const INPUT: &str = include_str!("input");
    let path = INPUT.lines().next().unwrap().chars().collect::<Vec<char>>();

    let nodes: Vec<Edge> = INPUT
        .lines()
        .skip(2)
        .flat_map(|l| {
            let re = Regex::new(r"(.{3}) = \((.{3}), (.{3})\)");
            let caps = re.unwrap().captures(l).unwrap();

            let name = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();

            vec![
                Edge {
                    from: Node { name: name.clone() },
                    to: Node { name: left.clone() },
                    direction: Direction::Left,
                },
                Edge {
                    from: Node { name: name.clone() },
                    to: Node {
                        name: right.clone(),
                    },
                    direction: Direction::Right,
                },
            ]
        })
        .collect();

    let mut graph = Graph::new(path.iter().collect::<String>(), nodes);

    let start_nodes = graph.get_start_nodes();
    let mut states = start_nodes
        .into_iter()
        .map(|s| {
            let s = SearchState((s.clone(), 0));
            graph.get_next_final(&s)
        })
        .collect::<BTreeSet<SearchState>>();

    loop {
        //println!("States: {:?}", states);
        let state = states.pop_first().unwrap();
        if states.iter().all(|s| s.0 .1 == state.0 .1) {
            states.insert(state);
            break;
        } else {
            states.insert(graph.get_next_final(&state));
        }
    }
    println!("Final States: {:?}", states);

    let result = states.first().unwrap().0 .1;
    //assert_eq!(result, 22357);
    println!("Day 8, Task 2: {}", result);
}

struct Graph {
    edges: HashMap<(Node, Direction), Node>,
    path: Vec<Direction>,
    next_final_cache: HashMap<CacheNode, (CacheNode, usize)>,
}

impl Graph {
    fn new(path: String, vertecies: Vec<Edge>) -> Self {
        let mut edges = HashMap::new();
        vertecies.iter().for_each(|v| {
            edges.insert((v.from.clone(), v.direction), v.to.clone());
        });

        let path = path
            .chars()
            .map(|c| Direction::from_char(c))
            .collect::<Vec<Direction>>();
        Self {
            edges,
            path,
            next_final_cache: HashMap::new(),
        }
    }

    fn get_start_nodes(&self) -> Vec<Node> {
        self.edges
            .keys()
            .filter(|(node, _)| node.name.ends_with("A"))
            .map(|(node, _)| node.clone())
            .collect()
    }

    fn get_next_final(&mut self, start_state: &SearchState) -> SearchState {
        let node = CacheNode((
            start_state.0 .0.clone(),
            self.offset(start_state.0 .1).try_into().unwrap(),
        ));
        if self.next_final_cache.contains_key(&node) {
            let cache = self.next_final_cache.get(&node).unwrap().clone();
            return SearchState((cache.0 .0 .0, cache.1));
        }

        let mut next_node = node;
        let mut steps = start_state.0 .1;
        loop {
            next_node = self.get_next(&next_node);
            steps += 1;
            if next_node.is_final() {
                break;
            }
        }
        return SearchState((next_node.0 .0, steps));
    }

    fn get_next(&mut self, node: &CacheNode) -> CacheNode {
        let direction = self.path[node.get_offset() as usize];
        let offset = self.offset((node.get_offset() + 1) as usize);
        let next_node = self
            .edges
            .get(&(node.0 .0.clone(), direction))
            .unwrap()
            .clone();
        CacheNode((next_node, offset as u32))
    }

    fn offset(&self, distance: usize) -> usize {
        distance % self.path.len()
    }
}

struct Edge {
    from: Node,
    to: Node,
    direction: Direction,
}

#[derive(Debug, Clone, Eq, PartialOrd, Hash)]
struct Node {
    name: String,
}

impl Node {
    fn is_final(&self) -> bool {
        self.name.ends_with("Z")
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Hash)]
struct CacheNode((Node, u32));

impl CacheNode {
    fn get_offset(&self) -> u32 {
        self.0 .1
    }

    fn is_final(&self) -> bool {
        self.0 .0.is_final()
    }
}

impl Ord for CacheNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0 .1.cmp(&other.0 .1)
    }
}

impl PartialEq<(Node, u32)> for CacheNode {
    fn eq(&self, other: &(Node, u32)) -> bool {
        self.0 .0 == other.0 && self.0 .1 == other.1
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unknown direction"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct SearchState((Node, usize));

impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 .1 == other.0 .1 {
            return self.0 .0.name.cmp(&other.0 .0.name);
        }
        self.0 .1.cmp(&other.0 .1)
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0 .1.partial_cmp(&other.0 .1)
    }
}
