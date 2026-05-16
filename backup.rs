use std::{fmt, io};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

// TREE PART

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
enum TrollType {
    // Troll having all carac equal and > 0 (like the first one)
    Poly,
    // Troll having no carryCapacity but chopPower and movementSpeed to destroy enemy trees
    EnemyChoper,
    // Troll having chopPower and carryCapacity
    Choper,
    // Troll only having movementSpeed and carryCapacity
    Planter,
    // Troll having harvestPower and carryCapacity
    Harvester,
    // Troll having high carryCapacity and chopPower
    Miner
}

#[derive(Copy, Clone)]
struct Troll {
    pub id: i32,
    pub player: i32,
    pub x: i32,
    pub y: i32,
    pub movement_speed: i32,
    pub carry_capacity: i32,
    pub harvest_power: i32,
    pub chop_power: i32,
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
        Troll {
            id, player, x, y, movement_speed, carry_capacity, harvest_power,
            chop_power, carry_plum, carry_lemon, carry_apple, carry_banana, carry_iron, carry_wood
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
        self.carry_capacity == self.total_carry()
    }

    fn place_left(&self) -> i32 {
        self.carry_capacity - self.total_carry()
    }

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

    fn harvest(&self) -> String {
        format!("HARVEST {}", self.id)
    }

    fn drop(&self) -> String {
        format!("DROP {}", self.id)
    }

    fn wait(&self) -> String {
        format!("WAIT {}", self.id)
    }

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

    fn act(&self, grid: &Grid, trees: &Vec::<Tree>) -> Option<String> {
        if self.can_drop(&grid.drop_positions) {
            return Some(self.drop())
        }
        if self.is_full() {
            return Some(self.go_to_drop(&grid.drop_positions))
        }
        if self.can_harvest(grid) {
            return Some(self.harvest())
        }
        let best_tree = self.find_shorter_tree(trees);
        match best_tree {
            Some((_, tree)) => Some(self.go_to_tree(tree)),
            None => None
        }
    }
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
}

impl Inventory {
    fn new(plum: i32, lemon: i32, apple: i32, banana: i32, iron: i32, wood: i32) -> Inventory {
        Inventory {
            plum, lemon, apple, banana, iron, wood
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
    pub drop_positions: Vec::<(i32, i32)>,
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
        let mut drop_positions = vec![];
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
                        drop_positions.push((pos_x + 1, pos_y));
                        drop_positions.push((pos_x - 1, pos_y));
                        drop_positions.push((pos_x, pos_y + 1));
                        drop_positions.push((pos_x, pos_y - 1));
                    }
                    acc_inner.push(Tile::from_char(c));
                    acc_inner
                }));
                acc_outer
            });
        Grid {
            initial_grid: grid.clone(),
            grid: grid.clone(),
            drop_positions: drop_positions.clone(),
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

    // game loop
    loop {
        let (
            inventory, e_inventory, mut trolls, mut enemy_trolls, mut trees
        ) = parse_loop();
        grid.update_tiles(&trolls, &enemy_trolls, &trees);

        let mut actions = vec![];
        for troll in trolls {
            match troll.act(&grid, &trees) {
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
