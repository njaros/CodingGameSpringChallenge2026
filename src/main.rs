use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet}, hash::{Hash, Hasher}};

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn a_star(from: (i32, i32), to: (i32, i32), speed: i32, troll_paths: &Vec<Vec<(i32, i32)>>) -> Option<Vec<(i32, i32)>> {
    
    #[derive(Copy, Clone)]
    struct Node {
        pos: (i32, i32),
        cost: i32,
        value: i32,
        index: usize,
        parent: Option<usize>
    }

    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl Eq for Node {}

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl Hash for Node {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.pos.hash(state);
        }
    }

    impl Node {
        fn new(pos: (i32, i32), to: (i32, i32), index: usize) -> Node {
            Node {
                pos,
                cost: 0,
                value: manhattan_dist(pos, to),
                index,
                parent: None
            }
        }

        fn from(parent_idx: usize, parent_cost: i32, pos: (i32, i32), to: (i32, i32), index: usize) -> Node {
            let cost = parent_cost + 1;
            Node {
                pos,
                cost: cost,
                value: cost + manhattan_dist(pos, to),
                index,
                parent: Some(parent_idx)
            }
        }

        fn collect(&self, RAM_of_nodes: &Vec<Node>) -> Vec<(i32, i32)> {
            match self.parent {
                None => vec![],
                Some(idx) => {
                    let node = RAM_of_nodes[idx];
                    let mut res = vec![self.pos];
                    res.extend(node.collect(RAM_of_nodes));
                    res
                }
            }
        }
    }

    fn get_neigh(pos: (i32, i32), speed: i32) -> Vec<(i32, i32)> {
    
        fn get_pos_at_dist(pos: (i32, i32), dist: i32) -> Vec<(i32, i32)> {
            (0..=dist)
            .fold(Vec::<(i32, i32)>::new(), |mut acc, x_dist| {
                let y_dist = dist - x_dist;
                acc.push((pos.0 - x_dist, pos.1 - y_dist));
                acc.push((pos.0 + x_dist, pos.1 + y_dist));
                if y_dist != 0 && x_dist != 0 {
                    acc.push((pos.0 - x_dist, pos.1 + y_dist));
                    acc.push((pos.0 + x_dist, pos.1 - y_dist));
                }
                acc
            })
        }

        (1..=speed)
        .fold(Vec::<(i32, i32)>::new(), |mut acc, dist| {
            acc.extend(get_pos_at_dist(pos, dist));
            acc
        })
    }

    fn check_in_close(node_to_check: Node, close: &mut HashSet<Node>, open: &mut BinaryHeap<Reverse<Node>>, RAM_of_nodes: &mut Vec<Node>) {
        match close.get(&node_to_check) {
            None => {
                RAM_of_nodes.push(node_to_check);
                open.push(Reverse(node_to_check));
            }
            Some(found) => {
                if found.value > node_to_check.value {
                    RAM_of_nodes.push(node_to_check);
                    open.push(Reverse(node_to_check));
                    // Need to clone here because RUST
                    close.remove(&found.clone());
                }
            }
        }
    }

    // Rust ownerships and lifetimes attempt to implement a normal a* algorithm,
    // so no choice to create a custom RAM as Vec<Node> to get all the node references when needed.
    // (Rust fault here)
    let mut RAM_of_nodes = Vec::<Node>::new();
    let mut close = HashSet::<Node>::new();
    let mut open = BinaryHeap::<Reverse<Node>>::new();
    let mut path: Option<Node> = None;
    RAM_of_nodes.push(Node::new(from, to, 0));
    open.push(Reverse(Node::new(from, to, 0)));

    while let Some(Reverse(current)) = open.pop() {
        if current.pos == to {
            path = Some(current);
            break;
        }
        for pos in get_neigh(current.pos, speed) {
            check_in_close(Node::from(current.index, current.cost, pos, to, RAM_of_nodes.len()), &mut close, &mut open, &mut RAM_of_nodes);
        }
        close.insert(current);
    }

    match path {
        None => None,
        Some(node) => Some(node.collect(&RAM_of_nodes))
    }
}


fn main() {
    
}