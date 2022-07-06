use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

struct BagCount {
    name: String,
    count: i32,
}

struct Edge {
    parent: String,
    child: String,
    count: i32,
}

impl Edge {
    fn clone(&self) -> Edge {
        Edge {
            parent: self.parent.clone(),
            child: self.child.clone(),
            count: self.count,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input/input07.txt").unwrap();

    let edges = parse_graph(input);

    let mut edges_by_parent: HashMap<String, Vec<Edge>> = HashMap::new();
    let mut edges_by_child: HashMap<String, Vec<Edge>> = HashMap::new();

    for edge in edges {
        add_to_map(&mut edges_by_parent, &edge.parent, edge.clone());
        add_to_map(&mut edges_by_child, &edge.child, edge.clone());
    }

    let mut check_bags: HashSet<String> = HashSet::new();
    let mut checked_bags: HashSet<String> = HashSet::new();
    check_bags.insert("shiny gold".to_owned());
    while check_bags.len() > 0 {
        let mut new_check_bags = HashSet::new();
        for check_bag in check_bags.iter() {
            checked_bags.insert(check_bag.to_owned());
            match edges_by_child.get(check_bag) {
                Some(edges) => {
                    for edge in edges {
                        new_check_bags.insert(edge.parent.clone());
                    }
                }
                None => {}
            }
        }
        check_bags = new_check_bags;
    }

    println!("Part 1 - {}", checked_bags.len() - 1);

    let mut total_bag_count = 0;
    let mut bags_this_level = Vec::new();
    bags_this_level.push(BagCount {
        name: "shiny gold".into(),
        count: 1,
    });
    while bags_this_level.iter().len() > 0 {
        let mut bags_next_level = Vec::new();
        for bag_count in &bags_this_level {
            match edges_by_parent.get(&bag_count.name) {
                None => {}
                Some(edges) => {
                    for edge in edges {
                        bags_next_level.push(BagCount {
                            name: edge.child.clone(),
                            count: edge.count * bag_count.count,
                        });
                        total_bag_count += edge.count * bag_count.count;
                    }
                }
            }
        }
        bags_this_level = bags_next_level;
    }

    println!("Part 2 - {}", total_bag_count)
}

fn add_to_map(m: &mut HashMap<String, Vec<Edge>>, key: &String, edge: Edge) {
    match m.get_mut(key) {
        Some(v) => {
            v.push(edge);
        }
        None => {
            let mut v = Vec::new();
            v.push(edge);
            m.insert(key.clone(), v);
        }
    }
}

fn parse_graph(input: String) -> Vec<Edge> {
    let mut edges = Vec::new();

    for line in input.split("\n") {
        let cap = RE.captures_iter(line).next().unwrap();

        let parent_bag_name = cap.get(1).unwrap().as_str().to_owned();

        for i in 0..4 {
            let amount: i32;
            match cap.get(2 * i + 2) {
                None => break,
                Some(v) => {
                    amount = v.as_str().parse::<i32>().unwrap();
                }
            }

            let child_bag_name = cap.get(2 * i + 3).unwrap().as_str().to_owned();
            edges.push(Edge {
                parent: parent_bag_name.clone(),
                child: child_bag_name,
                count: amount,
            });
        }
    }

    edges
}

// https://regex101.com/r/Yg4ToA/1
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"^([\w\s]+) bags contain(?:(?: no other bags.)|(?:(?: (\d+) ([\w\s]+) bags?[,.])?(?: (\d+) ([\w\s]+) bags?[,.])?(?: (\d+) ([\w\s]+) bags?[,.])?(?: (\d+) ([\w\s]+) bags?[,.])?))$").unwrap();
}
