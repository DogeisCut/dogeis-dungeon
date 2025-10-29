use std::io;
use std::any::Any;

struct Health {
    pub value: i32,
    pub max: i32,
}
impl Health {
    fn hurt(&mut self, amount: i32) {
        self.value -= amount;
    }
}

struct Player {
    pub health: Health,
    pub current_weapon: Weapon,
    pub inventory: Vec<Box<dyn Storable>>,
    pub x: usize,
    pub y: usize,
}

trait Entity {
    // Used when describing what is in the room.
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_description(&self) -> String;
    fn get_interaction(&mut self, interaction: InteractionType) -> Option<String>;
}

trait Storable: Entity {}

enum ItemType {
    RedKey,
    BlueKey,
    GreenKey,
    YellowKey,
    PurpleKey,
    HealthPotion(i32)
}
struct Item{
    item_type: ItemType
}
impl Item {
    fn on_item_use(&mut self) {
        match self.item_type {
            ItemType::RedKey => todo!(),
            ItemType::BlueKey => todo!(),
            ItemType::GreenKey => todo!(),
            ItemType::YellowKey => todo!(),
            ItemType::PurpleKey => todo!(),
            ItemType::HealthPotion(healing_power) => todo!(),
        }
    }
    fn on_item_battle_use(&mut self) {
        match self.item_type {
            ItemType::RedKey => todo!(),
            ItemType::BlueKey => todo!(),
            ItemType::GreenKey => todo!(),
            ItemType::YellowKey => todo!(),
            ItemType::PurpleKey => todo!(),
            ItemType::HealthPotion(healing_power) => todo!(),
        }
    }
    // just found out you can impl enums.. might want to move this code there instead as well for future type enums!!
    fn describe(item_type: &ItemType) -> String {
        String::from(match &item_type {
            ItemType::RedKey => "A red key",
            ItemType::BlueKey => "A blue key",
            ItemType::GreenKey => "A green key",
            ItemType::YellowKey => "A yellow key",
            ItemType::PurpleKey => "A purple key",
            ItemType::HealthPotion(healing_power) => {
                //TODO: health potion size names
                "A health potion"
            },
        })
    }
}
impl Entity for Item {
    fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn get_description(&self) -> String {
        Item::describe(&self.item_type)
    }
    
    fn get_interaction(&mut self, interaction: InteractionType) -> Option<String> {
        todo!()
    }
}
impl Storable for Item {}

// Determaines damage multipliers/miss chances
enum WeaponQuality {
    Dilapidated,
    Poor,
    Average,
    Good,
    Great,
    Pristine
}
enum WeaponTypes {
    Shortsword(WeaponQuality),
    Longsword(WeaponQuality),
    Bow(WeaponQuality, ArrowTypes)
}

enum ArrowTypes {
    Basic
}

struct Weapon {
    weapon_type: WeaponTypes,
}
impl Weapon {

}
impl Entity for Weapon {
    fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn get_description(&self) -> String {
        match &self.weapon_type {
            WeaponTypes::Shortsword(weapon_quality) => todo!(),
            WeaponTypes::Longsword(weapon_quality) => todo!(),
            WeaponTypes::Bow(weapon_quality, arrow_types) => todo!(),
        }
    }

    fn get_interaction(&mut self, interaction: InteractionType) -> Option<String> {
        todo!()
    }
}
impl Storable for Weapon {}
// Modifies how this enemy acts/makes decisions in battles, rooms, and other interactions.
enum EnemyPersonality {
    Boring, // Randomly does stuff, with even odds.
    Cautious, // Defends and uses healing items a lot.
    Agressive, // Mostly attacks.
}

enum EnemyTypes {
    Kobald(EnemyPersonality),
}

struct Enemy {
    pub health: Health,
    pub enemy_type: EnemyTypes,
    pub weapon: Weapon,
    pub inventory: Vec<Box<dyn Storable>>
}
impl Enemy {
    fn define(&mut self) {
        match self.enemy_type {
            EnemyTypes::Kobald(_) => {
                self.health.max = 10;
                self.health.value = 10;
            },
        }
    }
}
impl Entity for Enemy {
    fn as_any(&self) -> &dyn Any { self }
	fn as_any_mut(&mut self) -> &mut dyn Any { self }

    fn get_description(&self) -> String {
        let mut result = String::from("A kobald (");
        result += &self.health.value.to_string();
        result += "/";
        result += &self.health.max.to_string();
        result += ")";
        return result
    }

    fn get_interaction(&mut self, interaction: InteractionType) -> Option<String> {
        match self.enemy_type {
            EnemyTypes::Kobald(_) => {
                match interaction {
                    InteractionType::Inspect => Some(String::from("todo")),
                    InteractionType::Open => None,
                    InteractionType::Store => None,
                    InteractionType::Flirt =>  Some(String::from("todo")),
                }
            },
        }
    }
}


struct Chest {
    pub contents: Vec<Box<dyn Entity>>,
    pub requirement: Option<ItemType>    
}

enum ExitType {
    Open,
    Hallway,
    Door,
    LockedDoor(ItemType, bool),
    Staircase
}

enum Directions {
    Up(ExitType),
    Down(ExitType),
    North(ExitType),
    East(ExitType),
    South(ExitType),
    West(ExitType),
}
struct Room {
    pub title: String,
    pub description: String,
    pub exits: Vec<Directions>,
    pub contents: Vec<Box<dyn Entity>>,
    x: usize,
    y: usize
}
impl Room {
    fn describe_exit_type(exit_type: &ExitType) -> String {
        match exit_type {
            ExitType::Open => "Another part of the room".into(),
            ExitType::Hallway => "A hallway".into(),
            ExitType::Door => "A door".into(),
            ExitType::LockedDoor(item_type, mystery) => {
                if !!mystery {
                    "A mysterious locked door".into()
                } else {
                    format!("A locked door with {} shaped slot", Item::describe(&item_type))
                }
            },
            ExitType::Staircase => "A starcase leading".into(),
        }
    }
    fn describe(&self) -> String {
        let mut contents = String::from("");
        let mut exits = String::from("");
        let mut direction_index: usize = 0;
        for direction in &self.exits {
            direction_index += 1;
            exits += &(format!("{}. {}\n", direction_index, match direction {
                Directions::Up(exit_type) => Room::describe_exit_type(exit_type) + " towards the top.",
                Directions::Down(exit_type) => Room::describe_exit_type(exit_type) + " towards the floor.",
                Directions::North(exit_type) => Room::describe_exit_type(exit_type) + " to the north.",
                Directions::East(exit_type) => Room::describe_exit_type(exit_type) + " to the east.",
                Directions::South(exit_type) => Room::describe_exit_type(exit_type) + " to the south.",
                Directions::West(exit_type) => Room::describe_exit_type(exit_type) + " to the west.",
            }))
        }
        let mut entity_index: usize = 0;
        for entity in &self.contents {
            entity_index += 1;
            contents += &(format!("{}. {}\n", entity_index, entity.get_description()));
        }
        let result = format!(
            "{} ({}, {})
            
            {}
            
            Contents:
            {}
            
            Exits:
            {}",
        &self.title, &self.x, &self.y, &self.description, contents, exits);
        return result
    }
}
 
fn main() {
    let player = Player { health: Health{value: 32, max: 32}, current_weapon: Weapon{weapon_type: WeaponTypes::Shortsword(WeaponQuality::Poor)}, inventory: vec![], x: 0, y: 0};
    let mut world: Vec<Vec<Room>> = (0..10)
    .map(|_| Vec::new())
    .collect();

    for y in 0..10 {
        for x in 0..10 {
            world[y].push(Room {
                title: format!("Dusty Chamber"),
                description: "A dusty chamber.".into(),
                exits: vec![],
                contents: vec![],
                x: x,
                y: y
            });
        }
    }
    println!("Welcome to Dogeis Dungeon!");
    loop {
        let mut buffer = String::new();
        println!("TODO: room info");
        println!("Make your move: ");
        io::stdin().read_line(&mut buffer).expect("Failed");
    }
}

enum InteractionType {
    Inspect,
    Open,
    Store,
    Flirt
}
enum Actions {
    Fight(Enemy), 
    Move(Directions),
    Interact(Box<dyn Entity>),
    Drop(Box<dyn Storable>),
    Equip(Weapon),
    Debug
}

enum BattleActions {
    Attack,
    Defend,
    Use(Item)
}

fn handle_action(action: Actions) {
    match action {
        Actions::Fight(enemy) => todo!(),
        Actions::Move(directions) => todo!(),
        Actions::Interact(entity) => todo!(),
        Actions::Drop(storable) => todo!(),
        Actions::Equip(weapon) => todo!(),
        Actions::Debug => todo!(),
    }
}

fn parse_command(input: &str, room: &Room) -> Option<Actions> {
    let words: Vec<&str> = input.trim().split_whitespace().collect();
	if words.is_empty() {
		return None;
	}
    if words[1].parse::<usize>().is_err() {
        return None;
    }
    let chosen_index: usize = words[1].parse().unwrap();
    if chosen_index < 0 {
        return None;
    }

    match words[0] {
        "fight" => if let Some(enemy) = room.contents[chosen_index - 1].as_any().downcast_mut::<Enemy>() {
                Some(Actions::Fight(enemy))
            } else {
                None
            },
        "debug" => Some(Actions::Debug),
        _ => None,
    }
}