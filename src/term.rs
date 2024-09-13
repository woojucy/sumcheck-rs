use std::collections::HashMap;
use std::fmt;

const PRIME: i32 = 7; // Set a prime number for modular arithmetic

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: i32,
    pub exponents: HashMap<usize, usize>,  // Map of variable index to exponent
}

impl Term {
    pub fn evaluate(&self, variables: &[i32]) -> i32 {
        let mut result = self.coefficient;
        for (&var_index, &degree) in &self.exponents {
            result *= variables[var_index].pow(degree as u32);
            result %= PRIME; // Apply mod PRIME to the result
        }
        result
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut term_str = format!("{}", self.coefficient);
        for (&var_index, &degree) in &self.exponents {
            term_str.push_str(&format!(" * x{}^{}", var_index, degree));
        }
        write!(f, "{}", term_str)
    }
}