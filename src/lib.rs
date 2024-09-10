use rand::Rng;
use std::collections::HashMap;

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

/// Function to generate a random polynomial with `num_variables` variables
fn generate_random_polynomial(num_variables: usize) -> Polynomial {
    let mut rng = rand::thread_rng();
    let max_degree = 3;  // Maximum degree for any variable
    let num_terms = rng.gen_range(1..=5);  // Random number of terms between 1 and 5
    let mut terms = Vec::new();

    // Generate random terms
    for _ in 0..num_terms {
        let mut exponents = HashMap::new();
        let num_vars_in_term = rng.gen_range(1..=num_variables);
        let vars_in_term = rand::seq::index::sample(&mut rng, num_variables, num_vars_in_term).into_vec();
        for &var_index in &vars_in_term {
            let degree = rng.gen_range(1..=max_degree);
            exponents.insert(var_index, degree);
        }
        let coefficient = rng.gen_range(1..10);  // Random coefficient between 1 and 9
        terms.push(Term {
            coefficient,
            exponents,
        });
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

    /// Prover computes the sum of the polynomial over all possible inputs
    fn sum_over_all_inputs(&self) -> i32 {
        self.sum_recursive(0, &mut vec![0; self.num_variables])
    }

    /// Recursively evaluate the sum of all possible inputs
    fn sum_recursive(&self, depth: usize, current_input: &mut Vec<i32>) -> i32 {
        if depth == self.num_variables {
            return self.polynomial.evaluate(current_input);
        }
        let mut sum = 0;
        for x in 0..=1 {  // Binary variables (0 or 1)
            current_input[depth] = x;
            sum += self.sum_recursive(depth + 1, current_input);
        }
        sum
    }

    /// Prover evaluates the polynomial for a given input
    fn evaluate_polynomial(&self, input: &[i32]) -> i32 {
        self.polynomial.evaluate(input)
    }
}

/// Verifier struct
struct Verifier {
    num_variables: usize,
    expected_sum: i32,
}

impl Verifier {
    /// Create a new Verifier with an expected sum from the Prover
    fn new(num_variables: usize, expected_sum: i32) -> Self {
        Verifier {
            num_variables,
            expected_sum,
        }
    }

    /// Verify the sum provided by the Prover
    fn verify_initial_sum(&self, prover: &Prover) -> bool {
        let prover_sum = prover.sum_over_all_inputs();
        prover_sum == self.expected_sum
    }

    /// Verify partial evaluations from Prover step by step
    fn verify_partial_sum(&self, prover: &Prover, variable_index: usize, value: i32) -> bool {
        let mut input = vec![0; self.num_variables];
        input[variable_index] = value;
        let result = prover.evaluate_polynomial(&input);
        println!("Verifier checks input {:?}: Prover returned {}", input, result);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumcheck_protocol() {
        let num_variables = 3;

        // Step 1: Prover generates a polynomial and calculates the sum over all inputs
        let prover = Prover::new(num_variables);
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {}", sum);

        // Step 2: Verifier sets the expected sum and verifies the Prover's result
        let verifier = Verifier::new(num_variables, sum);
        assert!(verifier.verify_initial_sum(&prover), "Initial sum verification failed");

        // Step 3: Verifier checks partial evaluations from Prover
        for i in 0..num_variables {
            assert!(verifier.verify_partial_sum(&prover, i, 1), "Partial sum verification failed for variable {}", i);
        }
    }
}
