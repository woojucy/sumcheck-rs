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
    num_variables: usize,
    max_degree: usize,
) -> Vec<(usize, usize)> {
    let mut vars = Vec::new();
    let mut rng = thread_rng();

    // Iterate through each variable including the constant term (i = 0)
    for i in 0..=num_variables {
        if i == 0 {
            // Special handling for the constant term
            if rng.gen_bool(0.5) { // 50% chance to include the constant term
                let degree = rng.gen_range(1..=max_degree); // Ensure degree is at least 1 for the constant term
                vars.push((0, degree));
            }
        } else {
            // Randomly decide whether to include the variable
            if rng.gen_bool(0.5) { // 50% chance to include each variable
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
            let coefficient = F::from(1_u128);
            // let coefficient = F::rand(&mut rng); // Generate a random coefficient in the field F
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


