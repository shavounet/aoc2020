use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::GenericError;
use regex::{Regex, Captures};

#[derive(Default, Debug)]
pub struct BagSpec {
    pub color: String,
    pub can_contain: HashMap<String, usize>,
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
            can_contain,
        })
    }
}


mod tests {
    use crate::day7::BagSpec;
    use std::str::FromStr;

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

        let first_bag = &bag_specs.unwrap()[0];
        assert_eq!(first_bag.color, "light red");
        assert_eq!(first_bag.can_contain.len(), 2);
        assert!(first_bag.can_contain.contains_key("bright white"));
        assert_eq!(first_bag.can_contain.get("bright white"), Some(&1));
        assert!(first_bag.can_contain.contains_key("muted yellow"));
        assert_eq!(first_bag.can_contain.get("muted yellow"), Some(&2));
    }
}
