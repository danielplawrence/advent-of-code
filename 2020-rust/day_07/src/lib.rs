use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

#[derive(Debug,Clone)]
pub struct Bag {
    pub name: String,
    pub contains: HashMap<String, i32>
}

#[derive(Debug,Clone)]
pub struct Bags {
    bags: HashMap<String, Bag>
}
impl Bags {
    pub fn new() -> Bags {
        return Bags {
            bags: HashMap::new()
        }
    }
    pub fn add_bag(&mut self, bag: Bag, name: &str){
        self.bags.insert(String::from(name), bag);
    }
    pub fn get_bag(&self, name: &str) -> &Bag {
        return self.bags.get(name).unwrap();
    }
    //Return the names of bags which contain a bag with name
    //or who have a child containing a bag with name
    pub fn get_bags_containing(&self, name: &str) -> HashSet<String> {
        let mut res = HashSet::<String>::new();
        for bag_name in self.bags.keys(){
            if self.bag_contains(bag_name, name){
                res.insert(bag_name.to_string());
            }
        }
        return res;
    }
    pub fn bag_contains(&self, bag: &str, target: &str) -> bool {
        let t = self.get_bag(bag);
        if t.contains.len() == 0 {
            return false;
        }
        if t.contains.contains_key(target){
            return true
        }
        for k in t.contains.keys(){
            if self.bag_contains(k, target){
                return true
            }
        }
        return false;
    }
}

//Given a rule in plain text,
//return a map of bag types
//to allowed counts
pub fn parse_rule(input: &str) -> Bag {
    let target = input.split("contain").nth(0).unwrap().trim();
    let cleanup_pattern = Regex::new(r"(s$|s[,\.]|\.$)").unwrap();
    let name = cleanup_pattern.replace_all(target, "");
    let contains_strings = input.split("contain").nth(1).unwrap().split(", ");
    let contains: HashMap<String, i32> = contains_strings.fold(HashMap::new(), |mut contains, mut value| {
        value = value.trim();
        if value == "no other bags." {
            return contains;
        }
        let count_pattern = Regex::new("[0-9]").unwrap();
        let bags_pattern = Regex::new(r"([a-z\s\.])+$").unwrap();
        let count: i32 = count_pattern.captures(value).unwrap().get(0).unwrap().as_str().parse().unwrap();
        let bag = bags_pattern.captures(value).unwrap().get(0).unwrap().as_str().trim();
        contains.insert(String::from(cleanup_pattern.replace_all(bag, "")), count);
        return contains
    });
   return Bag {
       name: String::from(name),
       contains: contains
   }
}
// Find all bags directly containing target
// For each of those, find all bag types which can contain them u

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_rule(){
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags";
        let res = parse_rule(&input);
        assert_eq!(res.name, "light red bag");
        let contains = res.contains;
        println!("{:#?}", contains);
        assert_eq!(&1, contains.get("bright white bag").unwrap());
        assert_eq!(&2, contains.get("muted yellow bag").unwrap());
    }

    #[test]
    fn test_get_bags_containing_one_level(){
        let red_bag_map = HashMap::new();
        let yellow_bag_map = HashMap::new();
        let mut red_bag = Bag {
            name: String::from("red bag"),
            contains: red_bag_map
        };
        let mut yellow_bag = Bag {
            name: String::from("yellow bag"),
            contains: yellow_bag_map
        };
        red_bag.contains.insert(String::from("yellow bag"), 1);
        let mut bags = Bags::new();
        bags.add_bag(red_bag, &"red bag");
        bags.add_bag(yellow_bag, &"yellow bag");
        let res = bags.get_bags_containing(&"yellow bag");
        assert_eq!(res.len(), 1);
        assert_eq!(res.contains(&String::from("red bag")), true);
    }
    #[test]
    fn test_get_bags_containing_two_levels(){
        let red_bag_map = HashMap::new();
        let yellow_bag_map = HashMap::new();
        let blue_bag_map = HashMap::new();
        let mut red_bag = Bag {
            name: String::from("red bag"),
            contains: red_bag_map
        };
        let mut yellow_bag = Bag {
            name: String::from("yellow bag"),
            contains: yellow_bag_map
        };
        let mut blue_bag = Bag {
            name: String::from("blue bag"),
            contains: blue_bag_map
        };
        red_bag.contains.insert(String::from("yellow bag"), 1);
        blue_bag.contains.insert(String::from("red bag"), 1);
        blue_bag.contains.insert(String::from("yellow bag"), 1);
        let mut bags = Bags::new();
        bags.add_bag(red_bag, &"red bag");
        bags.add_bag(yellow_bag, &"yellow bag");
        bags.add_bag(blue_bag, &"blue bag");
        let res = bags.get_bags_containing(&"yellow bag");
        assert_eq!(res.len(), 2);
        assert_eq!(res.contains(&String::from("blue bag")), true);
        assert_eq!(res.contains(&String::from("red bag")), true);
    }
}
