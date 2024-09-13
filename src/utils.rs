use crate::term::Term;
use crate::polynomial::Polynomial;
use rand::Rng;
use std::collections::HashMap;

pub const PRIME: i32 = 7; // Set a prime number for modular arithmetic
pub const MAX_TERMS: i32 = 2;
pub const MAX_DEGREE: i32 = 3;
pub const MAX_COEFF: i32 = 5;

pub fn generate_random_polynomial(num_variables: usize) -> Polynomial {
    let mut rng = rand::thread_rng();
    let num_terms = rng.gen_range(1..=MAX_TERMS);  // Random number of terms between 1 and 5
    let mut terms = Vec::new();
    let mut used_vars = vec![false; num_variables]; // To track which variables are used

    // Generate random terms
    for _ in 0..num_terms {
        let mut exponents = HashMap::new();
        let num_vars_in_term = rng.gen_range(1..=num_variables);
        let vars_in_term = rand::seq::index::sample(&mut rng, num_variables, num_vars_in_term).into_vec();

        for &var_index in &vars_in_term {
            let degree = rng.gen_range(1..=2);  // Random degree for each variable
            exponents.insert(var_index, degree);
            used_vars[var_index] = true;
        }

        let coefficient = rng.gen_range(0..PRIME);  // Random coefficient between 0 and PRIME-1
        terms.push(Term { coefficient, exponents });
    }

    // Ensure all variables are used at least once
    for (var_index, used) in used_vars.iter().enumerate() {
        if !used {
            let mut exponents = HashMap::new();
            let degree = rng.gen_range(1..=MAX_DEGREE);
            exponents.insert(var_index, degree);
            let coefficient = rng.gen_range(1..=MAX_COEFF);
            terms.push(Term { coefficient, exponents });
        }
    }

    Polynomial { terms }
}