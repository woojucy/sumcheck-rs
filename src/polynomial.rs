use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial,
};
use ark_std::rand::{thread_rng, Rng};

/// Helper function to generate variable combinations based on var_index
/// Each `var_index` is treated as a bitmask where each bit represents a variable.
/// The degree of each variable is randomly selected between 0 and `max_degree`.
fn generate_term_from_var_index(
    var_index: usize,
    num_variables: usize,
    max_degree: usize,
) -> Vec<(usize, usize)> {
    let mut vars = Vec::new();
    let mut rng = thread_rng();

    // Iterate through each bit of var_index to determine which variables are involved
    for i in 0..num_variables {
        if (var_index >> i) & 1 == 1 {
            // Randomly select a degree between 0 and max_degree for each variable
            let degree = rng.gen_range(0..=max_degree);
            if degree > 0 {
                vars.push((i, degree)); // Variable i is included with a random degree if the degree is greater than 0
            }
        }
    }

    vars
}

/// Function to generate a random sparse polynomial with `num_variables` variables
/// and random coefficients, using `var_index` as the basis.
pub fn generate_random_polynomial<F: Field>(
    num_variables: usize,
    max_degree: usize,
) -> SparsePolynomial<F, SparseTerm> {
    let mut rng = thread_rng();

    let total_combinations = (1 << num_variables); // 2^num_variables
    let mut terms = Vec::new();

    // Iterate through each possible var_index (from 0 to total_combinations-1)
    for var_index in 1..total_combinations {
        let vars = generate_term_from_var_index(var_index, num_variables, max_degree); // Get the variable combination with random degrees
        if !vars.is_empty() {
            let coefficient = F::from(1_u128);
            // let coefficient = F::rand(&mut rng); // Generate a random coefficient in the field F
            let term = SparseTerm::new(vars); // Create a SparseTerm for the given var_index combination
            terms.push((coefficient, term)); // Add the term to the list
        }
    }

    SparsePolynomial::from_coefficients_vec(num_variables, terms) // Return the generated polynomial
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::fp128::Fq;

    #[test]
    fn test_generate_random_polynomial() {
        // Test generating a random polynomial with 3 variables and max degree of 3
        let poly = generate_random_polynomial::<Fq>(3, 3);

        // Output the polynomial for verification
        println!("Generated Polynomial: {:?}", poly);

        // Add assertions as needed
        assert!(!poly.terms.is_empty(), "Polynomial should have terms");
    }
}
