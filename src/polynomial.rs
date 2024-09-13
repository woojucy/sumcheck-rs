use crate::term::Term;
use std::fmt;

const PRIME: i32 = 7;

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub terms: Vec<Term>,  // A polynomial consists of multiple terms
}

impl Polynomial {
    pub fn evaluate(&self, variables: &[i32]) -> i32 {
        self.terms.iter().map(|term| term.evaluate(variables)).fold(0, |acc, x| (acc + x) % PRIME)
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let term_strings: Vec<String> = self.terms.iter().map(|term| format!("{}", term)).collect();
        write!(f, "{}", term_strings.join(" + "))
    }
}