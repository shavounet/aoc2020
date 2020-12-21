use std::collections::HashSet;
use std::str::FromStr;
use crate::utils::{GenericError, load_data};
use crate::daily_challenge::DailyChallenge;

pub struct Answers {
    all_answers: HashSet<String>,
    shared_answers: HashSet<String>,
}


impl FromStr for Answers {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input_list: Vec<HashSet<String>> = s.split_whitespace().into_iter()
            .map(|substr| {
                let mut set = HashSet::default();
                for char in substr.chars().into_iter() {
                    set.insert(char.to_string());
                }
                set
            })
            .collect();


        let union_chars: HashSet<String> = (&input_list).into_iter()
            .fold(HashSet::default(), |acc_set, cur_set| {
                acc_set.union(&cur_set).cloned().collect()
            });

        let first_input = match (&input_list).first() {
            Some(first_set) => first_set.clone(),
            None => HashSet::default()
        };
        let intersect_chars: HashSet<String> = input_list.into_iter()
            .fold(first_input, |acc_set, cur_set| {
                acc_set.intersection(&cur_set).cloned().collect()
            });


        Ok(Answers {
            all_answers: union_chars,
            shared_answers: intersect_chars,
        })
    }
}

impl Answers {
    pub fn all_yes_count(&self) -> usize {
        self.all_answers.len()
    }
    pub fn shared_yes_count(&self) -> usize {
        self.shared_answers.len()
    }
}

pub struct AnswersList {
    answers_list: Vec<Answers>
}

impl AnswersList {
    pub fn all_yes_count(&self) -> usize {
        (&self.answers_list).into_iter()
            .map(|answers| answers.all_yes_count())
            .sum()
    }
    pub fn shared_yes_count(&self) -> usize {
        (&self.answers_list).into_iter()
            .map(|answers| answers.shared_yes_count())
            .sum()
    }
}

impl From<Vec<Answers>> for AnswersList {
    fn from(answers_list: Vec<Answers>) -> Self {
        AnswersList { answers_list }
    }
}

#[derive(Default)]
pub struct Day6 {}

impl DailyChallenge for Day6 {
    type Data = Answers;
    type Wrapper = AnswersList;

    fn get_day_num(&self) -> usize { 6 }

    fn load_data(&self, file_path: &str) -> Result<Self::Wrapper, GenericError>
        where <Self::Data as std::str::FromStr>::Err: std::error::Error
    {
        let data: Vec<Self::Data> = load_data(file_path, "\n\n")?;
        Ok(data.into())
    }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        Ok(format!("there is {} unique yes", data.all_yes_count()))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        Ok(format!("there is {} shared answers", data.shared_yes_count()))
    }
}
