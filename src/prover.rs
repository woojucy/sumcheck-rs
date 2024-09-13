use crate::term::Term;
use crate::polynomial::Polynomial;
use crate::verifier::Verifier;
use crate::utils::*;

pub struct Prover {
    pub polynomial: Polynomial,  // The polynomial function
    pub num_variables: usize,     // Number of variables in the polynomial
}

impl Prover {
    /// Create a new Prover with a randomly generated polynomial
    pub fn new(num_variables: usize) -> Self {
        let polynomial = generate_random_polynomial(num_variables);
        Prover {
            polynomial,
            num_variables,
        }
    }
    /// Provides a public interface for interacting with the Verifier
    pub fn interact_with_verifier(&self, verifier: &Verifier, variable_index: usize) -> Polynomial {
        self.send_polynomial(verifier, variable_index)
    }

    fn reduce_to_univariate(&self, variable_index: usize, verifier: &Verifier) -> Polynomial {
        let mut reduced_terms: Vec<Term> = Vec::new();
    
        // Handle the two cases: when the challenged variable is 0 and 1
        for binary_value in [0, 1] {
            // Temporary terms for this specific binary value of x1
            let mut temp_terms = Vec::new();
    
            // Iterate over each term in the polynomial
            for term in &self.polynomial.terms {
                let mut new_term = term.clone();
                let mut new_coefficient = new_term.coefficient;
    
                // Iterate over the exponents in the term
                for (var_index, degree) in &term.exponents {
                    if *var_index == variable_index {
                        // Substitute the binary value and apply mod PRIME
                        new_coefficient *= (binary_value as i32).pow(*degree as u32);
                        new_coefficient %= PRIME;  // Apply mod PRIME
                        new_term.exponents.remove(var_index);  // Remove the fixed variable
                    }
                }
    
                new_term.coefficient = new_coefficient % PRIME;
    
                // Only add the term if the resulting coefficient is not zero
                if new_term.coefficient != 0 {
                    temp_terms.push(new_term);
                }
            }
    
            // Sum the terms with the same exponents
            for temp_term in temp_terms {
                if let Some(existing_term) = reduced_terms.iter_mut().find(|t| t.exponents == temp_term.exponents) {
                    existing_term.coefficient = (existing_term.coefficient + temp_term.coefficient) % PRIME;
                } else {
                    reduced_terms.push(temp_term);
                }
            }
        }
    
        // Return the reduced polynomial with only the terms that contain the free variable
        Polynomial { terms: reduced_terms }
    }
    

    /// Sends the reduced univariate polynomial for the current variable to the Verifier
    fn send_polynomial(&self, verifier: &Verifier, variable_index: usize) -> Polynomial {
        let polynomial = self.reduce_to_univariate(variable_index, verifier);
        println!("Prover sends reduced polynomial for variable x{}", variable_index + 1);
        println!("Polynomial: {}", polynomial);
        polynomial
    }

    /// Calculates the sum of the polynomial over all possible input combinations of 0 and 1
    pub fn sum_over_all_inputs(&self) -> i32 {
        let num_variables = self.num_variables;
        let mut sum = 0;

        // Generate all possible combinations of 0 and 1 for the variables
        let combinations = Self::generate_combinations(num_variables);

        // Evaluate the polynomial for each combination of inputs and sum the results
        for input in combinations {
            sum += self.polynomial.evaluate(&input);
            sum %= PRIME;  // Apply mod PRIME to the sum
        }

        sum
    }

    /// Generates all combinations of 0 and 1 for a given number of variables
    fn generate_combinations(num_variables: usize) -> Vec<Vec<i32>> {
        let mut combinations = Vec::new();

        // Iterate over all numbers from 0 to 2^num_variables - 1
        for i in 0..(1 << num_variables) {
            let mut combo = Vec::new();
            for j in 0..num_variables {
                // Extract the j-th bit from i (0 or 1)
                combo.push((i >> j) & 1);
            }
            combinations.push(combo);
        }
        combinations
    }
}