
use std::fmt;
//use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Reversal {
    first: usize,
    second: usize,
    c1: usize,
    c2: usize,
    line: Vec<char>,
    outpt: Sender<Reversal>,
}

impl Reversal {
    pub fn new(inpt: String, outpt: Sender<Reversal>) -> Reversal {
        // Clean the input string so there's only single spaces splitting things up
        let cleaned = inpt.split_whitespace().collect::<Vec<&str>>().join(" ");
        let ln: Vec<char> = cleaned.chars().collect();
        let lastword = count_words(ln.clone()) - 1;
        Reversal {
            first: 0,
            // Position of the second cursor begins at the start of the final whitespace seperated
            // group of characters. Since 'find' searches for byte position, not char position,
            // this will fail for non-ascii values.
            second: lastword,
            c1: 0,
            c2: word_position(ln.clone(), lastword),
            line: ln,
            outpt: outpt,
        }
    }
    // You'll probably want to run this in another thread.
    pub fn build_states(&mut self) {
        let wc = count_words(self.line.clone()) - 1;
        let mut skipinc = false;
        //let (mut word1, mut word2) = (0, wc);
        while self.first < self.second {
            if (self.c1 >= self.line.len() || self.line[self.c1].is_whitespace()) &&
               (self.c2 >= self.line.len() || self.line[self.c2].is_whitespace()) {

                self.first += 1;
                self.second = wc - self.first;

                // Subtract 1 here since 1 is added on below
                self.c1 = word_position(self.line.clone(), self.first);
                self.c2 = word_position(self.line.clone(), self.second);
                skipinc = true;
            } else if self.line[self.c1] == ' ' {
                // scoot_right
                self.scoot_right();
            } else if self.c2 >= self.line.len() || self.line[self.c2].is_whitespace() {
                self.scoot_left();
                self.c1 -= 1;
            } else {
                // Swap the chars at c1 and c2
                let t = self.line[self.c1];
                self.line[self.c1] = self.line[self.c2];
                self.line[self.c2] = t;
            }
            if !skipinc {
                if !(self.c2 >= self.line.len() || self.line[self.c2] == ' ') {
                    self.c2 += 1
                }
                if !(self.c1 >= self.line.len() || self.line[self.c1] == ' ') {
                    self.c1 += 1
                }
            } else {
                skipinc = false
            }
            self.outpt.send(self.clone()).unwrap();
        }
    }
    fn scoot_right(&mut self) {
        let (a, mut b) = (self.c1, self.c2);
        let t = self.line[b];
        while a != b {
            b -= 1;
            self.line[b + 1] = self.line[b];
        }
        self.line[a] = t;
    }
    fn scoot_left(&mut self) {
        let (mut a, b) = (self.c1, self.c2);
        let t = self.line[a];
        while a != b {
            a += 1;
            self.line[a - 1] = self.line[a];
        }
        self.line[b - 1] = t;
    }
}

impl fmt::Display for Reversal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (w1, w2) = (word_position(self.line.clone(), self.first),
                        word_position(self.line.clone(), self.second));
        for (i, c) in self.line.iter().enumerate() {
            if i == w1 || i == w2 {
                write!(f, "v").unwrap();
            } else {
                write!(f, " ").unwrap();
            }
        }
        write!(f, "\n").unwrap();
        for c in self.line.clone() {
            write!(f, "{}", c);
        }
        write!(f, "\n").unwrap();
        for (i, c) in self.line.iter().enumerate() {
            if i == self.c1 || i == self.c2 {
                write!(f, "^").unwrap();
            } else {
                write!(f, " ").unwrap();
            }
        }
        write!(f, "\n")
    }
}

impl Clone for Reversal {
    fn clone(&self) -> Reversal {
        Reversal {
            first: self.first,
            second: self.second,
            c1: self.c1,
            c2: self.c2,
            line: self.line.clone(),
            outpt: self.outpt.clone(),
        }
    }
}

pub fn count_words(line: Vec<char>) -> usize {
    let mut wc = 0;
    let mut inword = false;

    for c in line {
        if c == ' ' && inword {
            inword = false;
            wc += 1
        } else if !(c == ' ' || inword) {
            inword = true;
        }
    }
    if inword {
        wc += 1
    }
    return wc;
}

// Returns the character position in the vector of chars that the nth word is. Given a string:
//     "foo bar baz"
// Then calling providing a word position of 1 would yield 4, while a position 0 returns 0.
pub fn word_position(line: Vec<char>, wp: usize) -> usize {
    let mut current_word = 0;
    let max = line.len() + 1;

    for (i, c) in line.into_iter().enumerate() {
        // We can count spaces because the string was normalized before being turned into a vector,
        // otherwise this would be bug-ridden.
        if c == ' ' {
            current_word += 1;
        } else if current_word == wp {
            return i;
        }
    }

    return max;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;
    use std::str::FromStr;

    #[test]
    fn test_count_words() {
        let inpt = String::from_str("one  two three    ").unwrap();
        assert!(count_words(inpt.chars().collect()) == 3)
    }
}
