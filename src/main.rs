use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet}, hash::{Hash, Hasher}};

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn a_star(from: (i32, i32), to: (i32, i32), speed: i32, troll_paths: &Vec<Vec<(i32, i32)>>) -> Option<Vec<(i32, i32)>> {
    
    #[derive(Copy, Clone)]
    struct Node<'a> {
        pos: (i32, i32),
        cost: i32,
        value: i32,
        parent: Option<&'a Node<'a>>
    }

    impl PartialEq for Node<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.pos == other.pos
        }
    }

    impl Eq for Node<'_> {}

    impl PartialOrd for Node<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl Hash for Node<'_> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.pos.hash(state);
        }
    }

    impl Node<'_> {
        fn new(pos: (i32, i32), to: (i32, i32)) -> Node<'static> {
            Node {
                pos,
                cost: 0,
                value: manhattan_dist(pos, to),
                parent: None
            }
        }

        fn from<'a>(parent: &'a Node<'a>, pos: (i32, i32), to: (i32, i32)) -> Node<'a> {
            let cost = parent.cost + 1;
            Node {
                pos,
                cost: cost,
                value: cost + manhattan_dist(pos, to),
                parent: Some(parent)
            }
        }

        fn collect(&self) -> Vec<(i32, i32)> {
            match self.parent {
                None => vec![],
                Some(node) => {
                    let mut res = vec![self.pos];
                    res.extend(node.collect());
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

    fn check_in_close<'a>(node_to_check: Node<'a>, close: &mut HashSet<Node<'a>>, open: &mut BinaryHeap<Reverse<Node<'a>>>) {
        match close.get(&node_to_check) {
            None => open.push(Reverse(node_to_check)),
            Some(found) => {
                if found.value > node_to_check.value {
                    open.push(Reverse(node_to_check));
                    close.remove(&found);
                }
            }
        }
    }

    let mut close = HashSet::<Node>::new();
    let mut open = BinaryHeap::<Reverse<Node>>::new();
    let mut path = None;
    let first_node = Node::new(from, to);
    open.push(Reverse(first_node));

    while let Some(Reverse(current)) = open.pop() {
        if current.pos == to {
            path = Some(current);
            break;
        }
        get_neigh(current.pos, speed)
        .iter()
        .for_each(move |&pos| check_in_close(Node::from(&current, pos, to), &mut close, &mut open));
        close.insert(current);
    }

    match path {
        None => None,
        Some(node) => Some(node.collect())
    }
}


fn main() {
    
}