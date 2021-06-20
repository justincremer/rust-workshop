use regex::Regex;

pub struct Session<'a> {
    answer: String,
    letters_guessed: String,
    solution: &'a str,
    guesses_left: u8,
}

impl Session<'_> {
    pub fn new(solution: &str, guesses: u8) -> Session {
        let re = Regex::new(r"[A-Za-z]").unwrap();
        Session {
            answer: re.replace_all(solution, "*"),
            solution: solution,
            guesses_left: guesses,
        }
    }

    fn guess(&self, l: char) -> Option<&str> {
        if !self.letters_guessed.contains(l) {
            self.letters_guessed.add(l);
            match find_all(self.solution, l, None) {
                None => Some("{} is not in the solution"),
                Some(x) => Some(format!(
                    "There are {} instances of the letter {}",
                    x.len(),
                    l
                )),
            }
        }
        None
    }
}

fn find_all(s: &str, c: char, acc: Option<[i8]>) -> Option<[i8]> {
    // s.chars().position(|c| c == l)

    match s.find(c) {
        None => acc,
        Some(i) => {
            acc.insert(i);
            let size = s.len();
            match i {
                size => acc,
                _ => find_all(s[i..], c, acc),
            }
        }
    }
}
