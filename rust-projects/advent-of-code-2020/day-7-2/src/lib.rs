use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

type ID = u32;

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub struct Bag {
    name: String,
    id: ID,
    parents: Vec<u32>,
    children: Vec<(u32, u32)>,
}

impl Bag {
    pub fn new(name: &str) -> Result<Bag, Box<dyn Error>> {
        Ok(Bag {
            name: name.to_string(),
            id: 0,
            parents: Vec::<u32>::new(),
            children: Vec::<(u32, u32)>::new(),
        })
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Bag) -> bool {
        self.id == other.id || self.name == other.name
    }
}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Borrow<ID> for Bag {
    fn borrow(&self) -> &ID {
        &self.id
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::create(config.filename)?;
    let rules = BufReader::new(file);

    let mut bags: HashSet<Bag> = HashSet::new();

    for rule in rules.lines() {
        let rule = rule?;

        let parent = rule.split(" bags contain ").nth(0).unwrap();
        let children: Vec<&str> = rule.split(" bags, | bag.| bags.").collect();

        if bags.contains(parent.to_string()) {}
    }
    Ok(())
}
