
#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

use std::collections::{HashMap, HashSet};

const MY_BAG : &str = "shiny gold";

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
    pub fn new(name: &str) -> Self {
        Self{
            name: name.to_string(),
            edges: Vec::new()
        }
    }

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

    pub fn invert_edges(&self) -> Self {
        let mut new_nodes = HashMap::with_capacity(self.nodes.len());
        for node in &self.nodes {
            new_nodes.insert(node.1.name.clone(), Node::new(&node.1.name));
        }

        for node in &self.nodes {
            for edge in &node.1.edges {
                let connected = new_nodes.get_mut(&edge.1).unwrap();
                connected.edges.push(Edge(edge.0, node.1.name.clone()));
            }
        }

        Self{
            nodes: new_nodes
        }
    }

    #[cfg(test)]
    pub fn count_edges(&self) -> usize {
        let mut result = 0;
        for node in &self.nodes {
            result += node.1.edges.len();
        }
        result
    }

    pub fn find_connected_nodes(&self, start: &str) -> HashSet<&str> {
        // we basically perform a DFS over all edges starting from the given node. after that, the
        //  set of visited nodes is the set of all nodes connected to start

        let mut visited : HashSet<&str> = HashSet::with_capacity(self.nodes.len());
        let mut stack : Vec<&str> = Vec::with_capacity(self.nodes.len());

        stack.push(start); // initial node is the first on the stack

        while !stack.is_empty() {
            let top = stack.pop().unwrap();
            let node = self.nodes.get(top).expect("invalid edge or initial node missing");
            for edge in &node.edges {
                let not_yet_visited = visited.insert(&edge.1);
                if not_yet_visited {
                    stack.push(&edge.1);
                }
            }
        }

        visited
    }

    pub fn accumulate_edge_weights(&self, start: &str) -> usize {
        // this is also a DFS, but we accumulate all edge weights. also, this one is recursive
        //  because it's too late in the evening and i can't figure out how to do this iteratively

        // visited is now a map, where the value is the number of bags contained plus one. this is
        //  to prevent traversing the graph more than necessary or following cycles
        let mut visited : HashMap<&str,usize> = HashMap::with_capacity(self.nodes.len());
        visited.insert(start, 0); // insert start node to prevent a cycle with this node from being followed
        self.recursive_accum(start, &mut visited)
    }

    // this basically calculates "how many nodes does node contain plus one"
    fn recursive_accum<'a>(&'a self, node: &'a str, visited: &mut HashMap<&'a str,usize>) -> usize {
        let me = self.nodes.get(node).unwrap();
        let mut sum = 1;
        for edge in &me.edges {
            if let Some(previous_count) = visited.get(edge.1.as_str()) {
                sum += edge.0*previous_count;
            }else{
                let contained = self.recursive_accum(&edge.1, visited);
                visited.insert(&edge.1, contained);
                sum += edge.0*contained;
            }
        }
        sum
    }
}

fn main() {
    let input = std::fs::read_to_string("day7/input.txt").unwrap();
    let graph = Graph::new(&input);

    // we can find the desired count more quickly if we traverse the bags "bottom-up". for that,
    //  we have to invert the graph ("abc bags contain xyz.." becomes "xyz can be contained in abc..")
    let inverted = graph.invert_edges();
    let valid_colors = inverted.find_connected_nodes(MY_BAG);
    println!("Number of colors that can hold a {} bag somewhere in it: {}", MY_BAG, valid_colors.len());

    let accumulated_bag_count = graph.accumulate_edge_weights(MY_BAG);
    println!("To be rule-conformant, a {} bag has to contain {} total other bags", MY_BAG, accumulated_bag_count-1);
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
        let node = Node::parse("bright lavender bags contain 2 dark lavender bags, 2 mirrored cyan bags, 1 dim yellow bag, 5 vibrant teal bags.");
        assert_eq!(node.name, "bright lavender");
        assert_eq!(node.edges.len(), 4);
        assert_eq!(node.edges[0].0, 2);
        assert_eq!(node.edges[0].1, "dark lavender");
        assert_eq!(node.edges[1].0, 2);
        assert_eq!(node.edges[1].1, "mirrored cyan");
        assert_eq!(node.edges[2].0, 1);
        assert_eq!(node.edges[2].1, "dim yellow");
        assert_eq!(node.edges[3].0, 5);
        assert_eq!(node.edges[3].1, "vibrant teal");
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

const GRAPH_INPUT_2 : &str =
"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    #[test]
    fn graph_creation_inversion() {
        let graph = Graph::new(GRAPH_INPUT);
        assert_eq!(graph.nodes.len(), 9);
        assert_eq!(graph.count_edges(), 13);

        let inverted = graph.invert_edges();
        assert_eq!(inverted.nodes.len(), 9);
        assert_eq!(inverted.count_edges(), 13);
    }

    #[test]
    fn color_count() {
        let graph = Graph::new(GRAPH_INPUT);
        let inverted = graph.invert_edges();
        let valid_colors = inverted.find_connected_nodes(MY_BAG);
        assert_eq!(valid_colors.len(), 4);
    }

    #[test]
    fn total_bag_count() {
        let graph = Graph::new(GRAPH_INPUT);
        let total_contained_bags = graph.accumulate_edge_weights(MY_BAG) - 1; // don't count the outer bag!
        assert_eq!(total_contained_bags, 32);

        let graph2 = Graph::new(GRAPH_INPUT_2);
        let total_contained_bags2 = graph2.accumulate_edge_weights(MY_BAG) - 1;
        assert_eq!(total_contained_bags2, 126);
    }

}
