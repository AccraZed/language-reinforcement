use std::io::{BufRead, BufReader};
use std::{error::Error, fs::File};

pub struct Config {
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub struct Bag<'b> {
    name: String,
    children: Vec<(&'b Bag<'b>, usize)>,
}

impl<'b> Bag<'b> {
    /// Create a new bag with the given name
    pub fn new(name: &str) -> Result<Bag<'b>, Box<dyn Error>> {
        Ok(Bag {
            name: String::from(name),
            children: Vec::<(&'b Bag, usize)>::new(),
        })
    }
}

pub struct RuleSet<'rs> {
    bags: Vec<Bag<'rs>>,
}

impl<'rs> RuleSet<'rs> {
    /// Create a new RuleSet
    pub fn new() -> Result<RuleSet<'rs>, Box<dyn Error>> {
        Ok(RuleSet {
            bags: Vec::<Bag<'rs>>::new(),
        })
    }
    /// Update the list of bags so that they all link to each other's addresses
    pub fn update(&self) {}
    /// Return the address of a bag if it exists, or None otherwise
    pub fn find(&'rs mut self, query: &str) -> Result<Option<&'rs Bag<'rs>>, Box<dyn Error>> {
        for bag in self.bags.iter() {
            if bag.name == query {
                return Ok(Some(bag));
            }
        }
        Ok(None)
    }
    /// Returns a boolean depending on whether the bag name exists
    pub fn exists(&self, query: &str) -> Result<bool, Box<dyn Error>> {
        for bag in self.bags.iter() {
            if bag.name == query {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// Push a bag into the ruleset
    pub fn push(&mut self, bag: Bag<'rs>) -> Result<&Bag<'rs>, Box<dyn Error>> {
        self.bags.push(bag);

        Ok(self.bags.last().unwrap())
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::create(config.filename)?;
    let rules = BufReader::new(file);

    let mut ruleset = RuleSet::new()?;

    for rule in rules.lines() {
        let rule = rule?;

        let parent = rule.split(" bags contain ").nth(0).unwrap();
        let children: Vec<&str> = rule.split(" bags, | bag.| bags.").collect();

        for child in children {
            if child == "no other bags." {
                break;
            }
            let num = child[..2].trim().parse::<usize>()?;

            ruleset.bags.push(Bag::new(parent)?);

            if ruleset.exists(child)? {
                ruleset
                    .bags
                    .last_mut()
                    .unwrap()
                    .children
                    .push((ruleset.find(child)?.unwrap(), num));
            } else {
                ruleset
                    .bags
                    .last_mut()
                    .unwrap()
                    .children
                    .push((&Bag::new(child)?, num));
            }
        }

        // then, add children bags to ruleset list

        // finally, add references to children in parent bag data
    }
    Ok(())
}
