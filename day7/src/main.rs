
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref NODE_REGEX : Regex = Regex::new("(\\D+?) bags contain ").unwrap();
    static ref EDGE_REGEX : Regex = Regex::new("(\\d+) (\\D+?) bags?").unwrap();
}

struct Edge(usize, String);

struct Node {
    name: String,
    edges: Vec<Edge>
}

impl Node {
    pub fn parse(s: &str) -> Self {
        let node_matches = NODE_REGEX.captures(s).unwrap();
        let name = node_matches[1].to_string();

        let mut edges = Vec::with_capacity(2);
        let edge_specifier = &s[node_matches.get(0).unwrap().start()..];
        for edge_match in EDGE_REGEX.captures_iter(edge_specifier) {
            let count : usize = edge_match[1].parse().unwrap();
            let node : String = edge_match[2].to_string();
            edges.push(Edge(count,node));
        }

        Self{
            name,
            edges
        }
    }
}

struct Graph {
    nodes: HashMap<String, Node>
}

impl Graph {
    pub fn new(s: &str) -> Self {
        let mut nodes = HashMap::new();
        for line in s.split('\n').filter(|l| !l.is_empty()) {
            let node = Node::parse(line);
            if let Some(_) = nodes.get(&node.name) {
                panic!("Duplicate node in graph string");
            }else{
                nodes.insert(node.name.clone(), node);
            }
        }

        Self {
            nodes
        }
    }

    pub fn find_valid_bag_colors(&self, exact_quantities_only: bool) -> HashSet<String> {
        // we basically perform a DFS over all edges starting from the shiny golden bag,
        //  potentially ignoring those that have a quantity other than one. after that, the set of
        //  visited nodes is the set of all valid bag colors
        let mut visited : HashSet<String> = HashSet::with_capacity(self.nodes.len());
        let mut stack : Vec<&str> = Vec::with_capacity(self.nodes.len());

        stack.push("shiny gold"); // initial node is the first on the stack

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let node = self.nodes.get(top).unwrap();
            for edge in &node.edges {
                if exact_quantities_only && edge.0 > 1 {
                    continue;
                }
                let not_yet_visited = visited.insert(edge.1.clone());
                if not_yet_visited {
                    stack.push(&edge.1);
                }
            }
        }

        visited
    }
}

fn main() {
    let input = std::fs::read_to_string("day7_input.txt").unwrap();
    let graph = Graph::new(&input);
    let valid_colors = graph.find_valid_bag_colors(false);
    println!("Number of colors that can hold a shiny gold bag: {}", valid_colors.len());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_single_edge() {
        let node = Node::parse("bright white bags contain 1 shiny gold bag.");
        assert_eq!(node.name, "bright white");
        assert_eq!(node.edges.len(), 1);
        assert_eq!(node.edges[0].0, 1);
        assert_eq!(node.edges[0].1, "shiny gold");
    }

    #[test]
    fn parse_multi_edge() {
        let node = Node::parse("light red bags contain 1 bright white bag, 2 muted yellow bags.");
        assert_eq!(node.name, "light red");
        assert_eq!(node.edges.len(), 2);
        assert_eq!(node.edges[0].0, 1);
        assert_eq!(node.edges[0].1, "bright white");
        assert_eq!(node.edges[1].0, 2);
        assert_eq!(node.edges[1].1, "muted yellow");
    }

    #[test]
    fn parse_no_edge() {
        let node = Node::parse("dotted black bags contain no other bags.");
        assert_eq!(node.name, "dotted black");
        assert_eq!(node.edges.len(), 0);
    }

    const GRAPH_INPUT : &str =
"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn simple_graph() {
        let graph = Graph::new(GRAPH_INPUT);
        let valid_colors = graph.find_valid_bag_colors(false);
        assert_eq!(valid_colors.len(), 4);
    }

}
