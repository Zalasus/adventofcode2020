
use std::collections::HashMap;

pub struct Passport {
    fields: HashMap<String, String>
}

impl Passport {

    pub fn new() -> Self {
        Self {
            fields: HashMap::new()
        }
    }

    pub fn new_from_str(s: &str) -> Self {
        let mut p = Self::new();
        for line in s.split('\n') {
            p.add_fields(line);
        }
        p
    }

    pub fn add_fields(&mut self, fields_str: &str) -> usize {
        let mut count = 0;
        for field in fields_str.split(' ').filter(|f| !f.is_empty()) {
            let mut parts = field.split(':');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap();
            if let Some(old_value) = self.fields.insert(key.to_string(), value.to_string()) {
                println!("Warning! Duplicate key {}, was {}", key, old_value);
            }else{
                count += 1;
            }
        }
        count
    }

    pub fn clear(&mut self) {
        self.fields.clear();
    }

    pub fn get_field<'a>(&'a self, key: &str) -> Option<&'a str> {
        match self.fields.get(key) {
            Some(v) => Some(v.as_str()),
            None => None
        }
    }

    pub fn has_required_fields(&self, keys: &[&str]) -> bool {
        for key in keys {
            if let None = self.get_field(key) {
                return false;
            }
        }
        true
    }

    pub fn validate_int_field(&self, key: &str, name: &str, digits: usize, min: i32, max: i32) -> bool {
        if let Some(value) = self.get_field(key) {
            if value.len() != digits {
                println!("{} has invalid digit count: {}", name, value);
                false
            }else if let Ok(int_value) = value.parse::<i32>() {
                (int_value >= min) && (int_value <= max)
            }else{
                println!("{} not parseable: {}", name, value);
                false
            }
        }else{
            println!("{} not present", name);
            false
        }
    }

    pub fn is_valid(&self) -> bool {

        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        if !self.has_required_fields(&required_fields) {
            return false;
        }

        if !self.validate_int_field("byr", "Birth year", 4, 1920, 2002) {
            return false;
        }

        if !self.validate_int_field("iyr", "Issue year", 4, 2010, 2020) {
            return false;
        }

        if !self.validate_int_field("eyr", "Expiry year", 4, 2020, 2030) {
            return false;
        }

        let hgt = self.get_field("hgt").unwrap();
        if let Some(cm_str) = hgt.strip_suffix("cm") {
            if let Ok(cm) = cm_str.parse::<u32>() {
                if cm < 150 || cm > 193 {
                    println!("Height out of valid range: {}cm", cm);
                    return false;
                }
                //println!("Valid height: {}cm", cm);
            }else{
                println!("Height (cm suffix) not parseable");
                return false;
            }
        }else if let Some(inches_str) = hgt.strip_suffix("in") {
            if let Ok(inches) = inches_str.parse::<u32>() {
                if inches < 59 || inches > 76 {
                    println!("Height out of valid range: {}in", inches);
                    return false;
                }
                //println!("Valid height: {}in", inches);
            }else{
                println!("Height (inches suffix) not parseable");
                return false;
            }
        }else{
            println!("Height has invalid suffix");
            return false;
        }

        let hcl = self.get_field("hcl").unwrap();
        if let Some(hcl_hex) = hcl.strip_prefix('#') {
            if hcl_hex.len() != 6 {
                println!("Hair color has invalid length");
                return false;
            }
            let allowed_chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' ];
            for c in hcl_hex.chars() {
                if !allowed_chars.contains(&c) {
                    println!("Hair color has invalid character {}", c);
                    return false;
                }
            }
        }else{
            println!("Hair color has invalid prefix: {}", hcl);
            return false;
        }

        let valid_eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let eye_color = self.get_field("ecl").unwrap();
        if !valid_eye_colors.contains(&eye_color) {
            println!("Invalid eye color {}", eye_color);
            return false;
        }

        if !self.validate_int_field("pid", "Passport ID", 9, 0, i32::MAX){
            return false;
        }

        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_keys() {
        let mut p = Passport::new();
        let count = p.add_fields("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd");
        assert_eq!(count, 4);
        assert_eq!(p.get_field("eyr"), Some("2020"));
        assert_eq!(p.get_field("hcl"), Some("#fffffd"));
        assert_eq!(p.get_field("oaa"), None);
        assert_eq!(p.get_field("ppt"), None);
    }

    #[test]
    fn validity() {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        let mut p = Passport::new();
        p.add_fields("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd");
        p.add_fields("byr:1937 iyr:2017 cid:147 hgt:183cm");
        assert_eq!(p.has_required_fields(&required_fields), true);

        p.clear();
        p.add_fields("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884");
        p.add_fields("hcl:#cfa07d byr:1929");
        assert_eq!(p.has_required_fields(&required_fields), false);
    }
}

fn main() {

    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut have_fields = 0;
    let mut valid_passport_count = 0;

    let passports = std::fs::read_to_string("day4/input.txt").unwrap();
    for passport in passports.split("\n\n") {
        let p = Passport::new_from_str(passport);
        if p.has_required_fields(&required_fields) {
            have_fields += 1;
            if p.is_valid() {
                valid_passport_count += 1;
            }
        }
    }

    println!("{} passports have all required fields. Of these, {} have only valid fields", have_fields, valid_passport_count);
}
