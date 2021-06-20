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

fn find_all(s: &str, l: char, acc: Option<[usize]>) -> Option<[usize]> {
    s.chars().position(|c| c == l)

    // match s.find(c) {
    //     None => match acc {
    //         None => None,
    //         _ => acc,
    //     },
    //     Some(i) => {
    //         acc.insert(i);
    //         if i == s.len() {
    //             find_all(s., c, acc)
    //         }

    //         match i {
    //             s.len() =>
    //             _ => {
    //                 acc.insert(i),
    //             }
    //         }

    //     }
    // }
    // res
}
