use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    DenseMVPolynomial,
};
use ark_std::rand::{thread_rng, Rng};

/// Helper function to generate variable combinations based on var_index
/// Each `var_index` is treated as a bitmask where each bit represents a variable.
/// The degree of each variable is randomly selected between 0 and `max_degree`.
fn generate_term_from_var_index(num_variables: usize, max_degree: usize) -> Vec<(usize, usize)> {
    let mut vars = Vec::new();
    let mut rng = thread_rng();

    // Iterate through each variable including the constant term (i = 0)
    for i in 0..=num_variables {
        if i == 0 {
            // Special handling for the constant term
            if rng.gen_bool(0.5) {
                // 50% chance to include the constant term
                let degree = rng.gen_range(1..=max_degree); // Ensure degree is at least 1 for the constant term
                vars.push((0, degree));
            }
        } else {
            // Randomly decide whether to include the variable
            if rng.gen_bool(0.5) {
                // 50% chance to include each variable
                let degree = rng.gen_range(0..=max_degree);
                if degree > 0 {
                    vars.push((i, degree)); // Variable i is included with a random degree if the degree is greater than 0
                }
            }
        }
    }

    vars
}

fn count_active_variables<F: Field>(terms: &[(F, SparseTerm)]) -> usize {
    let mut max_variable_index = 0; // To track the maximum variable index

    // Iterate over all terms to find the maximum variable index with non-zero degrees
    for (_, term) in terms {
        for &(variable, degree) in term.iter() {
            // Update the maximum index if the degree is non-zero
            if degree > 0 && variable > max_variable_index {
                max_variable_index = variable;
            }
        }
    }
    // Return the maximum index + 1 to represent the number of variables
    max_variable_index + 1
}

/// Function to generate a random sparse polynomial with `max_num_variables` variables
pub fn generate_random_polynomial<F: Field>(
    max_num_variables: usize,
    max_degree: usize,
    max_terms: usize,
) -> SparsePolynomial<F, SparseTerm> {
    assert!(max_terms >= 1, "max_terms must be at least 1");

    let mut num_variables = 0;

    let mut rng = thread_rng();
    let mut terms = Vec::new();

    // Randomly choose the number of terms to generate, which is less than or equal to max_terms
    let num_terms = rng.gen_range(1..=max_terms);
    println!("Generating {} terms for the polynomial.", num_terms);

    // Generate terms randomly based on num_terms
    for i in 0..num_terms {
        let vars = generate_term_from_var_index(max_num_variables, max_degree);

        // Debug: Print generated vars for the current term
        println!("Term {}: vars = {:?}", i, vars);

        if !vars.is_empty() {
            let mut crng = thread_rng();
            // let coefficient = F::from(1_u128);
            let coefficient = F::rand(&mut crng); // Generate a random coefficient in the field F
            let term = SparseTerm::new(vars.clone()); // Create a SparseTerm for the given combination
            terms.push((coefficient, term)); // Add the term to the list
        }

        // Recalculate num_variables based on active variables
        num_variables = count_active_variables(&terms);

        // Debug: Print current number of active variables
        println!("Current active variables count: {}", num_variables);

        // Debug: Print all current terms
        println!("Current terms: {:?}", terms);
    }

    // Final debug information
    println!("Final number of variables: {}", num_variables);
    println!("Final terms: {:?}", terms);

    SparsePolynomial::from_coefficients_vec(num_variables, terms)
}

/// Calculate the maximum number of variables used in a given SparsePolynomial
pub fn max_variables<F: Field>(polynomial: &SparsePolynomial<F, SparseTerm>) -> usize {
    let mut max_index = 0;
    for (_, term) in &polynomial.terms {
        for (var_index, _) in term.iter() {
            if *var_index > max_index {
                max_index = *var_index;
            }
        }
    }
    max_index + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Zero;
    use ark_test_curves::fp128::Fq; // Example field element for testing

    #[test]
    fn test_generate_random_polynomial_basic() {
        let num_variables = 3;
        let max_degree = 5;
        let max_terms = 10;

        // Generate a random polynomial
        let poly = generate_random_polynomial::<Fq>(num_variables, max_degree, max_terms);

        // Check that the number of terms is less than or equal to max_terms
        assert!(
            poly.terms().len() <= max_terms,
            "Polynomial should have at most max_terms terms"
        );

        // Check that all terms have degrees within the correct range
        for (coeff, term) in poly.terms() {
            assert!(!coeff.is_zero(), "Coefficient should not be zero");

            for (var, deg) in term.iter() {
                assert!(
                    *deg > 0 && *deg <= max_degree,
                    "Degree should be between 1 and max_degree"
                );
                assert!(
                    *var <= num_variables,
                    "Variable index should be within the number of variables"
                );
            }
        }
    }

    #[test]
    fn test_generate_random_polynomial_minimum_input() {
        // Test with the smallest possible values
        let num_variables = 1;
        let max_degree = 1;
        let max_terms = 1;

        let poly = generate_random_polynomial::<Fq>(num_variables, max_degree, max_terms);

        // The polynomial should have at most 1 term
        assert!(
            poly.terms().len() <= 1,
            "Polynomial should have at most 1 term"
        );

        // If a term exists, ensure it follows the constraints
        if let Some((coeff, term)) = poly.terms().get(0) {
            assert!(!coeff.is_zero(), "Coefficient should not be zero");
            for (var, deg) in term.iter() {
                assert!(*deg > 0 && *deg <= max_degree, "Degree should be 1");
                assert!(*var <= num_variables, "Variable index should be 0 or 1");
            }
        }
    }

    #[test]
    fn test_generate_random_polynomial_maximum_input() {
        // Test with a larger input size
        let num_variables = 10;
        let max_degree = 10;
        let max_terms = 100;

        let poly = generate_random_polynomial::<Fq>(num_variables, max_degree, max_terms);

        // Check that the polynomial has the correct number of terms
        assert!(
            poly.terms().len() <= max_terms,
            "Polynomial should have at most max_terms terms"
        );

        // Check all terms' degrees and variables
        for (coeff, term) in poly.terms() {
            assert!(!coeff.is_zero(), "Coefficient should not be zero");

            for (var, deg) in term.iter() {
                assert!(
                    *deg > 0 && *deg <= max_degree,
                    "Degree should be between 1 and max_degree"
                );
                assert!(
                    *var <= num_variables,
                    "Variable index should be within the number of variables"
                );
            }
        }
    }

    #[test]
    fn test_constant_term_only() {
        // Test case where only the constant term can be included
        let num_variables = 1;
        let max_degree = 5;
        let max_terms = 1;

        let poly = generate_random_polynomial::<Fq>(num_variables, max_degree, max_terms);

        // There should be at most one term, which may be the constant term
        assert!(
            poly.terms().len() <= 1,
            "Polynomial should have at most 1 term"
        );

        if let Some((_, term)) = poly.terms().get(0) {
            // If the term is present, check if it's the constant term (i = 0)
            for (var, _) in term.iter() {
                assert!(
                    *var == 0 || *var == 1,
                    "Variable should be either 0 (constant) or 1 (first variable)"
                );
            }
        }
    }

    #[test]
    fn test_max_terms_less_than_one_should_panic() {
        let num_variables = 3;
        let max_degree = 5;

        // max_terms is set to 0, which should cause a panic
        let result = std::panic::catch_unwind(|| {
            generate_random_polynomial::<Fq>(num_variables, max_degree, 0);
        });

        assert!(
            result.is_err(),
            "Function should panic when max_terms is less than 1"
        );
    }

    #[test]
    fn test_empty_polynomial() {
        // Test case where num_variables and max_degree are large but max_terms is very small
        let num_variables = 5;
        let max_degree = 5;
        let max_terms = 1;

        let poly = generate_random_polynomial::<Fq>(num_variables, max_degree, max_terms);

        // The polynomial should contain exactly 1 term at most
        assert!(
            poly.terms().len() <= 1,
            "Polynomial should have at most 1 term"
        );
    }
}
