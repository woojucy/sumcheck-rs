use ark_ff::Field;
use ark_poly::multivariate::{SparsePolynomial, SparseTerm, Term};
use ark_poly::DenseMVPolynomial;
// use ark_std::rand::thread_rng;

/// Helper function to generate all combinations of variable degrees
fn generate_all_combinations(num_variables: usize, max_degree: usize) -> Vec<Vec<(usize, usize)>> {
    let mut combinations = Vec::new();

    // Generate 2^num_variables combinations (each variable can have a degree from 0 to max_degree)
    let total_combinations = (max_degree + 1).pow(num_variables as u32);

    for i in 0..total_combinations {
        let mut vars = Vec::new();
        let mut n = i;

        for var_index in 0..num_variables {
            let degree = n % (max_degree + 1); // Choose a degree between 0 and max_degree
            n /= max_degree + 1;
            if degree > 0 {
                vars.push((var_index, degree)); // Add only variables with degree > 0
            }
        }
        combinations.push(vars);
    }
    combinations
}

/// Function to generate a random sparse polynomial with `num_variables` variables
pub fn generate_random_polynomial<F: Field>(
    num_variables: usize,
    max_degree: usize,
) -> SparsePolynomial<F, SparseTerm> {
    // let mut rng = thread_rng();

    // Generate all possible variable combinations
    let combinations = generate_all_combinations(num_variables, max_degree);

    let mut terms = Vec::new();

    // For each combination, generate a random coefficient
    for vars in combinations {
        // let coefficient = F::rand(&mut rng); // Generate a random coefficient in the field F
        let coefficient = F::from(1 as u128);
        let term = Term::new(vars); // Create a Term for the given combination
        terms.push((coefficient, term)); // Add the term to the list
    }
    SparsePolynomial::from_coefficients_vec(num_variables, terms) // Return the generated polynomial
}
