use rand::Rng;
use std::collections::HashMap;
use std::fmt;

// Define the maximum degree for any variable and maximum coefficient value as constants
const MAX_DEGREE: usize = 2; // Maximum degree for any variable
const MAX_COEFFICIENT: i32 = 3;

/// Struct to represent a single term in a polynomial
#[derive(Debug, Clone)]
struct Term {
    coefficient: i32,
    exponents: HashMap<usize, usize>,  // Map of variable index to exponent
}

impl Term {
    /// Evaluate the term for the given variables
    fn evaluate(&self, variables: &[i32]) -> i32 {
        let mut result = self.coefficient;
        for (&var_index, &degree) in &self.exponents {
            result *= variables[var_index].pow(degree as u32);
        }
        result
    }
}

/// Implement Display for Term to print the term in a human-readable format
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut term_str = format!("{}", self.coefficient);
        for (&var_index, &degree) in &self.exponents {
            term_str.push_str(&format!(" * x{}^{}", var_index, degree));
        }
        write!(f, "{}", term_str)
    }
}

/// Struct to represent a polynomial
#[derive(Debug, Clone)]
struct Polynomial {
    terms: Vec<Term>,  // A polynomial consists of multiple terms
}

impl Polynomial {
    /// Evaluate the polynomial for the given set of variable assignments
    fn evaluate(&self, variables: &[i32]) -> i32 {
        self.terms.iter().map(|term| term.evaluate(variables)).sum()
    }
}

/// Implement Display for Polynomial to print all terms in a human-readable format
impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let term_strings: Vec<String> = self.terms.iter().map(|term| format!("{}", term)).collect();
        write!(f, "{}", term_strings.join(" + "))
    }
}

/// Function to generate a random polynomial with `num_variables` variables
fn generate_random_polynomial(num_variables: usize) -> Polynomial {
    let mut rng = rand::thread_rng();
    let num_terms = rng.gen_range(1..=2);  // Random number of terms between 1 and 5
    let mut terms = Vec::new();
    
    let mut used_vars = vec![false; num_variables]; // To track which variables are used

    // Generate random terms
    for _ in 0..num_terms {
        let mut exponents = HashMap::new();
        
        // Choose a random number of variables to include in this term
        let num_vars_in_term = rng.gen_range(1..=num_variables);
        
        // Choose which variables to include in this term (without repeats)
        let vars_in_term = rand::seq::index::sample(&mut rng, num_variables, num_vars_in_term).into_vec();

        for &var_index in &vars_in_term {
            let degree = rng.gen_range(1..=MAX_DEGREE);  // Random degree for each variable
            exponents.insert(var_index, degree);
            used_vars[var_index] = true;  // Mark the variable as used
        }

        let coefficient = rng.gen_range(1..=MAX_COEFFICIENT);  // Random coefficient between 1 and MAX_COEFFICIENT
        terms.push(Term {
            coefficient,
            exponents,
        });
    }

    // Ensure all variables are used at least once
    for (var_index, used) in used_vars.iter().enumerate() {
        if !used {
            // Add a term with a single unused variable to ensure it's included in the polynomial
            let mut exponents = HashMap::new();
            let degree = rng.gen_range(1..=MAX_DEGREE);
            exponents.insert(var_index, degree);

            let coefficient = rng.gen_range(1..=MAX_COEFFICIENT);
            terms.push(Term {
                coefficient,
                exponents,
            });
        }
    }

    Polynomial { terms }
}

/// Prover struct
struct Prover {
    polynomial: Polynomial,  // The polynomial function
    num_variables: usize,     // Number of variables in the polynomial
}

impl Prover {
    /// Create a new Prover with a randomly generated polynomial
    fn new(num_variables: usize) -> Self {
        let polynomial = generate_random_polynomial(num_variables);
        Prover {
            polynomial,
            num_variables,
        }
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
                        // If the variable matches the fixed variable (x1), substitute its value (binary_value)
                        new_coefficient *= (binary_value as i32).pow(*degree as u32);
                        new_term.exponents.remove(var_index);  // Remove the fixed variable (x1)
                    }
                }
    
                new_term.coefficient = new_coefficient;
    
                // Only add the term if the resulting coefficient is not zero
                if new_term.coefficient != 0 {
                    temp_terms.push(new_term);
                }
            }
    
            // Sum the terms with the same exponents
            for temp_term in temp_terms {
                if let Some(existing_term) = reduced_terms.iter_mut().find(|t| t.exponents == temp_term.exponents) {
                    existing_term.coefficient += temp_term.coefficient;
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
    fn sum_over_all_inputs(&self) -> i32 {
        let num_variables = self.num_variables;
        let mut sum = 0;

        // Generate all possible combinations of 0 and 1 for the variables
        let combinations = Self::generate_combinations(num_variables);

        // Evaluate the polynomial for each combination of inputs and sum the results
        for input in combinations {
            sum += self.polynomial.evaluate(&input);
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

struct Verifier {
    num_variables: usize,
    expected_sum: i32,
    challenge_values: Vec<i32>,  // Stores the challenge values chosen by the Verifier
}

impl Verifier {
    /// Initializes the Verifier with the expected sum and the number of variables
    fn new(num_variables: usize, expected_sum: i32) -> Self {
        Verifier {
            num_variables,
            expected_sum,
            challenge_values: Vec::new(),  // Initially, no challenges have been chosen
        }
    }

    /// Chooses a random challenge value (0 or 1) for the current round
    /// and stores it in the challenge_values list
    fn choose_challenge(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let challenge_value = rng.gen_range(0..=1);  // Randomly select 0 or 1
        self.challenge_values.push(challenge_value);  // Save the chosen challenge value
        challenge_value
    }

    // /// Verifies the polynomial received from the Prover by evaluating it at 0 and 1
    // /// Checks if the sum of the polynomial evaluated at 0 and 1 matches the expected sum
    // fn verify_polynomial(&self, polynomial: &Polynomial) -> bool {
    //     let sum_at_0 = polynomial.evaluate(&vec![0; 1]);  // Evaluate the polynomial at 0
    //     let sum_at_1 = polynomial.evaluate(&vec![1; 1]);  // Evaluate the polynomial at 1

    //     let verified = sum_at_0 + sum_at_1 == self.expected_sum;
    //     println!(
    //         "Verifier checks reduced polynomial, evaluated at 0: {}, at 1: {}",
    //         sum_at_0, sum_at_1
    //     );

    //     verified
    // }

    fn verify_polynomial(&self, polynomial: &Polynomial) -> bool {
        // Create vectors with 0s and 1s for all variables
        println!("verification poly: {}", polynomial);
        let sum_at_0 = polynomial.evaluate(&vec![0; self.num_variables]);  // Evaluate at 0
        let sum_at_1 = polynomial.evaluate(&vec![1; self.num_variables]);  // Evaluate at 1
    
        let verified = sum_at_0 + sum_at_1 == self.expected_sum;
        println!(
            "Verifier checks reduced polynomial, evaluated at 0: {}, at 1: {} to be: {}",
            sum_at_0, sum_at_1, self.expected_sum
        );
    
        verified
    }
    

    fn verify_and_challenge(
        &mut self,
        prover: &mut Prover,
        // previous_univariate: &Polynomial,  // The previous round's univariate polynomial
        variable_index: usize
    ) -> Option<Polynomial> {
        // Verifier chooses a new challenge value for the current variable
        let challenge_value = self.choose_challenge();
        
        println!(
            "Verifier sends challenge for variable {}: {}",
            variable_index + 1, challenge_value
        );
    
        // Prover sends the reduced polynomial based on the challenge values
        let reduced_polynomial = prover.send_polynomial(self, variable_index);
    
        // // Verifier checks if the reduced polynomial is correct by evaluating it at 0 and 1
        // let sum_at_0 = reduced_polynomial.evaluate(&vec![0; 1]);
        // let sum_at_1 = reduced_polynomial.evaluate(&vec![1; 1]);
        
        // println!(
        //     "Verifier checks reduced polynomial for variable x{}: evaluated at 0 = {}, at 1 = {}, expected sum = {}",
        //     variable_index + 1,
        //     sum_at_0,
        //     sum_at_1,
        //     self.expected_sum
        // );
    
        if !self.verify_polynomial(&reduced_polynomial) {
            println!(
                "Verification failed"
            );
            return None;
        }
    
        // Update the expected sum for the next challenge round using the challenge_value
        self.expected_sum = reduced_polynomial.evaluate(&vec![challenge_value; 1]);
    
        Some(reduced_polynomial)
    }
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumcheck_protocol() {
        let num_variables = 2;  // n-variable polynomial

        // Step 1: Prover generates a polynomial and prints it
        let mut prover = Prover::new(num_variables);
        println!("Generated Polynomial: {}", prover.polynomial);

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {}", sum);

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(num_variables, sum);

        // First round: Prover sends the initial polynomial
        // let mut previous_univariate = prover.send_polynomial(&verifier, 0);

        // Continue for each variable
        for i in 1..num_variables {
            // Verifier verifies the current polynomial and chooses the next challenge
            if let Some(reduced_polynomial) = verifier.verify_and_challenge(&mut prover, i) {
                // Update the previous polynomial for the next round
                // previous_univariate = reduced_polynomial;
            } else {
                panic!("Verification failed for variable {}", i);
            }
        }
    }
}
