use std::collections::HashSet;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
struct Room {
    objects: HashSet<String>,
}

impl Room {
    fn new<'a>(iterable: impl std::iter::Iterator<Item = &'a str>) -> Self {
        Self {
            objects: iterable.map(|a| a.to_owned()).collect(),
        }
    }

    fn describe(&self) {
        println!("the room is dark and small, it contains {:?}", self.objects);
    }

    fn neighbor(self, direction: &str) -> Self {
        match direction {
            "left" => {
                println!("changed to room {}", direction);
                Room::new(vec!["knife", "bread", "butter"].into_iter())
            }
            "north" => {
                println!("changed to room {}", direction);
                Room::new(&mut ["polar-bear", "ice", "snow"].into_iter())
            }
            "right" | "front" | "back" => {
                println!("No room to the {}", direction);
                self
            }
            _ => {
                println!("unknown direction {}", direction);
                self
            }
        }
    }
}

struct Character {
    backpack: HashSet<String>,
}

impl Character {
    fn new() -> Self {
        Self {
            backpack: HashSet::new(),
        }
    }

    fn take(&mut self, obj: String, room: &mut Room) {
        if room.objects.contains(&obj) {
            room.objects.remove(&obj);
            self.backpack.insert(obj);
        } else {
            println!("not found {}", obj);
        }
    }

    fn drop(&mut self, obj: String, room: &mut Room) {
        if self.backpack.contains(&obj) {
            self.backpack.remove(&obj);
            room.objects.insert(obj);
        } else {
            println!("not in backpack {}", obj);
        }
    }
}

fn main() {
    let mut current_room = Room::new(vec!["banana", "chair", "piano"].into_iter());
    let mut character = Character::new();
    let valid_chants = vec!["team", "wildcats"];

    let stdin = io::stdin();
    loop {
        print!("What are you doing next? ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        stdin.lock().read_line(&mut command).unwrap();
        let elems: Vec<&str> = command.split_whitespace().collect();
        println!("{:?}", &elems);
        match &elems[..] {
            ["quit"] => {
                println!("Good bye!");
                break;
            }
            ["look"] => current_room.describe(),
            ["look", "backpack"] => println!("in backpack {:?}", character.backpack),
            ["take", obj] => character.take(obj.to_string(), &mut current_room),
            ["drop", objects @ ..] => {
                for obj in objects {
                    character.drop(obj.to_string(), &mut current_room);
                }
            }
            ["go", direction] => current_room = current_room.neighbor(direction),
            ["go", direction @ ("north" | "south" | "east" | "west")] => {
                current_room = current_room.neighbor(direction)
            }
            ["go", chant] if valid_chants.contains(chant) => {
                println!("YEAH, GOOOOO {}", chant.to_uppercase())
            }
            _ => println!("dunno"),
        }
    }
}
