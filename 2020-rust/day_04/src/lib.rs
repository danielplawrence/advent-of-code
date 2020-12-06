use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;
use regex::Regex;

/// Confirm that a record contains all of the required fields
pub fn validate_record(
    input: &str,
   ) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let required_fields_set: HashSet<&str> = HashSet::from_iter(required_fields.iter().cloned());
    let record_fields: HashSet<&str> = input
       .split_whitespace()
       .filter(|l| !l.is_empty())
       .map(|c|c.split(':').next().unwrap())
       .collect();
    let result = required_fields_set.is_subset(&record_fields);
    return result
   }

pub struct Record {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>
}

impl Record {
    // Another static method, taking two arguments:
    pub fn new(input: &str) -> Record {
        let mut fields_map: HashMap<&str, Option<&str>> = HashMap::new();
        input
       .split_whitespace()
       .filter(|l| !l.is_empty())
       .for_each(|c|{
           let mut c_split = c.split(':');
           fields_map.insert(
            c_split.next().unwrap(),
            c_split.next()
            );
        });
        return Record {
            byr: fields_map.get("byr").and_then(|v| Some(String::from(v.unwrap()))),
            iyr: fields_map.get("iyr").and_then(|v| Some(String::from(v.unwrap()))),
            eyr: fields_map.get("eyr").and_then(|v| Some(String::from(v.unwrap()))),
            hgt: fields_map.get("hgt").and_then(|v| Some(String::from(v.unwrap()))),
            hcl: fields_map.get("hcl").and_then(|v| Some(String::from(v.unwrap()))),
            ecl: fields_map.get("ecl").and_then(|v| Some(String::from(v.unwrap()))),
            pid: fields_map.get("pid").and_then(|v| Some(String::from(v.unwrap()))),
        }
    }

    pub fn is_valid(&self) -> bool {
        return
        Record::validate_byr(self) &
        Record::validate_iyr(self) &
        Record::validate_eyr(self) &
        Record::validate_hgt(self) &
        Record::validate_hcl(self) &
        Record::validate_ecl(self) &
        Record::validate_pid(self)
    }

    fn validate_byr(&self) -> bool {
        if self.byr.is_none() {return false};
        let field_content = self.byr.as_ref().unwrap();
        if field_content.len() != 4 {return false};
        return field_content.parse::<i32>().unwrap() >= 1920 && field_content.parse::<i32>().unwrap() <= 2002;
    }

    fn validate_iyr(&self) -> bool {
        if self.iyr.is_none() {return false};
        let field_content = self.iyr.as_ref().unwrap();
        if field_content.len() != 4 {return false};
        return field_content.parse::<i32>().unwrap() >= 2010 && field_content.parse::<i32>().unwrap() <= 2020;
    }

    fn validate_eyr(&self) -> bool {
        if self.eyr.is_none() {return false};
        let field_content = self.eyr.as_ref().unwrap();
        if field_content.len() != 4 {return false};
        return field_content.parse::<i32>().unwrap() >= 2020 && field_content.parse::<i32>().unwrap() <= 2030
    }

    fn validate_hgt(&self) -> bool {
        if self.hgt.is_none() {return false};

        let field_content = self.hgt.as_ref().unwrap();
        let base_pattern = Regex::new(r"\d+(cm|in)").unwrap();
        let value_pattern = Regex::new(r"\d+").unwrap();
        let units_pattern = Regex::new(r"(cm|in)").unwrap();
        
        if !base_pattern.is_match(&field_content) {return false};
        if !value_pattern.is_match(&field_content) {return false};
        if !units_pattern.is_match(&field_content) {return false};

        let value = value_pattern
        .find(&field_content)
        .unwrap()
        .as_str()
        .parse::<i32>().unwrap();

        let units = units_pattern.find(&field_content)
        .unwrap()
        .as_str();

        if units == "cm" && value >= 150 && value <= 193 {return true};
        if units == "in" && value >= 59 && value <= 76 {return true};
        return false;
    }

    fn validate_hcl(&self) -> bool {
        if self.hcl.is_none() {return false};
        let field_content = self.hcl.as_ref().unwrap();
        if field_content.to_string().len() != 7 {return false};
        let pattern = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
        if pattern.is_match(field_content) {return true};
        return false;
    }

    fn validate_ecl(&self) -> bool {
        if self.ecl.is_none() {return false};
        let field_content = self.ecl.as_ref().unwrap();
        if field_content.to_string().len() != 3 {return false};
        let pattern = Regex::new(r"(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
        if pattern.is_match(field_content) {return true};
        return false;
    }

    fn validate_pid(&self) -> bool {
        if self.pid.is_none() {return false};
        let field_content = self.pid.as_ref().unwrap();
        if field_content.len() != 9 {return false};
        return true;
    }
}

pub fn validate_record_v2(
    input: &str,
   ) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let required_fields_set: HashSet<&str> = HashSet::from_iter(required_fields.iter().cloned());
    let record_fields: HashSet<&str> = input
       .split_whitespace()
       .filter(|l| !l.is_empty())
       .map(|c|c.split(':').next().unwrap())
       .collect();
    let result = required_fields_set.is_subset(&record_fields);
    return result
   }

pub fn split_lines_on_empty_line(input: &str) -> Vec<&str> {
    let re = Regex::new(r"\n\s*\n").unwrap();
    let result: Vec<&str> = re.split(input).into_iter().collect();
    return result;
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record_from() {
        let input = String::from("
        eyr:2029 byr:1931 hcl:z cid:128
        ecl:amb hgt:150cm iyr:2015 pid:148714704
        ");
        let rec = Record::new(&input);
        assert_eq!(rec.byr.unwrap(), 1931);
        assert_eq!(rec.eyr.unwrap(), 2029);
        assert_eq!(rec.hcl.unwrap(), "z");
        assert_eq!(rec.ecl.unwrap(), "amb");
        assert_eq!(rec.hgt.unwrap(), "150cm");
        assert_eq!(rec.iyr.unwrap(), 2015);
        assert_eq!(rec.pid.unwrap(), "148714704");

    }
    #[test]
    fn test_validate_byr_true() {
        let rec = Record {
            byr: Some(1931),
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_byr(), true);
    }
    #[test]
    fn test_validate_byr_false_low() {
        let rec = Record {
            byr: Some(1919),
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_byr(), false);
    }
    #[test]
    fn test_validate_byr_false_high() {
        let rec = Record {
            byr: Some(2003),
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_byr(), false);
    }
    #[test]
    fn test_validate_iyr_true() {
        let rec = Record {
            byr: None,
            iyr: Some(2010),
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_iyr(), true);
    }
    #[test]
    fn test_validate_iyr_false_low() {
        let rec = Record {
            byr: None,
            iyr: Some(2009),
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_iyr(), false);
    }
    #[test]
    fn test_validate_iyr_false_high() {
        let rec = Record {
            byr: None,
            iyr: Some(2021),
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_iyr(), false);
    }
    #[test]
    fn test_validate_eyr_true() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: Some(2020),
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_eyr(), true);
    }
    #[test]
    fn test_validate_eyr_false_low() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: Some(2019),
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_eyr(), false);
    }
    #[test]
    fn test_validate_eyr_false_high() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: Some(2031),
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_eyr(), false);
    }
    #[test]
    fn test_validate_hgt_true_cm() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("151cm")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), true);
    }
    #[test]
    fn test_validate_hgt_true_in() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("60in")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), true);
    }
    #[test]
    fn test_validate_hgt_false_no_units() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("123")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_units_wrong_order() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("cm160")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_no_value() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("cm")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_cm_too_large() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("194cm")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_cm_too_small() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("149cm")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_in_too_large() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("77in")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hgt_false_in_too_small() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: Some(String::from("58in")),
            hcl: None,
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hgt(), false);
    }
    #[test]
    fn test_validate_hcl_true_alphabetic() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("#abcabc")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), true);
    }
    #[test]
    fn test_validate_hcl_true_numbers() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("#012012")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), true);
    }
    #[test]
    fn test_validate_hcl_false_too_many_chars() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("abcabcabcd")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), false);
    }
    #[test]
    fn test_validate_hcl_false_not_enough_chars() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("abc")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), false);
    }
    #[test]
    fn test_validate_hcl_false_no_hash() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("abcabc")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), false);
    }
    #[test]
    fn test_validate_hcl_false_mixed_chars() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("abc021")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), false);
    }
    #[test]
    fn test_validate_hcl_false_wrong_chars() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: Some(String::from("!!!!!!")),
            ecl: None,
            pid: None,
        };
        assert_eq!(rec.validate_hcl(), false);
    }
    #[test]
    fn test_validate_ecl_true() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: Some(String::from("amb")),
            pid: None,
        };
        assert_eq!(rec.validate_ecl(), true);
    }
    #[test]
    fn test_validate_ecl_false_incorrect_length() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: Some(String::from("ambc")),
            pid: None,
        };
        assert_eq!(rec.validate_ecl(), false);
    }
    #[test]
    fn test_validate_ecl_false_not_in_enum() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: Some(String::from("bma")),
            pid: None,
        };
        assert_eq!(rec.validate_ecl(), false);
    }
    #[test]
    fn test_validate_pid_true() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl:  None,
            pid: Some("000000001".to_string()),
        };
        assert_eq!(rec.validate_pid(), true);
    }
    #[test]
    fn test_validate_pid_false() {
        let rec = Record {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl:  None,
            pid: Some("00000001".to_string()),
        };
        assert_eq!(rec.validate_pid(), false);
    }

    #[test]
    fn test_validate_record_true() {
        let input = String::from("
        eyr:2029 byr:1931 hcl:z cid:128
        ecl:amb hgt:150cm iyr:2015 pid:148714704
        ");
        let valid = validate_record(&input);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_validate_record_true_missing_cid() {
        let input = String::from("
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        ");
        let valid = validate_record(&input);
        assert_eq!(valid, true);
    }
    #[test]
    fn test_validate_record_false() {
        let input = String::from("
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929
        ");
        let valid = validate_record(&input);
        assert_eq!(valid, false);
    }

    #[test]
    fn test_split_lines_two_lines(){
        let input = String::from("eyr:2029 byr:1931 hcl:z cid:128
        ecl:amb hgt:150cm iyr:2015 pid:148714704
        
        byr:2013 hgt:70cm pid:76982670 ecl:#4f9a1c
        hcl:9e724b eyr:1981 iyr:2027
        ");
        let lines = split_lines_on_empty_line(&input);
        assert_eq!(lines.len(), 2);
    }
    #[test]
    fn test_split_lines_one_line(){
        let input2 = String::from("
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        ");
        let lines2 = split_lines_on_empty_line(&input2);
        assert_eq!(lines2.len(), 1);    
    }

    #[test]
    fn test_split_lines_three_lines(){
        let input3 = String::from("
        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm
        ");
        let lines3 = split_lines_on_empty_line(&input3);
        assert_eq!(lines3.len(), 3);   
    }
}
