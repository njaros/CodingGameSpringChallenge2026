extern crate itertools;

use std::{fmt, io, cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashSet}, hash::{Hash, Hasher}};
use itertools::Itertools;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// TROLL CHARACTERISTICS CONSTANTS TO GET CHARACTERISTICS ARRAY INDEXES

const MS: usize = 0;
const CAPA: usize = 1;
const HARV: usize = 2;
const CHOP: usize = 3;

// COST INDEXES or OBJECTIVES

const PLUM: usize = 0;
const LEMON: usize = 1;
const APPLE: usize = 2;
const BANANA: usize = 3;
const IRON: usize = 4;
const WOOD: usize = 5;
const ANY: usize = 6;
const PLANT_PLUM: usize = 7;
const PLANT_LEMON: usize = 8;
const PLANT_APPLE: usize = 9;
const PLANT_BANANA: usize = 10;
const PLANT_ANY: usize = 11;
const CHOP_PLUM: usize = 11;
const CHOP_LEMON: usize = 12;
const CHOP_APPLE: usize = 13;
const CHOP_BANANA: usize = 14;
const CHOP_ANY: usize = 15;
const CUT_ALL: usize = 16;
const THIEF: usize = 17;
const THIEF_NO_WASTE: usize = 18;
const BANANA_TRICKS: usize = 19;

// MAP TYPE

const DISTANT: usize = 0;
const NEAR: usize = 1;

// UTILITIES

fn manhattan_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
}

fn safe_push_positions(positions: &Vec<(i32, i32)>, to_vec: &mut Vec<(i32, i32)>, max_h: i32, max_w: i32) {
    positions
    .iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < max_w && *y < max_h)
    .for_each(|pos| to_vec.push(*pos))
}

fn a_star(grid: &Grid, from: (i32, i32), to: (i32, i32), speed: i32, troll_paths: &mut Vec<Vec<(i32, i32)>>, ignore_collide: bool) -> Option<Vec<(i32, i32)>> {
    
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

        fn collect(&self, ram_of_nodes: &Vec<Node>) -> Vec<(i32, i32)> {
            match self.parent {
                None => vec![],
                Some(idx) => {
                    let node = ram_of_nodes[idx];
                    let mut res = node.collect(ram_of_nodes);
                    res.extend([self.pos]);
                    res
                }
            }
        }
    }

    fn get_neigh(pos: (i32, i32), speed: i32, grid: &Grid, visited: &mut HashSet<(i32, i32)>, current_cost: i32, troll_paths: &Vec<Vec<(i32, i32)>>, ignore_collide: bool) -> Vec<(i32, i32)> {
        match speed {
            0 => vec![],
            n => {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .fold(Vec::<(i32, i32)>::new(), |mut acc, (x, y)| {
                    let new_pos = (pos.0 + x, pos.1 + y);
                    let positions = troll_paths.get((current_cost - 1) as usize);
                    if
                        positions.is_none_or(|pos| !pos.contains(&new_pos)) &&
                        new_pos.0 >= 0 && new_pos.1 >= 0 &&
                        new_pos.0 < grid.width && new_pos.1 < grid.height &&
                        grid.grid[new_pos.1 as usize][new_pos.0 as usize].is_grass()
                    {
                        match visited.insert(new_pos) {
                            true => {
                                if ignore_collide || grid.grid[new_pos.1 as usize][new_pos.0 as usize].can_walk_through() {
                                    acc.push(new_pos);
                                }
                                acc.extend(get_neigh(new_pos, n - 1, grid, visited, current_cost, troll_paths, ignore_collide))
                            },
                            false => {}
                        }
                    }
                    acc
                })
            }
        }
    }

    fn check_in_close(node_to_check: Node, close: &mut HashSet<Node>, open: &mut BinaryHeap<Reverse<Node>>, ram_of_nodes: &mut Vec<Node>) {
        match close.get(&node_to_check) {
            None => {
                ram_of_nodes.push(node_to_check);
                open.push(Reverse(node_to_check));
            }
            Some(found) => {
                if found.value > node_to_check.value {
                    ram_of_nodes.push(node_to_check);
                    open.push(Reverse(node_to_check));
                    // Need to clone here because RUST
                    close.remove(&found.clone());
                }
            }
        }
    }

    // Rust ownerships and lifetimes attempt to implement any a* algorithm,
    // so no choice to create a custom RAM as Vec<Node> to get all the node references when needed.
    // (Rust fault here)
    let mut ram_of_nodes = Vec::<Node>::new();
    let mut close = HashSet::<Node>::new();
    let mut open = BinaryHeap::<Reverse<Node>>::new();
    let mut path: Option<Node> = None;
    ram_of_nodes.push(Node::new(from, to, 0));
    open.push(Reverse(Node::new(from, to, 0)));

    while let Some(Reverse(current)) = open.pop() {
        if current.pos == to {
            path = Some(current);
            break;
        }
        for pos in get_neigh(current.pos, speed, grid, &mut HashSet::<(i32, i32)>::new(), current.cost, troll_paths, ignore_collide) {
            check_in_close(Node::from(current.index, current.cost, pos, to, ram_of_nodes.len()), &mut close, &mut open, &mut ram_of_nodes);
        }
        close.insert(current);
    }

    match path {
        None => None,
        Some(node) => {
            let path = node.collect(&ram_of_nodes);
            let diff_size = path.len() as i32 - troll_paths.len() as i32;
            if diff_size > 0 {
                (0..diff_size).into_iter().for_each(|_| troll_paths.push(Vec::<(i32, i32)>::new()));
            }
            path.iter().enumerate().for_each(|(idx, &pos)| troll_paths[idx].push(pos));
            Some(path)
        }
    }
}

// TREE PART

#[derive(Copy, Clone, PartialEq, Eq)]
enum Fruit {
    Plum,
    Lemon,
    Apple,
    Banana
}

impl Fruit {
    fn from(fruit_as_string: &String) -> Fruit {
        match fruit_as_string.as_str() {
            "PLUM" => Fruit::Plum,
            "LEMON" => Fruit::Lemon,
            "APPLE" => Fruit::Apple,
            "BANANA" => Fruit::Banana,
            _ => unreachable!("Unknown fruit {fruit_as_string}")
        }
    }

    fn from_usize(fruit_as_usize: usize) -> Fruit {
        match fruit_as_usize {
            PLUM | PLANT_PLUM | CHOP_PLUM => Fruit::Plum,
            LEMON | PLANT_LEMON | CHOP_LEMON => Fruit::Lemon,
            APPLE | PLANT_APPLE | CHOP_APPLE => Fruit::Apple,
            BANANA | PLANT_BANANA | CHOP_BANANA | BANANA_TRICKS => Fruit::Banana,
            _ => unreachable!("Unknowm fruit_as_usize {fruit_as_usize}")
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            Fruit::Plum => 0,
            Fruit::Lemon => 1,
            Fruit::Apple => 2,
            Fruit::Banana => 3
        }
    }
}

impl fmt::Display for Fruit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Fruit::Apple => "APPLE",
            Fruit::Banana => "BANANA",
            Fruit::Lemon => "LEMON",
            Fruit::Plum => "PLUM"
        })
    }
}

#[derive(Copy, Clone)]
struct Tree {
    pub fruit: Fruit,
    pub nb_fruit: i32,
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub health: i32,
    pub cooldown: i32,
}

impl Tree {
    fn new(fruit: Fruit, nb_fruit: i32, x: i32, y: i32, size: i32, health: i32, cooldown: i32) -> Tree {
        Tree {
            fruit, nb_fruit, x, y, size, health, cooldown
        }
    }

    fn from(input_line: &String) -> Tree {
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Tree::new(
            Fruit::from(&inputs[0].trim().to_string()),
            parse_input!(inputs[5], i32),
            parse_input!(inputs[1], i32),
            parse_input!(inputs[2], i32),
            parse_input!(inputs[3], i32),
            parse_input!(inputs[4], i32),
            parse_input!(inputs[6], i32),
        )
    }

    fn from_stdin() -> Tree {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        Tree::from(&input_line)
    }

    fn from_stdin_all() -> Vec::<Tree> {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let trees_count = parse_input!(input_line, i32);
        (0..trees_count)
        .fold(Vec::<Tree>::new(), |mut acc, _| {
            acc.push(Tree::from_stdin());
            acc
        })
    }
}

// TROLL PART

#[derive(Copy, Clone, PartialEq, Eq)]
enum TrollType {
    // Troll having all charac > 0 (like the first one)
    Poly,
    // Troll having no carryCapacity but chopPower and movementSpeed to destroy enemy trees
    Griefer,
    // Troll having chopPower and carryCapacity
    Choper,
    // Troll having harvestPower and carryCapacity
    Peasant,
    // Troll having chopPower and carryCapacity but his
    // carryCapacity and movementSpeed according to the iron mine distance
    Miner
}

impl TrollType {
    fn get_type(charac: [i32; 4]) -> TrollType {
        match charac {
            [1, 1, 1, 1] => TrollType::Poly,
            [2, 4, 0, 3] | [2, 2, 0, 2] => TrollType::Choper,
            [2, 0, 0, 2] => TrollType::Griefer,
            [2, 2, 0, 1] => TrollType::Miner,
            _ => TrollType::Peasant,
        }
    }

    fn charac_need(&self, game_state: i32, mine_dist: i32) -> [i32; 4] {
        match self {
            TrollType::Poly => [1, 1, 1, 1],
            TrollType::Choper => [2, 4, 0, 3],
            TrollType::Miner => [2, 2, 0, 1],
            TrollType::Griefer => [2, 0, 0, 2],
            TrollType::Peasant => [2, 2, 2, 1]
        }
    }

    fn best_possible_train(inventory: &Inventory, total_trolls: i32) -> [i32; 4] {
        fn to_higher_square_root(val: i32) -> i32 {
            match val {
                0 => 0,
                1..=3 => 1,
                4..=8 => 2,
                9..=15 => 3,
                16..=24 => 4,
                25..=35 => 5,
                36..=48 => 6,
                49..=63 => 7,
                64..=80 => 8,
                81..=99 => 9,
                _ => unreachable!("faut pas deconner...")
            }
        }
        let speed_max = to_higher_square_root(inventory.resources[PLUM] - total_trolls);
        let capa_max = to_higher_square_root(inventory.resources[LEMON] - total_trolls);
        let harvest_max = i32::min(capa_max, to_higher_square_root(inventory.resources[APPLE] - total_trolls));
        let chop_max = i32::min(capa_max, to_higher_square_root(inventory.resources[IRON] - total_trolls));
        [speed_max, capa_max, harvest_max, chop_max]
    }

    fn cost(&self, game_state: i32, mine_dist: i32, total_trolls: i32) -> [i32; 6] {
        let charac = self.charac_need(game_state, mine_dist);
        [
            total_trolls + charac[MS] * charac[MS],
            total_trolls + charac[CAPA] * charac[CAPA],
            total_trolls + charac[HARV] * charac[HARV],
            0,
            total_trolls + charac[CHOP] * charac[CHOP],
            0
        ]
    }

    fn from_usize(n: usize) -> TrollType {
        match n {
            0 => TrollType::Poly,
            1 => TrollType::Peasant,
            2 => TrollType::Miner,
            3 => TrollType::Choper,
            4 => TrollType::Griefer,
            _ => unreachable!("unknown troll type {n}")
        }
    }

    fn to_usize(&self) -> usize {
        match self {
            TrollType::Poly => 0,
            TrollType::Peasant => 1, 
            TrollType::Miner => 2,
            TrollType::Choper => 3,
            TrollType::Griefer => 4
        }
    }

    fn default_objective(&self) -> usize {
        match self {
            TrollType::Poly => ANY,
            TrollType::Peasant => ANY, 
            TrollType::Miner => IRON,
            TrollType::Choper => CHOP_ANY,
            TrollType::Griefer => ANY
        }
    }
}

impl fmt::Display for TrollType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TrollType::Choper => "CHOPER",
            TrollType::Griefer => "GRIEFER",
            TrollType::Miner => "MINER",
            TrollType::Peasant => "PEASANT",
            TrollType::Poly => "POLY"
        })
    }
}

#[derive(Copy, Clone)]
struct Troll {
    pub id: i32,
    pub player: i32,
    pub x: i32,
    pub y: i32,
    pub charac: [i32; 4],
    pub class: TrollType,
    pub objective: usize,
    pub assigned: bool,
    pub items: [i32; 6]
}

impl Troll {

    // INSTANTIATION

    fn new
    (
        id: i32, player: i32, x: i32, y: i32,
        movement_speed: i32, carry_capacity: i32,
        harvest_power: i32, chop_power: i32,
        carry_plum: i32, carry_lemon: i32, carry_apple: i32,
        carry_banana: i32, carry_iron: i32, carry_wood: i32
    ) -> Troll {
        let charac = [movement_speed, carry_capacity, harvest_power, chop_power];
        let items = [carry_plum, carry_lemon, carry_apple, carry_banana, carry_iron, carry_wood];
        let class = TrollType::get_type(charac);
        Troll {
            id, player, x, y, charac, class, objective: class.default_objective(), assigned: false, items
        }
    }

    fn from(input_line: &String) -> Troll {
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Troll::new(
            parse_input!(inputs[0], i32),
            parse_input!(inputs[1], i32),
            parse_input!(inputs[2], i32),
            parse_input!(inputs[3], i32),
            parse_input!(inputs[4], i32),
            parse_input!(inputs[5], i32),
            parse_input!(inputs[6], i32),
            parse_input!(inputs[7], i32),
            parse_input!(inputs[8], i32),
            parse_input!(inputs[9], i32),
            parse_input!(inputs[10], i32),
            parse_input!(inputs[11], i32),
            parse_input!(inputs[12], i32),
            parse_input!(inputs[13], i32),
        )
    }

    fn from_stdin() -> Troll {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        Troll::from(&input_line)
    }

    // STATUS

    fn total_carry(&self) -> i32 {
        self.items.iter().sum()
    }

    fn is_full(&self) -> bool {
        self.charac[CAPA] == self.total_carry()
    }

    fn empty(&self) -> bool {
        self.total_carry() == 0
    }

    fn place_left(&self) -> i32 {
        self.charac[CAPA] - self.total_carry()
    }

    fn have_fruit(&self, fruit: Fruit) -> i32 {
        self.items[fruit.to_usize()]
    }

    fn have(&self, item: usize) -> bool {
        self.items[item] != 0
    }

    fn have_any_fruit(&self) -> bool {
        self.items
        .iter()
        .take(4)
        .any(|item| *item > 0)
    }

    fn dist_from_base(&self, grid: &Grid) -> i32 {
        grid.drop_positions
        .iter()
        .map(|&pos| manhattan_dist(pos, (self.x, self.y)))
        .min()
        .unwrap()
    }

    // POSSIBILITIES

    fn can_harvest(&self, grid: &Grid, trees: &Vec<Tree>) -> bool {
        match &grid.grid[self.y as usize][self.x as usize] {
            Tile::Grass(entities) => {
                entities.iter().any(|e| {
                    match e {
                        Entity::Tree(t) => {
                            t.nb_fruit > 0 && match self.objective {
                                ANY => true,
                                n  => {
                                    match trees
                                        .iter()
                                        .any(|tree| tree.fruit == Fruit::from_usize(n) && tree.nb_fruit > 0) {
                                            true => t.fruit == Fruit::from_usize(n),
                                            false => true
                                        }
                                }
                            }
                        }
                        _ => false
                    }
                })
            }
            _ => false
        }
    }

    fn can_chop(&self, grid: &Grid) -> bool {
        match &grid.grid[self.y as usize][self.x as usize] {
            Tile::Grass(entities) => {
                entities.iter().any(|e| {
                    match e {
                        Entity::Tree(t) => {
                            self.objective == CUT_ALL ||
                            t.size == 4 && match self.objective {
                                CHOP_ANY => true,
                                n  => t.fruit == Fruit::from_usize(n)
                            }
                        }
                        _ => false
                    }
                })
            }
            _ => false
        }
    }

    fn can_drop(&self, drop_positions: &Vec::<(i32, i32)>) -> bool {
        drop_positions.contains(&(self.x, self.y)) && self.total_carry() > 0
    }

    fn can_mine(&self, grid: &Grid) -> bool {
        grid.mine_positions.contains(&(self.x, self.y))
    }

    fn can_pick(&self, drop_positions: &Vec::<(i32, i32)>, inventory: &Inventory) -> bool {
        inventory.resources[self.objective - 7] > 0 &&
        drop_positions.contains(&(self.x, self.y)) &&
        !self.is_full()
    }

    fn can_pick_fruit(&self, drop_positions: &Vec::<(i32, i32)>, inventory: &Inventory, fruit: Fruit) -> bool {
        inventory.resources[fruit.to_usize()] > 0 &&
        drop_positions.contains(&(self.x, self.y)) &&
        !self.is_full()
    }

    fn can_plant(&self, grid: &Grid) -> bool {
        self.items
        .iter()
        .take(4)
        .any(|n| *n > 0) &&
        grid.grid[self.y as usize][self.x as usize].can_be_planted() &&
        match grid.nearest_empty_place_to_plant(3) {
            None => false,
            Some((x, y)) => manhattan_dist((self.x, self.y), grid.shack_position) <= 1 + manhattan_dist((x, y), grid.shack_position)
        }
    }

    fn can_grief(&self, grid: &Grid, trees: &Vec<Tree>) -> bool {
        match grid.tree_to_grief_whatever_dist(trees) {
            None => false,
            Some((x, y)) => {
                manhattan_dist((self.x, self.y), grid.enemy_shack_position) == manhattan_dist((x, y), grid.enemy_shack_position)
                && grid.is_on_tree(self)
            }
        }
    }

    // GOTOS

    fn go_to(&self, x: i32, y: i32, grid: &Grid, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> String {
        eprintln!("id: {} wants to go to {:?}", self.id, (x, y));
        // let (n_x, n_y) = match a_star(grid, (self.x, self.y), (x, y), self.charac[MS], troll_paths, false) {
        //     None => {
        //         eprintln!("no path found with collide");
        //         match a_star(grid, (self.x, self.y), (x, y), self.charac[MS], troll_paths, true) {
        //             None => (self.x, self.y),
        //             Some(v) => match v.len() {
        //                 0 => (self.x, self.y),
        //                 l => v[0]
        //             }
        //         } 
        //     },
        //     Some(v) => match v.len() {
        //         0 => (self.x, self.y),
        //         l => v[0]
        //     }
        // };
        // format!("MOVE {} {n_x} {n_y}", self.id)
        format!("MOVE {} {x} {y}", self.id)
    }

    fn go_to_best_pos(&self, positions: &Vec<(i32, i32)>, grid: &Grid, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> String {
        // let (mut n_x, mut n_y) = match positions
        // .iter()
        // .map(|&to| a_star(grid, (self.x, self.y), to, self.charac[MS], troll_paths, false))
        // .filter(|mapped| mapped.is_some())
        // .map(|filtered| filtered.unwrap())
        // .min_by(|v1, v2| v1.len().cmp(&v2.len())) {
        //     None => (self.x, self.y),
        //     Some(v) => match v.len() {
        //         0 => (self.x, self.y),
        //         l => v[0]
        //     }
        // };
        // if (n_x, n_y) == (self.x, self.y) {
        //     (n_x, n_y) = match positions
        //     .iter()
        //     .map(|&to| a_star(grid, (self.x, self.y), to, self.charac[MS], troll_paths, true))
        //     .filter(|mapped| mapped.is_some())
        //     .map(|filtered| filtered.unwrap())
        //     .min_by(|v1, v2| v1.len().cmp(&v2.len())) {
        //         None => (self.x, self.y),
        //         Some(v) => match v.len() {
        //             0 => (self.x, self.y),
        //             l => v[0]
        //         }
        //     };
        // }
        // format!("MOVE {} {n_x} {n_y}", self.id)
        let (x, y) = positions
        .iter()
        .min_by(|&&a, &&b| manhattan_dist(a, (self.x, self.y)).cmp(&manhattan_dist(b, (self.x, self.y))))
        .unwrap();
        format!("MOVE {} {x} {y}", self.id)
    }

    fn go_to_tree(&self, tree: &Tree, grid: &Grid, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> String {
        self.go_to(tree.x, tree.y, grid, troll_paths)
    }

    fn go_to_drop(&self, grid: &Grid, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        let available_drops = grid.drop_positions
        .iter()
        .filter(|&&(x, y)| grid.grid[y as usize][x as usize].can_walk_through())
        .map(|tamere| *tamere)
        .collect::<Vec<_>>();
        Some(self.go_to_best_pos(&available_drops, grid, troll_paths))
    }

    fn go_to_mine(&self, grid: &Grid, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        Some(self.go_to_best_pos(&grid.mine_positions, grid, troll_paths))
    }

    fn go_to_grief(&self, grid: &Grid, trees: &Vec<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        match grid.tree_to_grief_far_from_home(trees) {
            Some(pos) => Some(self.go_to(pos.0, pos.1, grid, troll_paths)),
            None => None
        }
    }

    // FINDS ALGO

    fn find_shorter_tree<'a>(&self, trees: &'a Vec::<Tree>) -> Option<(i32, &'a Tree)> {
        trees.iter()
        .filter(|tree| {
            tree.nb_fruit > 0
        })
        .map(|tree| {
            (i32::abs(self.x - tree.x) + i32::abs(self.y - tree.y), tree)
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
    }

    fn find_shorter_tree_to_chop<'a>(&self, trees: &'a Vec::<Tree>) -> Option<(i32, &'a Tree)> {
        trees.iter()
        .filter(|tree| {
            match tree.fruit {
                Fruit::Apple => false,
                _ => match tree.size {
                    4 => true,
                    _ => false
                }
            }
        })
        .map(|tree| {
            (i32::abs(self.x - tree.x) + i32::abs(self.y - tree.y), tree)
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
    }

    fn find_shorter_fruit<'a>(&self, trees: &'a Vec::<Tree>, fruit: Fruit) -> Option<(i32, &'a Tree)> {
        match trees.iter()
        .filter(|tree| {
            tree.fruit == fruit && tree.nb_fruit > 0
        })
        .map(|tree| {
            (i32::abs(self.x - tree.x) + i32::abs(self.y - tree.y), tree)
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        }) {
            None => self.find_shorter_tree(trees),
            Some(a) => Some(a)
        }
    }

    fn find_shorter_fruit_to_chop<'a>(&self, trees: &'a Vec::<Tree>, fruit: Fruit) -> Option<(i32, &'a Tree)> {
        trees.iter()
        .filter(|tree| {
            tree.fruit == fruit && tree.size == 4
        })
        .map(|tree| {
            (i32::abs(self.x - tree.x) + i32::abs(self.y - tree.y), tree)
        })
        .min_by(|a, b| {
            a.0.cmp(&b.0)
        })
    }

    // ACTIONS

    fn harvest(&self) -> String {
        format!("HARVEST {}", self.id)
    }

    fn mine(&self) -> String {
        format!("MINE {}", self.id)
    }

    fn drop(&self) -> String {
        format!("DROP {}", self.id)
    }

    fn pick(&self) -> String {
        format!("PICK {} {}", self.id, Fruit::from_usize(self.objective))
    }

    fn plant(&self) -> String {
        match self.have_fruit(Fruit::from_usize(self.objective)) {
            0 => format!("PLANT {} {}", self.id, Fruit::from_usize(self.items
            .iter()
                                                        .enumerate()
                                                        .find(|(_, n)| **n > 0)
                                                        .map(|(idx, _)| idx)
                                                        .unwrap())
            ),
            _ => format!("PLANT {} {}", self.id, Fruit::from_usize(self.objective))
        }
    }

    fn plant_fruit(&self, fruit: Fruit) -> String {
        format!("PLANT {} {fruit}", self.id)
    }

    fn chop(&self) -> String {
        format!("CHOP {}", self.id)
    }

    fn act_as_poly(&mut self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_poly", self.id);
        if self.is_full() && (self.items[4] > 0 || self.items[5] > 0) {
            if self.can_drop(&grid.drop_positions) {
                return Some(self.drop())
            }
            return self.go_to_drop(grid, troll_paths)
        }
        match self.objective {
            THIEF => self.act_as_thief(grid, trees, troll_paths),
            THIEF_NO_WASTE => self.act_as_thief_no_waste(grid, trees, troll_paths),
            BANANA_TRICKS => self.act_as_banana_tricks(grid, trees, inventory, troll_paths),
            IRON => self.act_as_miner(grid, trees, troll_paths),
            WOOD => self.act_as_choper(grid, trees, troll_paths),
            _ => self.act_as_peasant(grid, trees, inventory, troll_paths)
        }
    }

    fn act_as_choper(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_choper", self.id);
        if self.objective == THIEF_NO_WASTE {
            return self.act_as_thief_no_waste(grid, trees, troll_paths)
        }
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return self.go_to_drop(grid, troll_paths)
        }
        if self.can_chop(grid) {
            return Some(self.chop())
        }
        let best_tree = match self.objective {
            CUT_ALL => grid.find_shorter_available_tree_to_cut((self.x, self.y), trees, 1),
            CHOP_ANY => grid.find_shorter_available_tree_to_cut((self.x, self.y), trees, 4),
            fruit => grid.find_shorter_available_fruit_to_cut((self.x, self.y), trees, Fruit::from_usize(fruit), 4),
        };
        match best_tree {
            Some((_, tree)) => Some(self.go_to_tree(tree, grid, troll_paths)),
            None => None
        }
    }

    fn act_as_peasant(&mut self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_peasant", self.id);
        match self.objective {
            PLUM..=BANANA | ANY => self.act_as_harvester(grid, trees, troll_paths),
            PLANT_PLUM..=PLANT_ANY => self.act_as_planter(grid, trees, inventory, troll_paths),
            CUT_ALL => self.act_as_choper(grid, trees, troll_paths),
            _ => unreachable!("peasant cannot be asked for {}", self.objective)
        }
    }

    fn act_as_harvester(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_harvester", self.id);
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return self.go_to_drop(grid, troll_paths)
        }
        if self.can_harvest(grid, trees) {
            return Some(self.harvest())
        }
        let best_tree = match self.objective {
            ANY => self.find_shorter_tree(trees),
            fruit => self.find_shorter_fruit(trees, Fruit::from_usize(fruit))
        };
        match best_tree {
            Some((_, tree)) => Some(self.go_to_tree(tree, grid, troll_paths)),
            None => None
        }
    }

    fn act_as_planter(&mut self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_planter", self.id);
        match self.empty() {
            true => {
                if self.can_pick(&grid.drop_positions, inventory) {
                    return Some(self.pick())
                }
                if self.can_harvest(grid, trees) {
                    return Some(self.harvest())
                }
                let best_tree = self.find_shorter_fruit(trees, Fruit::from_usize(self.objective));
                match best_tree {
                    None => {
                        if inventory.resources[self.objective - 7] > 0 {
                            return self.go_to_drop(grid, troll_paths)
                        }
                    },
                    Some((dist, tree)) => {
                        if inventory.resources[self.objective - 7] > 0 && (dist > self.dist_from_base(grid)) {
                            return self.go_to_drop(grid, troll_paths)
                        }
                        return Some(self.go_to_tree(tree, grid, troll_paths))
                    }
                }
            },
            false => {
                if self.have_fruit(Fruit::from_usize(self.objective)) > 0 {
                    if self.can_plant(grid) {
                        return Some(self.plant())
                    }
                    match grid.nearest_empty_place_to_plant(4) {
                        None => {},
                        Some((x, y)) => return Some(self.go_to(x, y, grid, troll_paths))
                    }
                }
            }
        }

        // If the target is impossible, do whatever.
        self.target(ANY);
        return self.act_as_harvester(grid, trees, troll_paths);
    }

    fn act_as_miner(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_miner", self.id);
        match self.objective {
            CHOP_PLUM..=CUT_ALL => {
                return self.act_as_choper(grid, trees, troll_paths)
            },
            THIEF => return self.act_as_thief(grid, trees, troll_paths),
            THIEF_NO_WASTE => return self.act_as_thief_no_waste(grid, trees, troll_paths),
            _ => {   
                if self.can_drop(&grid.drop_positions) {
                    return Some(self.drop())
                }
                if self.is_full() {
                    return self.go_to_drop(grid, troll_paths)
                }
                if self.can_mine(grid) {
                    return Some(self.mine())
                }
                self.go_to_mine(grid, troll_paths)
            }
        }
    }

    fn act_as_griefer(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_griefer", self.id);
        if self.can_grief(grid, trees) {
            return Some(self.chop())
        }
        self.go_to_grief(grid, trees, troll_paths)
    }

    fn act_as_thief_no_waste(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_thief_no_waste", self.id);
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return self.go_to_drop(grid, troll_paths)
        }
        return self.act_as_griefer(grid, trees, troll_paths);
    }

    fn act_as_thief(&self, grid: &Grid, trees: &Vec::<Tree>, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_thief", self.id);
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() && manhattan_dist((self.x, self.y), grid.enemy_shack_position) > manhattan_dist((self.x, self.y), grid.shack_position) {
            return self.go_to_drop(grid, troll_paths)
        }
        self.act_as_griefer(grid, trees, troll_paths)
    }

    fn act_as_banana_tricks(&mut self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I act_as_banana_tricks", self.id);
        if self.is_full() {
            if self.have_fruit(Fruit::Banana) > 0 {
                return Some(self.plant_fruit(Fruit::Banana))
            }
            if self.can_drop(&grid.drop_positions) {
                return Some(self.drop())
            }
            return self.go_to_drop(grid, troll_paths)
        }
        self.target(CUT_ALL);
        if self.can_chop(grid) {
            return Some(self.chop())
        }
        if inventory.resources[BANANA] > 0 {
            if self.can_pick_fruit(&grid.drop_positions, inventory, Fruit::Banana) {
                self.target(BANANA);
                return Some(self.pick());
            }
            return self.go_to_drop(grid, troll_paths)
        }
        self.act_as_choper(grid, trees, troll_paths)
    }

    // ENTRYPOINTS

    fn target(&mut self, objective: usize) {
        self.objective = objective;
        self.assigned = true;
    }

    fn act(&mut self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory, troll_paths: &mut Vec<Vec<(i32, i32)>>) -> Option<String> {
        eprintln!("{}: I'm a {}", self.id, self.class);
        match self.class {
            TrollType::Poly => self.act_as_poly(grid, trees, inventory, troll_paths),
            TrollType::Choper => self.act_as_choper(grid, trees, troll_paths),
            TrollType::Griefer => self.act_as_griefer(grid, trees, troll_paths),
            TrollType::Miner => self.act_as_miner(grid, trees, troll_paths),
            TrollType::Peasant => self.act_as_peasant(grid, trees, inventory, troll_paths),
        }
        
    }
}

fn ask_for_resources(trolls: &mut Vec<Troll>, needs: [i32; 6]) {
    if needs[IRON] > 0 {
        trolls
        .iter_mut()
        .filter(|troll| match troll.class {
            TrollType::Poly => true,
            _ => false
        })
        .for_each(|troll| troll.target(IRON));
    }
    let mut peasants = trolls.iter_mut().filter(|troll| {
        match troll.class {
            TrollType::Peasant => true,
            TrollType::Poly => !troll.assigned,
            _ => false
        }
    });
    (0..3).for_each(|idx| {
        let mut nb_needs = needs[idx];
        while nb_needs > 0 {
            match peasants.next() {
                None => break,
                Some(troll) => {
                    troll.target(idx);
                    nb_needs -= troll.place_left();
                }
            }
        }
    })
}

// Inventory part

#[derive(Copy, Clone)]
struct Inventory {
    pub resources: [i32; 6],
}

impl Inventory {

    // INSTANTIATION

    fn new(plum: i32, lemon: i32, apple: i32, banana: i32, iron: i32, wood: i32) -> Inventory {
        Inventory {resources: [plum, lemon, apple, banana, iron, wood]}
    }

    fn from(input_line: &String) -> Inventory {
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        Inventory::new(
            parse_input!(inputs[0], i32),
            parse_input!(inputs[1], i32),
            parse_input!(inputs[2], i32),
            parse_input!(inputs[3], i32),
            parse_input!(inputs[4], i32),
            parse_input!(inputs[5], i32),
        )
    }

    fn from_stdin() -> Inventory {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        Inventory::from(&input_line)
    }

    // POSSIBILITIES

    fn what_is_needed(&self, target: [i32; 6]) -> Option<[i32; 6]> {
        let needs = (0..6).fold([0, 0, 0, 0, 0, 0], |mut acc, idx| {
            acc[idx] = i32::max(0, target[idx] - self.resources[idx]);
            acc
        });
        match needs.iter().all(|n| *n == 0) {
            true => None,
            false => Some(needs)
        }
    }

    // ORDER

    fn train_asked(&self, charac: [i32; 4], cost: [i32; 6], trolls: &mut Vec<Troll>) -> Option<String> {
        eprintln!("charac: {:?}, cost: {:?}", charac, cost);
        match self.what_is_needed(cost) {
            None => Some(format!("TRAIN {} {} {} {}", charac[MS], charac[CAPA], charac[HARV], charac[CHOP])),
            Some(needs) => {
                ask_for_resources(trolls, needs);
                None
            }
        }
    }

}

// Grid part

#[derive(Copy, Clone)]
enum Entity {
    Tree(Tree),
    Troll(Troll),
    EnemyTroll(Troll)
}

impl Entity {
    fn from_troll(troll: Troll) -> Entity {
        if troll.player == 0 {
            return Entity::Troll(troll)
        }
        Entity::EnemyTroll(troll)
    }

    fn from_tree(tree: Tree) -> Entity {
        Entity::Tree(tree)
    }
}

#[derive(Clone)]
enum Tile {
    Grass(Vec::<Entity>),
    Shack(Vec::<Entity>),
    EnemyShack(Vec::<Entity>),
    Water,
    Rock,
    Iron
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Grass(vec![]),
            '0' => Tile::Shack(vec![]),
            '1' => Tile::EnemyShack(vec![]),
            '~' => Tile::Water,
            '#' => Tile::Rock,
            '+' => Tile::Iron,
            _ => unreachable!("unknown tile: {c}")
        }
    }

    fn push(&mut self, e: Entity) {
        match self {
            Tile::EnemyShack(v) => v.push(e),
            Tile::Shack(v) => v.push(e),
            Tile::Grass(v) => v.push(e),
            _ => unreachable!("This tile cannot accept entities")
        }
    }

    fn get_tree(&self) -> Option<&Tree> {
        match self {
            Tile::Grass(entities) => {
                match entities
                .iter()
                .find(|entity| match entity {
                    Entity::Tree(tree) => true,
                    _ => false
                }) {
                    Some(Entity::Tree(tree)) => Some(tree),
                    _ => None
                }
            }
            _ => None
        }
    }

    fn is_a_tree(&self) -> bool {
        match self {
            Tile::Grass(entities) => {
                entities
                .iter()
                .any(|entity| match entity {
                    Entity::Tree(..) => true,
                    _ => false
                })
            },
            _ => false
        }
    }

    fn can_be_planted(&self) -> bool {
        match self {
            Tile::Grass(_) => match self.get_tree() {
                None => true,
                _ => false
            },
            _ => false
        }
    }

    fn can_walk_through(&self) -> bool {
        match self {
            Tile::Grass(entities) => match entities
                .iter()
                .any(|entity| match entity {
                    Entity::Troll(troll) => troll.player == 0,
                    _ => false
                }) {
                    true => false,
                    false => true
                }
            _ => false
        }
    }

    fn is_grass(&self) -> bool {
        match self {
            Tile::Grass(..) => true,
            _ => false
        }
    }
}

#[derive(Clone)]
struct Grid {
    pub initial_grid: Vec::<Vec::<Tile>>,
    pub grid: Vec::<Vec::<Tile>>,
    pub shack_position: (i32, i32),
    pub enemy_shack_position: (i32, i32),
    pub drop_positions: Vec::<(i32, i32)>,
    pub mine_positions: Vec::<(i32, i32)>,
    pub mine_dist: i32,
    pub width: i32,
    pub height: i32
}

impl Grid {
    fn new() -> Grid {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let width = parse_input!(inputs[0], i32);
        let height = parse_input!(inputs[1], i32);
        let mut raw_mine_side_positions: Vec::<(i32, i32)> = vec![];
        let mut shack_position: (i32, i32) = (0, 0);
        let mut enemy_shack_position: (i32, i32) = (0, 0);
        let mut raw_drop_positions = vec![];
        let grid =
            (0..height)
            .fold(Vec::<Vec::<Tile>>::new(), |mut acc_outer, _| {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let line = input_line.trim_matches('\n').to_string();
                acc_outer.push(line.chars().fold(Vec::<Tile>::new(), |mut acc_inner, c| {
                    if c == '0' {
                        let pos_x = acc_inner.len() as i32;
                        let pos_y = acc_outer.len() as i32;
                        shack_position = (pos_x, pos_y);
                        safe_push_positions(
                            &vec![(pos_x + 1, pos_y), (pos_x - 1, pos_y), (pos_x, pos_y + 1), (pos_x, pos_y - 1)],
                            &mut raw_drop_positions,
                            height,
                            width
                        );
                    }
                    if c == '1' {
                        enemy_shack_position = (acc_inner.len() as i32, acc_outer.len() as i32)
                    }
                    if c == '+' {
                        let pos_x = acc_inner.len() as i32;
                        let pos_y = acc_outer.len() as i32;
                        safe_push_positions(
                            &vec![(pos_x + 1, pos_y), (pos_x - 1, pos_y), (pos_x, pos_y + 1), (pos_x, pos_y - 1)],
                            &mut raw_mine_side_positions,
                            height,
                            width
                        );
                    }
                    acc_inner.push(Tile::from_char(c));
                    acc_inner
                }));
                acc_outer
            });
        let drop_positions = raw_drop_positions
        .iter()
        .filter(|(x, y)| {
            match grid[*y as usize][*x as usize] {
                Tile::Grass(_) => true,
                _ => false
            }
        })
        .map(|a| *a)
        .collect::<Vec<(i32, i32)>>();

        let mine_side_positions = raw_mine_side_positions
        .iter()
        .filter(|(x, y)| {
            match grid[*y as usize][*x as usize] {
                Tile::Grass(_) => true,
                _ => false
            }
        })
        .map(|a| *a)
        .collect::<Vec<(i32, i32)>>();

        Grid {
            initial_grid: grid.clone(),
            grid: grid.clone(),
            shack_position,
            enemy_shack_position,
            drop_positions: drop_positions.clone(),
            mine_positions: mine_side_positions.clone(),
            mine_dist: 0,
            width,
            height
        }
    }

    fn set_mine_distance(&mut self) {
        self.mine_dist = self.mine_positions
        .iter()
        .map(|&mine_side_position|
            match a_star(self, self.shack_position, mine_side_position, 1, &mut vec![], true) {
                None => None,
                Some(v) => Some(v.len() - 1)
            }
        )
        .filter(|mapped| mapped.is_some())
        .map(|filtered| filtered.unwrap())
        .min()
        .unwrap() as i32;
    }

    fn update_tiles(&mut self, trolls: &Vec::<Troll>, e_trolls: &Vec::<Troll>, trees: &Vec::<Tree>) {
        self.grid = self.initial_grid.clone();
        for troll in trolls {
            self.grid[troll.y as usize][troll.x as usize].push(Entity::from_troll(*troll))
        }
        for troll in e_trolls {
            self.grid[troll.y as usize][troll.x as usize].push(Entity::from_troll(*troll))
        }
        for tree in trees {
            self.grid[tree.y as usize][tree.x as usize].push(Entity::from_tree(*tree))
        }
    }

    fn get_shack_area_iter(&self, area: i32) -> impl Iterator<Item = (i32, i32)> {
        (i32::max(0, self.shack_position.0 - area)..i32::min(self.width as i32, self.shack_position.0 + area + 1))
        .cartesian_product(i32::max(0, self.shack_position.1 - area)..i32::min(self.height as i32, self.shack_position.1 + area + 1))
        .filter(move |pos| manhattan_dist(*pos, self.shack_position) <= area)
    }

    fn tree_census(&self, area: i32) -> [i32; 4] {
        self.get_shack_area_iter(area)
        .fold([0, 0, 0, 0], |mut acc, (x, y)| {
            match &self.grid[y as usize][x as usize] {
                Tile::Grass(entities) => {
                    match entities.iter().find(|&entity| {
                        match entity {
                            Entity::Tree(_) => true,
                            _ => false
                        }
                    }) {
                        Some(Entity::Tree(tree)) => {
                            acc[tree.fruit.to_usize()] += 1;
                        }
                        _ => {}
                    }
                },
                _ => {}
            };
            acc
        })
    }

    fn plant_need(&self, area: i32, required: [i32; 4]) -> Option<usize> {
        let census = self.tree_census(area);
        eprintln!("Tree census: {:?}", census);
        match census
        .iter()
        .zip(required)
        .enumerate()
        .map(|(fruit, (c, r))| (fruit, r - *c))
        .filter(|&(_, n)| n > 0)
        .max() {
            None => None,
            Some((fruit, _)) => Some(fruit)
        }
    }

    fn ask_for_plant(&self, area: i32, required: [i32; 4], trolls: &mut Vec<Troll>) {
        match self.plant_need(area, required) {
            None => {},
            Some(fruit) => {
                match trolls
                    .iter_mut()
                    .filter(|troll| troll.class == TrollType::Peasant || (troll.class == TrollType::Poly && !troll.assigned))
                    .max_by(|a, b| {
                        a.items[fruit].cmp(&b.items[fruit])
                    }) {
                        None => {},
                        Some(troll) => {
                            troll.target(fruit + 7)   
                        }
                    }
            }
        }
    }

    fn assign_plant_bananas(&self, area: i32, required: i32, trolls: &mut Vec<Troll>) {
        let banana_count = self.tree_census(area)[3];
        let needs = required - banana_count;
        
        if needs > 0 {
            trolls
            .iter_mut()
            .filter(|troll| troll.have_fruit(Fruit::Banana) > 0)
            .for_each(|troll| troll.target(PLANT_BANANA));

            let bananas_carried: i32 = trolls
            .iter()
            .filter(|troll| troll.have_fruit(Fruit::Banana) > 0)
            .map(|troll| troll.have_fruit(Fruit::Banana))
            .sum();
            let bananas_to_plant = needs - bananas_carried;
            
            let mut trolls_available_iter = trolls
            .iter_mut()
            .filter(|troll| (troll.class == TrollType::Poly || troll.class == TrollType::Peasant) && troll.empty());

            while bananas_to_plant > 0 {
                match trolls_available_iter.next() {
                    None => break,
                    Some(troll) => troll.target(PLANT_BANANA)
                }
            }
        }
    }

    fn plants_need(&self, area: i32, required: [i32; 4]) -> [i32; 4] {
        self.tree_census(area)
        .iter()
        .zip(required)
        .enumerate()
        .fold([0, 0, 0, 0], |mut acc, (idx, (c, r))| {
            acc[idx] = i32::max(0, r - *c);
            acc
        })
    }

    fn assign_plants(&self, area: i32, required: [i32; 4], trolls: &mut Vec<Troll>) -> bool {
        let needs = self.plants_need(area, required);
        eprintln!("tree needed: {:?}", needs);
        if needs.iter().all(|n| *n == 0) {
            return false;
        }

        needs
        .iter()
        .enumerate()
        .for_each(|(fruit_idx, nb_to_plant)| {
            let this_fruit_carried: i32 = trolls
            .iter_mut()
            .filter(|troll| troll.have_fruit(Fruit::from_usize(fruit_idx)) > 0)
            .map(|troll| {
                troll.target(fruit_idx + 7);
                troll.have_fruit(Fruit::from_usize(fruit_idx))
            })
            .sum();

            let mut trolls_available_iter = trolls
            .iter_mut()
            .filter(|troll| troll.empty());

            let mut fruit_to_pick = nb_to_plant - this_fruit_carried;
            eprintln!("{} to pick: {fruit_to_pick}", Fruit::from_usize(fruit_idx));
            while fruit_to_pick > 0 {
                match trolls_available_iter.next() {
                    None => break,
                    Some(troll) => {
                        troll.target(fruit_idx + 7);
                        fruit_to_pick -= 1;
                    }
                }
            }
        });

        true
    }

    fn is_on_tree(&self, troll: &Troll) -> bool {
        self.grid[troll.y as usize][troll.x as usize].is_a_tree()
    }

    fn nearest_empty_place_to_plant(&self, area: i32) -> Option<(i32, i32)> {
        self.get_shack_area_iter(area)
        .filter(|&(x, y)| self.grid[y as usize][x as usize].can_be_planted())
        .filter(|&(x, y)| self.grid[y as usize][x as usize].can_walk_through())
        .min_by(|a, b| manhattan_dist(*a, self.shack_position).cmp(&manhattan_dist(*b, self.shack_position)))
    }

    fn tree_to_grief_far_from_home(&self, trees: &Vec<Tree>) -> Option<(i32, i32)> {
        trees
        .iter()
        .filter(|tree| self.grid[tree.y as usize][tree.x as usize].can_walk_through())
        .filter(|tree| manhattan_dist((tree.x, tree.y), self.shack_position) > 3)
        .map(|tree| (tree.x, tree.y))
        .min_by(|a, b| manhattan_dist(*a, self.enemy_shack_position).cmp(&manhattan_dist(*b, self.enemy_shack_position)))
    }

    fn tree_to_grief_ignore_walk_through(&self, trees: &Vec<Tree>) -> Option<(i32, i32)> {
        trees
        .iter()
        .map(|tree| (tree.x, tree.y))
        .min_by(|a, b| manhattan_dist(*a, self.enemy_shack_position).cmp(&manhattan_dist(*b, self.enemy_shack_position)))
    }

    fn tree_to_grief_whatever_dist(&self, trees: &Vec<Tree>) -> Option<(i32, i32)> {
        trees
        .iter()
        .map(|tree| (tree.x, tree.y))
        .min_by(|a, b| manhattan_dist(*a, self.enemy_shack_position).cmp(&manhattan_dist(*b, self.enemy_shack_position)))
    }

    fn find_shorter_available_tree_to_cut<'a>(&self, pos: (i32, i32), trees: &'a Vec<Tree>, min_size: i32) -> Option<(i32, &'a Tree)> {
        trees
        .iter()
        .filter(move |tree| tree.size >= min_size && self.grid[tree.y as usize][tree.x as usize].can_walk_through())
        .map(|tree| (manhattan_dist((tree.x, tree.y), pos), tree))
        .min_by(|a, b| a.0.cmp(&b.0))
    }

    fn find_shorter_available_fruit_to_cut<'a>(&self, pos: (i32, i32), trees: &'a Vec<Tree>, fruit: Fruit, min_size: i32) -> Option<(i32, &'a Tree)> {
        trees
        .iter()
        .filter(move |tree| tree.fruit == fruit && tree.size >= min_size && self.grid[tree.y as usize][tree.x as usize].can_walk_through())
        .map(|tree| (manhattan_dist((tree.x, tree.y), pos), tree))
        .min_by(|a, b| a.0.cmp(&b.0))
    }

}

fn parse_loop() -> (
    Inventory,
    Inventory,
    Vec::<Troll>,
    Vec::<Troll>,
    Vec::<Tree>
) {
    let inventory = Inventory::from_stdin();
    let e_inventory = Inventory::from_stdin();
    let trees = Tree::from_stdin_all();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let trolls_count = parse_input!(input_line, i32);
    let (mut trolls, mut enemy_trolls) =
        (0..trolls_count)
        .fold((Vec::<Troll>::new(), Vec::<Troll>::new()), |(mut t, mut e_t), _| {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let troll = Troll::from(&input_line);
            if troll.player == 0 {
                t.push(troll);
            }
            else {
                e_t.push(troll);
            }
            (t, e_t)
        });
    trolls.sort_by(|a, b| a.id.cmp(&b.id));
    enemy_trolls.sort_by(|a, b| a.id.cmp(&b.id));
    (inventory, e_inventory, trolls, enemy_trolls, trees)
}

fn what_to_train(ratio: [i32; 5], trolls: &Vec::<Troll>) -> Option<TrollType> {
    let got = trolls
    .iter()
    .fold([0, 0, 0, 0, 0], |mut acc, troll| {
        acc[troll.class.to_usize()] += 1;
        acc
    });

    let ratio_sum = ratio.iter().sum::<i32>() as f32;
    let got_sum = got.iter().sum::<i32>() as f32;

    eprintln!("got {:?}, sum: {got_sum}", got);

    if got_sum == 0.0 {
        return Some(TrollType::Poly)
    }

    match got.iter().zip(ratio).enumerate()
    .filter(|(_, (_, r))| *r > 0)
    .map(|(troll_type, (g, r))| {
        (troll_type, (*g as f32 * ratio_sum) / (r as f32 * got_sum))
    })
    .min_by(|a, b| {
        match a.1.lt(&b.1) {
            true => Ordering::Less,
            false => {
                match b.1.lt(&a.1) {
                    true => Ordering::Greater,
                    false => Ordering::Equal
                }
            }
        }
    }) {
        None => None,
        Some((troll_type, _)) => Some(TrollType::from_usize(troll_type))
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GameStateNear {
    Start(bool),
    End
}

impl From<GameStateNear> for i32 {
    fn from(game_state: GameStateNear) -> i32 {
        match game_state {
            GameStateNear::Start(..) => 0,
            GameStateNear::End => 1,
        }
    }
}

impl From<i32> for GameStateNear {
    fn from(n: i32) -> GameStateNear {
        match n {
            0 => GameStateNear::Start(false),
            1 => GameStateNear::End,
            _ => unreachable!("unknown GameStateNear index: {n}")
        }
    }
}

impl GameStateNear {

    fn upgrade(&mut self) {
        *self = (Into::<i32>::into(*self) + 1).into()
    }

    fn update(&mut self) {
        match self {
            GameStateNear::Start(trained_1) => {
                if *trained_1 {
                    self.upgrade();
                }
            },
            _ => {}
        }
    }
}

fn near_strategy(inventory: &Inventory, game_state: &mut GameStateNear, round: i32, grid: &Grid, trolls: &mut Vec<Troll>, trees: &Vec<Tree>, actions: &mut Vec<String>) {

    game_state.update();

    match game_state {
        GameStateNear::Start(one) => {
            let train: [i32; 4] = TrollType::best_possible_train(inventory, 1);
            actions.push(format!("TRAIN {} {} {} {}", train[0], train[1], train[2], train[3]));
            *one = true;
        },
        GameStateNear::End => {
            trolls
            .iter_mut()
            .for_each(|troll| match troll.class {
                TrollType::Choper => troll.target(THIEF_NO_WASTE),
                _ => troll.target(BANANA_TRICKS)
            });
        }
    }

}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GameStateDistant {
    Start(bool, bool),
    Early(bool, bool),
    Mid(bool),
    End,
    CutAll
}

impl From<GameStateDistant> for i32 {
    fn from(game_state: GameStateDistant) -> i32 {
        match game_state {
            GameStateDistant::Start(..) => 0,
            GameStateDistant::Early(..) => 1,
            GameStateDistant::Mid(..) => 2,
            GameStateDistant::End => 3,
            GameStateDistant::CutAll => 4,
        }
    }
}

impl From<i32> for GameStateDistant {
    fn from(n: i32) -> GameStateDistant {
        match n {
            0 => GameStateDistant::Start(false, false),
            1 => GameStateDistant::Early(false, false),
            2 => GameStateDistant::Mid(false),
            3 => GameStateDistant::End,
            4 => GameStateDistant::CutAll,
            _ => unreachable!("unknown GameStateDistant index: {n}")
        }
    }
}

impl GameStateDistant {

    fn upgrade(&mut self) {
        *self = (Into::<i32>::into(*self) + 1).into()
    }

    fn update(&mut self) {
        match self {
            GameStateDistant::Start(trained, trees_planted) => {
                if *trained && *trees_planted {
                    self.upgrade();
                }
            },
            GameStateDistant::Early(trained_1, trained_2) => {
                if *trained_1 && *trained_2 {
                    self.upgrade();
                }
            },
            GameStateDistant::Mid(trained) => {
                if *trained {
                    self.upgrade();
                }
            },
            _ => {}
        }
    }
}

fn distant_strategy(inventory: &Inventory, game_state: &mut GameStateDistant, round: i32, grid: &Grid, trolls: &mut Vec<Troll>, trees: &Vec<Tree>, actions: &mut Vec<String>) {
    if round == 280 {
        *game_state = GameStateDistant::CutAll;
    }

    game_state.update();

    let game_state_i32: i32 = (*game_state).into();

    match game_state {
        GameStateDistant::Start(peasant_trained, trees_to_plant) => {
            if !*peasant_trained {
                let train = TrollType::best_possible_train(inventory, 1);
                actions.push(format!("TRAIN {} {} {} {}", train[0], train[1], train[2], train[3]));
                *peasant_trained = true;
            }
            else if !grid.assign_plants(4, [2, 2, 2, 1], trolls) {
                *trees_to_plant = true;
            };
        },
        GameStateDistant::Early(miner_trained, griefer_trained) => {
            if manhattan_dist(grid.shack_position, grid.enemy_shack_position) < 7 {
                *griefer_trained = true;
            }
            let to_train = match *miner_trained {
                false => TrollType::Miner,
                _ => TrollType::Griefer
            };
            match inventory.train_asked(
                to_train.charac_need(
                    game_state_i32,
                    grid.mine_dist),
                to_train.cost(
                    game_state_i32,
                    grid.mine_dist,
                    trolls.len() as i32
                ),
                trolls
            ) {
                Some(cmd) => {
                    actions.push(cmd);
                    *miner_trained = true;
                    if to_train == TrollType::Griefer {
                        *griefer_trained = true;
                    }
                },
                None => {}
            };
            grid.assign_plants(4, [2, 2, 2, 1], trolls);
        },
        GameStateDistant::Mid(choper_trained) => {
            let to_train = TrollType::Choper;
            let cost = to_train.cost(
                game_state_i32,
                grid.mine_dist,
                trolls.len() as i32
            );
            let irons_need = cost[IRON];
            if inventory.resources[IRON] >= irons_need {
                trolls
                .iter_mut()
                .filter(|troll| troll.class == TrollType::Miner)
                .for_each(|troll| troll.target(THIEF));
            }
            match inventory.train_asked(
                to_train.charac_need(
                    game_state_i32,
                    grid.mine_dist),
                cost,
                trolls
            ) {
                Some(cmd) => {
                    actions.push(cmd);
                    *choper_trained = true;
                },
                None => {}
            };
            grid.assign_plant_bananas(4, 3, trolls);
            grid.assign_plants(4, [2, 2, 2, 0], trolls);
        },
        GameStateDistant::End => {
            trolls
            .iter_mut()
            .for_each(|troll| match troll.class {
                TrollType::Poly => troll.target(THIEF),
                TrollType::Miner => troll.target(THIEF),
                _ => {}
            });
        },
        GameStateDistant::CutAll => {
            trolls
            .iter_mut()
            .for_each(|troll|  troll.target(CUT_ALL));
        }
    };
}

fn main() {
    let mut grid = Grid::new();
    grid.set_mine_distance();
    let map_type = match manhattan_dist(grid.shack_position, grid.enemy_shack_position) {
        n if n > 6 => {
            match grid.drop_positions.len() {
                n if n <= 2 => NEAR,
                _ => DISTANT
            }
        },
        _ => NEAR
    };
    let mut game_state_distant: GameStateDistant = 0.into();
    let mut game_state_near: GameStateNear = 0.into();
    let mut round = 0;

    // game loop
    loop {
        round += 1;
        let mut actions = vec![];
        let mut troll_paths: Vec<Vec<(i32, i32)>> = vec![];

        let (
            inventory, e_inventory, mut trolls, mut enemy_trolls, mut trees
        ) = parse_loop();
        grid.update_tiles(&trolls, &enemy_trolls, &trees);

        match map_type {
            DISTANT => distant_strategy(&inventory, &mut game_state_distant, round, &grid, &mut trolls, &trees, &mut actions),
            NEAR => near_strategy(&inventory, &mut game_state_near, round, &grid, &mut trolls, &trees, &mut actions),
            _ => unreachable!("unknown map type {map_type}")
        };

        // TROLLS ACTIONS

        for mut troll in trolls {
            match troll.act(&grid, &trees, &inventory, &mut troll_paths) {
                Some(a) => actions.push(a),
                _ => {}
            }
        }

        // APPLY ACTIONS

        let actions_to_string = match actions.is_empty() {
            true => String::from("WAIT"),
            false => actions.join(";")
        };

        // valid actions:
        // MOVE <id> <x> <y>
        // HARVEST <id> - when you are on the same cell as a tree
        // DROP <id> - when you are next to your shack and carry items
        println!("{actions_to_string}");
    }
}
