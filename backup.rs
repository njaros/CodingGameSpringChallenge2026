use std::{fmt, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// TROLL CHARACTERISTICS CONSTANTS TO GET CHARACTERISTICS ARRAY INDEXES

const MS: usize = 0;
const CAPA: usize = 1;
const HARV: usize = 2;
const CHOP: usize = 3;

// GLOBAL GAME STATES

const EARLY: i32 = 0;
const MID: i32 = 1;
const END: i32 = 2;

// COST INDEXES or OBJECTIVES

const PLUM: usize = 0;
const LEMON: usize = 1;
const APPLE: usize = 2;
const IRON: usize = 3;
const WOOD: usize = 4;
const ANY: usize = 5;
const BANANA: usize = 6;

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
            PLUM => Fruit::Plum,
            LEMON => Fruit::Lemon,
            APPLE => Fruit::Apple,
            _ => unreachable!("Unknowm fruit_as_usize {fruit_as_usize}")
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
    EnemyChoper,
    // Troll having chopPower and carryCapacity
    Choper,
    // Troll only having movementSpeed and carryCapacity
    Planter,
    // Troll having harvestPower and carryCapacity
    Harvester,
    // Troll having chopPower and carryCapacity but his
    // carryCapacity and movementSpeed according to the iron mine distance
    Miner
}

impl TrollType {
    fn get_type(charac: [i32; 4]) -> TrollType {
        if charac.iter().all(|c| *c > 0) {
            return TrollType::Poly
        }
        if charac[CAPA] > 0 {
            if charac[CHOP] > 2 {
                return TrollType::Miner
            }
            if charac[CHOP] > 0 {
                return TrollType::Choper
            }
            if charac[HARV] > 0 {
                return TrollType::Harvester
            }
            return TrollType::Planter
        }
        return TrollType::EnemyChoper
    }

    fn charac_need(&self, game_state: i32, mine_dist: i32) -> [i32; 4] {
        match self {
            TrollType::Poly => [1 + game_state / 2, 1 + game_state / 2, 1 + game_state / 2, 1 + game_state / 2],
            TrollType::Choper => [2, 4, 0, 1],
            TrollType::Miner => [
                    1 + i32::min(1 + game_state, mine_dist / 2),
                    1 + i32::min(2 + game_state, mine_dist / 2),
                    0,
                    2 + game_state / 2
                ],
            TrollType::Planter => [1, 1, 0, 0],
            TrollType::EnemyChoper => [2 + game_state, 0, 0, 1 + game_state],
            TrollType::Harvester => [1 + game_state, 1 + game_state, 1 + game_state, 0]
        }
    }

    fn cost(&self, game_state: i32, mine_dist: i32, total_troll: i32) -> [i32; 4] {
        let charac = self.charac_need(game_state, mine_dist);
        [
            total_troll + charac[MS] * charac[MS],
            total_troll + charac[CAPA] * charac[CAPA],
            total_troll + charac[HARV] * charac[HARV],
            total_troll + charac[CHOP] * charac[CHOP]
        ]
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
    pub carry_plum: i32,
    pub carry_lemon: i32,
    pub carry_apple: i32,
    pub carry_banana: i32,
    pub carry_iron: i32,
    pub carry_wood: i32
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
        let class = TrollType::get_type(charac);
        Troll {
            id, player, x, y, charac, class, objective: ANY, assigned: false, carry_plum, carry_lemon, carry_apple, carry_banana, carry_iron, carry_wood
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
        self.carry_apple + self.carry_banana + self.carry_iron + self.carry_lemon + self.carry_plum + self.carry_wood
    }

    fn is_full(&self) -> bool {
        self.charac[CAPA] == self.total_carry()
    }

    fn place_left(&self) -> i32 {
        self.charac[CAPA] - self.total_carry()
    }

    fn carry_vegetable(&self) -> bool {
        self.carry_apple > 0 || self.carry_plum > 0 || self.carry_lemon > 0 || self.carry_banana > 0
    }

    // POSSIBILITIES

    fn can_harvest(&self, grid: &Grid) -> bool {
        match &grid.grid[self.y as usize][self.x as usize] {
            Tile::Grass(entities) => {
                entities.iter().any(|e| {
                    match e {
                        Entity::Tree(t) => t.nb_fruit > 0,
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

    fn can_mine(&self, mine_position: (i32, i32)) -> bool {
        (self.x, self.y) == mine_position
    }

    fn can_pick_vegetable(&self, inventory: &Inventory) -> bool {
        !self.is_full() && (inventory.apple > 0 || inventory.lemon > 0 || inventory.plum > 0) {

        }
    }

    fn can_plant(&self, grid: &Grid) -> bool {

    }

    // GOTOS

    fn go_to(&self, x: i32, y: i32) -> String {
        format!("MOVE {} {x} {y}", self.id)
    }

    fn go_to_tree(&self, tree: &Tree) -> String {
        self.go_to(tree.x, tree.y)
    }

    fn go_to_drop(&self, drop_pos: &Vec::<(i32, i32)>) -> String {
        let pos = drop_pos
            .iter()
            .min_by(|&a, &b| {
                (i32::abs(a.0 - self.x) + i32::abs(a.1 - self.y)).cmp(
                    &(i32::abs(b.0 - self.x) + i32::abs(b.1 - self.y))
                )
            }).unwrap();
        self.go_to(pos.0, pos.1)
    }

    // FINDS ALGO

    fn find_shorter_tree<'a>(&self, trees: &'a Vec::<Tree>) -> Option<(i32, &'a Tree)> {
        trees.iter()
        .filter(|tree| {
            match tree.fruit {
                Fruit::Banana => false,
                _ => tree.nb_fruit > 0
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
        trees.iter()
        .filter(|tree| {
            tree.fruit == fruit && tree.nb_fruit > 0
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

    fn act_as_poly(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        match self.objective {
            IRON => self.act_as_miner(grid, trees),
            WOOD => self.act_as_choper(grid, trees),
            _ => self.act_as_harvester(grid, trees)
        }
    }

    fn act_as_choper(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return Some(self.go_to_drop(&grid.drop_positions))
        }
        None
    }

    fn act_as_harvester(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return Some(self.go_to_drop(&grid.drop_positions))
        }
        if self.can_harvest(grid) {
            return Some(self.harvest())
        }
        let best_tree = match self.objective {
            ANY => self.find_shorter_tree(trees),
            n => self.find_shorter_fruit(trees, Fruit::from_usize(n))
        };
        match best_tree {
            Some((_, tree)) => Some(self.go_to_tree(tree)),
            None => None
        }
    }

    fn act_as_miner(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return Some(self.go_to_drop(&grid.drop_positions))
        }
        if self.can_mine(grid.mine_position) {
            return Some(self.mine())
        }
        return Some(self.go_to(grid.mine_position.0, grid.mine_position.1))
    }

    fn act_as_enemy_choper(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        None
    }

    fn act_as_planter(&self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory) -> Option<String> {
        if self.carry_vegetable() {

        }
        if self.can_pick() {

        }
        else {
            return Some(self.go_to_drop(&grid.drop_positions));
        }
        None
    }

    // ENTRYPOINTS

    fn target(&mut self, objective: usize) {
        self.objective = objective;
        self.assigned = true;
    }

    fn act(&self, grid: &Grid, trees: &Vec::<Tree>, inventory: &Inventory) -> Option<String> {
        match self.class {
            TrollType::Poly => self.act_as_poly(grid, trees),
            TrollType::Choper => self.act_as_choper(grid, trees),
            TrollType::EnemyChoper => self.act_as_enemy_choper(grid, trees),
            TrollType::Miner => self.act_as_miner(grid, trees),
            TrollType::Planter => self.act_as_planter(grid, trees, inventory),
            TrollType::Harvester => self.act_as_harvester(grid, trees)
        }
        
    }
}

fn ask_for_resources(trolls: &mut Vec<Troll>, needs: [i32; 4]) {
    if needs[IRON] > 0 {
        trolls
        .iter()
        .filter(|&troll| match troll.class {
            TrollType::Poly => true,
            _ => false
        })
        .for_each(|troll| troll.target(IRON));
    }
    let mut harvesters = trolls.iter().filter(|&troll| {
        match troll.class {
            TrollType::Harvester => true,
            TrollType::Poly => !troll.assigned,
            _ => false
        }
    });
    (0..2).for_each(|idx| {
        let mut nb_needs = needs[idx];
        while nb_needs > 0 {
            match harvesters.next() {
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
    pub plum: i32,
    pub lemon: i32,
    pub apple: i32,
    pub banana: i32,
    pub iron: i32,
    pub wood: i32,
    pub resources: [i32; 4],
}

impl Inventory {

    // INSTANTIATION

    fn new(plum: i32, lemon: i32, apple: i32, banana: i32, iron: i32, wood: i32) -> Inventory {
        let resources = [plum, lemon, apple, iron];
        Inventory {
            plum, lemon, apple, banana, iron, wood, resources
        }
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

    fn what_is_needed(&self, target: [i32; 4]) -> Option<[i32; 4]> {
        let needs = (0..4).map(
            |idx| i32::min(0, target[idx] - self.resources[idx])
        ).collect::<Vec>();
        match needs.iter().all(|n| n == 0) {
            true => None,
            false => Some(needs)
        }
    }

    // ORDER

    fn train_asked(&self, charac: [i32; 4], cost: [i32; 4], trolls: &mut Vec<Troll>) -> Option<String> {
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
}

#[derive(Clone)]
struct Grid {
    pub initial_grid: Vec::<Vec::<Tile>>,
    pub grid: Vec::<Vec::<Tile>>,
    pub shack_position: (i32, i32),
    pub drop_positions: Vec::<(i32, i32)>,
    pub mine_position: (i32, i32),
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
        let mut raw_mine_positions: Vec::<(i32, i32)> = vec![];
        let mut shack_position: Option<(i32, i32)> = None;
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
                        raw_drop_positions.push((pos_x + 1, pos_y));
                        raw_drop_positions.push((pos_x - 1, pos_y));
                        raw_drop_positions.push((pos_x, pos_y + 1));
                        raw_drop_positions.push((pos_x, pos_y - 1));
                    }
                    if c == '+' {
                        let pos_x = acc_inner.len() as i32;
                        let pos_y = acc_outer.len() as i32;
                        raw_mine_positions.push((pos_x + 1, pos_y));
                        raw_mine_positions.push((pos_x - 1, pos_y));
                        raw_mine_positions.push((pos_x, pos_y + 1));
                        raw_mine_positions.push((pos_x, pos_y - 1));
                    }
                    acc_inner.push(Tile::from_char(c));
                    acc_inner
                }));
                acc_outer
            });
        let drop_positions = raw_drop_positions
            .iter()
            .filter(|(x, y)| {
                match grid[y][x] {
                    Tile::Grass(_) => true,
                    _ => false
                }
            })
            .collect::<Vec>();

        let mine_positions_iter = raw_mine_positions
            .iter()
            .filter(|(x, y)| {
                match grid[y][x] {
                    Tile::Grass(_) => true,
                    _ => false
                }
            });

        let (mine_dist, mine_position) = mine_positions_iter
            .map(|&(x, y)| {
                drop_positions
                .iter()
                .map(|&(drop_x, drop_y)| {
                    (i32::abs(x - drop_x) + i32::abs(y - drop_y), (x, y))
                })
                .min_by(|a, b| {
                    a.0.cmp(&b.0)
                })
                .unwrap()
            })
            .min_by(|a, b| {
                a.0.cmp(&b.0)
            })
            .unwrap();

        Grid {
            initial_grid: grid.clone(),
            grid: grid.clone(),
            shack_position,
            drop_positions: drop_positions.clone(),
            mine_position,
            mine_dist,
            width,
            height
        }
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
            let mut troll = Troll::from(&input_line);
            if troll.player == 0 {
                t.push(troll);
            }
            else {
                e_t.push(troll);
            }
            (t, e_t)
        });
    (inventory, e_inventory, trolls, enemy_trolls, trees)
}

fn main() {
    let mut grid = Grid::new();
    let mut game_state = EARLY;

    // game loop
    loop {

        let troll_ratio = match game_state {
            EARLY => [1, 0, 0, 0, 0],
            MID => [1, 1, 1, 1, 1],
            _ => [2, 2, 2, 2, 2]
        };

        let (
            inventory, e_inventory, mut trolls, mut enemy_trolls, mut trees
        ) = parse_loop();
        grid.update_tiles(&trolls, &enemy_trolls, &trees);

        let mut actions = vec![];
        for troll in trolls {
            match troll.act(&grid, &trees, &inventory) {
                Some(a) => actions.push(a),
                _ => {}
            }
        }

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
