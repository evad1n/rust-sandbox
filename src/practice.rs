use std::cmp::Ordering;

// This was for a coding problem that I tried to make as difficult as possible for myself. Learned a lot about traits, deriving, and such though!
fn order(sentence: &str) -> String {
    let mut x: Vec<Special> = sentence
        .split(' ')
        .map(|w| Special(w.to_string()))
        .collect();
    x.sort();
    x.iter()
        .map(|w| w.0.as_str().to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[derive(Eq, Debug)]
struct Special(String);

impl Ord for Special {
    fn cmp(&self, other: &Self) -> Ordering {
        get_number(&self.0).cmp(&get_number(&other.0))
    }
}

impl PartialOrd for Special {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Special {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn get_number(word: &str) -> i32 {
    word.chars().fold(0, |acc, x| match x.to_digit(10) {
        Some(n) => n as i32,
        _ => acc,
    })
}
