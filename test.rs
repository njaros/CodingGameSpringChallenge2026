extern crate priority-queue;
use priority_queue::PriorityQueue;

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

struct Node<'a> {
    pos: (i32, i32),
    cost: i32,
    value: i32,
    parent: Option<&'a Node<'a>>
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

fn main() {
    println!("{:?}", get_neigh((5, 5), 3));

    let a = Node::new((0, 0), (5, 4));
    println!("a: {:?}, {}, {}", a.pos, a.cost, a.value);

    let b = Node::from(&a, (0, 1), (5, 4));
    println!("b: {:?}, {}, {}, parent pos: {:?}", b.pos, b.cost, b.value, b.parent.unwrap().pos);
}