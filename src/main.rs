use std::env::args;
use std::collections::HashMap;

struct Todo {
    map: HashMap<String, bool>
}

impl Todo {
    fn new(file: &str) -> Result<Self, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file)?;

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Self{map}),
            Err(e) if e.is_eof() => {
                Ok(Todo {
                    map: HashMap::new(),
                })
            },
            Err(e) => panic!("An error occured: {}", e)
        }
    }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;

        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = args().nth(1).expect("Please specify an action");
    let item = args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new("db.json").expect("Init failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo Saved"),
            Err(e) => println!("Failed to save. {}", e)
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            Some(_) => {
                match todo.save() {
                    Ok(_) => println!("Todo saved"),
                    Err(e) => println!("An error {}", e),
                }
            }
            None => println!("{} is not present in the map", item)
        }
    }
}
