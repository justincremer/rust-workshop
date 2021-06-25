pub struct Session {
    pub hidden: String,
    pub answer: String,
    pub letters_guessed: String,
    pub guesses_left: u8,
}

impl Session {
    pub fn new(answer: &str, guesses: u8) -> Self {
        let answer = answer.to_lowercase();
        let hidden = String::from(&answer)
            .chars()
            .map(|c| match c {
                _ => '*',
            })
            .collect();

        Session {
            hidden: hidden,
            answer: answer,
            letters_guessed: String::new(),
            guesses_left: guesses,
        }
    }

    pub fn guess(&mut self, c: char) -> &'static Result<(), &str> {
        match self.letters_guessed.contains(c) {
            true => &Err("Cannot guess a letter twice"),
            false => {
                self.letters_guessed.push(c);

                let mut i = 0;
                loop {
                    if self.answer.chars().nth(i).unwrap() == c {
                        self.hidden.chars().nth(i).replace(c);
                    }
                    i += 1;
                    if i == self.answer.len() {
                        break;
                    }
                }
                &Ok(())
            }
        }
    }

    pub fn test_display(&self) {
        println!(
            "{} {} {} {}",
            self.answer, self.hidden, self.letters_guessed, self.guesses_left
        )
    }
}
