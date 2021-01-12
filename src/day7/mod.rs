use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::GenericError;
use regex::{Regex, Captures};
use crate::daily_challenge::DailyChallenge;

#[derive(Default, Debug, Clone)]
pub struct BagSpec {
    pub color: String,
    pub sub_bags: HashMap<String, usize>,
}


impl FromStr for BagSpec {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let main_regex = Regex::new(
            r"^(?P<color>[a-z]+ [a-z]+) bags contain (?:no other bags|(?P<color_deps>(?:\d+ [a-z]+ [a-z]+ bags?(?:, )?)+))\.$"
        )?;

        let color_dep_regex = Regex::new(r"^(?P<count>\d+) (?P<color>[a-z]+ [a-z]+) bags?$")?;

        let captures = main_regex.captures(s);

        if let None = captures {
            return Err(GenericError::new("Spec did not match regex".to_string()));
        }

        let matches = captures.unwrap();
        let color_match = matches.name("color").unwrap().as_str().to_string();

        let mut can_contain = HashMap::default();
        if let Some(dep_matches) = matches.name("color_deps") {
            let deps_captures = dep_matches.as_str()
                .split(", ")
                .map(|dep_spec| match color_dep_regex.captures(dep_spec) {
                    Some(capture) => Ok(capture),
                    None => Err(GenericError::new("Dep spec did not match".to_string()))
                })
                .collect::<Result<Vec<Captures>, _>>()?;

            for capture in deps_captures.into_iter() {
                let color = capture.name("color").unwrap().as_str().to_string();
                let count: usize = capture.name("count").unwrap().as_str().parse()?;

                can_contain.insert(color, count);
            }
        }


        Ok(BagSpec {
            color: color_match,
            sub_bags: can_contain,
        })
    }
}

impl BagSpec {
    pub fn contains_color_recursive(&self, color: &String, list: &BagList, cache: &mut HashMap<String, bool>) -> bool {
        if &self.color == color {
            return false;
        } else if self.sub_bags.contains_key(color.as_str()) {
            return true;
        } else if self.sub_bags.len() == 0 {
            return false;
        } else if cache.contains_key(color.as_str()) {
            return cache.get(color.as_str()).unwrap().clone();
        }

        for (color_dep, _) in (&self.sub_bags).into_iter() {
            let has_color = match list.bags.get(color_dep.as_str()) {
                Some(bag) => bag.contains_color_recursive(color, list, cache),
                None => false
            };

            if has_color {
                cache.insert(self.color.clone(), has_color);
                return true;
            }
        }

        cache.insert(self.color.clone(), false);
        return false;
    }

    pub fn sum_contains_recursive(&self, list: &BagList, cache: &mut HashMap<String, usize>) -> usize {
        if self.sub_bags.len() == 0 {
            return 0;
        } else if cache.contains_key(self.color.as_str()) {
            return cache.get(self.color.as_str()).unwrap().clone();
        }

        let mut sum = 0;
        for (sub_bag_color, count) in (&self.sub_bags).into_iter() {
            let bag = list.bags.get(sub_bag_color.as_str()).unwrap();
            let sub_sum = bag.sum_contains_recursive(list, cache);

            sum += count + count * sub_sum;
        }

        cache.insert(self.color.clone(), sum);

        sum
    }
}

pub struct BagList {
    pub bags: HashMap<String, BagSpec>
}

impl From<Vec<BagSpec>> for BagList
{
    fn from(input: Vec<BagSpec>) -> Self {
        let mut bags = HashMap::default();
        for item in input {
            bags.insert(item.color.clone(), item.clone());
        }

        BagList {
            bags
        }
    }
}

#[derive(Default)]
pub struct Day7 {}

impl DailyChallenge for Day7 {
    type Data = BagSpec;
    type Wrapper = BagList;

    fn get_day_num(&self) -> usize { 7 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut cache = HashMap::default();
        let matches: Vec<_> = (&data.bags).into_iter()
            .filter(|(_, bag)| bag.contains_color_recursive(&"shiny gold".to_string(), data, &mut cache))
            .collect();

        Ok(format!("final count is {}", matches.len()))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut cache = HashMap::default();
        let shiny_gold_bag = data.bags.get("shiny gold").unwrap();

        Ok(format!("final count is {}", shiny_gold_bag.sum_contains_recursive(data, &mut cache)))
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{BagSpec, BagList};
    use std::str::FromStr;
    use std::collections::HashMap;

    #[test]
    fn it_read_specs() {
        let specs = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];

        let bag_specs: Result<Vec<BagSpec>, _> = specs.into_iter()
            .map(|s| BagSpec::from_str(s))
            .collect();

        assert!(bag_specs.is_ok());

        let bag_list: BagList = bag_specs.unwrap().into();

        let first_bag_option = bag_list.bags.get("light red");
        assert!(first_bag_option.is_some());
        let first_bag = first_bag_option.unwrap();

        assert_eq!(first_bag.color, "light red");
        assert_eq!(first_bag.sub_bags.len(), 2);
        assert!(first_bag.sub_bags.contains_key("bright white"));
        assert_eq!(first_bag.sub_bags.get("bright white"), Some(&1));
        assert!(first_bag.sub_bags.contains_key("muted yellow"));
        assert_eq!(first_bag.sub_bags.get("muted yellow"), Some(&2));

        let mut cache = HashMap::default();
        assert!(first_bag.contains_color_recursive(&"shiny gold".to_string(), &bag_list, &mut cache));

        let shiny_gold_bag_option = bag_list.bags.get("shiny gold");
        assert!(shiny_gold_bag_option.is_some());
        let shiny_gold_bag = shiny_gold_bag_option.unwrap();
        let mut cache2 = HashMap::default();
        assert_eq!(shiny_gold_bag.sum_contains_recursive(&bag_list, &mut cache2), 32);
    }
}
