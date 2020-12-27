use std::error::Error;
use std::fs;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Default for Passport {
    fn default() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
}

impl Passport {
    /// Take an input string `field:value` and parse it into the correct variable for `self`
    fn add_field(&mut self, input: &str) -> Result<(), Box<dyn Error>> {
        // Split the field into type: field and value: data
        let (field, data) = match &input.split(':').collect::<Vec<&str>>()[..] {
            &[field, data, ..] => (field, data),
            _ => return Ok(()),
        };

        match field {
            "byr" => self.byr = Some(data.parse()?),
            "iyr" => self.iyr = Some(data.parse()?),
            "eyr" => self.eyr = Some(data.parse()?),
            "hgt" => self.hgt = Some(data.parse()?),
            "hcl" => self.hcl = Some(data.parse()?),
            "ecl" => self.ecl = Some(data.parse()?),
            "pid" => self.pid = Some(data.parse()?),
            "cid" => self.cid = Some(data.parse()?),
            _ => return Err(format!("Error: Could not parse passport field {}", field).into()),
        }

        Ok(())
    }
    /// Determines if a passport is valid based on the rules set
    fn is_valid(&self) -> Result<bool, Box<dyn Error>> {
        // First check if any of the fields are not even entered
        if self.byr == None
            || self.iyr == None
            || self.eyr == None
            || self.hgt == None
            || self.hcl == None
            || self.ecl == None
            || self.pid == None
        {
            return Ok(false);
        }

        // Check byr
        let byr = self.byr.clone().unwrap().parse::<usize>()?;
        if byr < 1920 || byr > 2002 {
            return Ok(false);
        }

        // Check iyr
        let iyr = self.iyr.clone().unwrap().parse::<usize>()?;
        if iyr < 2010 || iyr > 2020 {
            return Ok(false);
        }

        // Check eyr
        let eyr = self.eyr.clone().unwrap().parse::<usize>()?;
        if eyr < 2020 || eyr > 2030 {
            return Ok(false);
        }

        // Check hgt
        let hgt = self.hgt.clone().unwrap();
        let mut index = 0;
        for (i, c) in hgt.chars().enumerate() {
            if !c.is_numeric() {
                index = i;
                break;
            }
        }
        let height = hgt[..index].parse::<usize>()?;
        let h_type = &hgt[index..];

        match h_type {
            "cm" => {
                if height < 150 || height > 193 {
                    return Ok(false);
                }
            }
            "in" => {
                if height < 59 || height > 76 {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        }

        // Check hcl
        let mut hcl: Vec<char> = self.hcl.clone().unwrap().chars().collect();
        if hcl.remove(0) != '#' {
            return Ok(false);
        }
        let len = hcl.len();
        if len != 6 {
            return Ok(false);
        }
        for c in hcl {
            if !c.is_alphanumeric() {
                return Ok(false);
            }
        }

        // Check ecl
        match &self.ecl.clone().unwrap()[..] {
            "amb" => (),
            "blu" => (),
            "brn" => (),
            "gry" => (),
            "grn" => (),
            "hzl" => (),
            "oth" => (),
            _ => return Ok(false),
        };

        // Check pid
        let pid = self.pid.clone().unwrap();
        if pid.len() != 9 {
            return Ok(false);
        }
        let _pid = pid.parse::<usize>()?;

        Ok(true)
    }

    /// Create a new passport given the string of `field:value`s, separated by `'\n'` or `' '`
    pub fn new(data: &str) -> Result<Passport, Box<dyn Error>> {
        let mut passport = Passport {
            ..Default::default()
        };

        for field in data.split(&[' ', '\n'][..]) {
            passport.add_field(field)?;
        }

        Ok(passport)
    }
}

/// Count and return the number of valid `Passport`s
fn check_registry(registry: &Vec<Passport>) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;

    for passport in registry.iter() {
        match passport.is_valid() {
            Ok(valid) => {
                if valid == true {
                    count += 1;
                }
            }
            Err(_) => continue,
        }
    }

    Ok(count)
}

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let raw = fs::read_to_string(filename)?.to_string();
    let mut registry: Vec<Passport> = Vec::new();

    for passport in raw.split("\n\n") {
        registry.push(Passport::new(passport)?)
    }

    let num_valid = check_registry(&registry)?;

    println!("{} passports are valid.", num_valid);

    Ok(())
}