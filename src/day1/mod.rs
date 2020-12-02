pub fn find_complements2(input: &Vec<usize>, goal: usize) -> Option<(usize, usize)> {
    for value1 in input {
        for value2 in input {
            if value1 + value2 == goal {
                return Some((value1.clone(), value2.clone()));
            }
        }
    }

    return None;
}

pub fn find_complements3(input: &Vec<usize>, goal: usize) -> Option<(usize, usize, usize)> {
    for value1 in input {
        for value2 in input {
            for value3 in input {
                if value1 + value2 + value3 == goal {
                    return Some((value1.clone(), value2.clone(), value3.clone()));
                }
            }
        }
    }

    return None;
}
