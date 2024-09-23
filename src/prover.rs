use crate::{polynomial::generate_random_polynomial, verifier::Verifier};
use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    univariate::SparsePolynomial as UniSparsePolynomial,
    Polynomial,
};
// use rand::Rng;

pub struct Prover<F: Field> {
    pub polynomial: SparsePolynomial<F, SparseTerm>,
    pub num_variables: usize,
    pub steps: Vec<F>,
}

impl<F: Field> Prover<F> {
    /// Create a new Prover with a randomly generated polynomial
    pub fn new(num_variables: usize, max_degree: usize) -> Self {
        let polynomial = generate_random_polynomial(num_variables, max_degree);
        Prover {
            polynomial,
            num_variables,
            steps: Vec::new(),
        }
    }

    fn reduce_to_univariate(
        &mut self,
        target_var: usize,
        randoms: Vec<F>,
    ) -> UniSparsePolynomial<F> {
        let mut reduced_terms_at_0 = Vec::new();
        let mut reduced_terms_at_1 = Vec::new();

        // Iterate over each term in the polynomial
        for (coeff, term) in &self.polynomial.terms {
            let mut new_coeff_at_0 = *coeff;
            let mut new_coeff_at_1 = *coeff;
            let mut degree_target = 0;

            // Iterate over the variables in the term
            for (var_index, var_degree) in term.iter() {
                if *var_index == target_var {
                    // target_var
                    degree_target = *var_degree;
                } else if *var_index < target_var {
                    new_coeff_at_0 *= randoms[*var_index].pow(&[*var_degree as u64]);
                    new_coeff_at_1 *= randoms[*var_index].pow(&[*var_degree as u64]);
                } else {
                    // let random_value = randoms[*var_index];
                    new_coeff_at_0 *= F::zero().pow(&[*var_degree as u64]);
                    new_coeff_at_1 *= F::one().pow(&[*var_degree as u64]);

                    println!(
                        "Evaluating variable x_{}: Coeff at 0 -> {:?}, Coeff at 1 -> {:?}",
                        var_index, new_coeff_at_0, new_coeff_at_1
                    );
                }
            }

            // Add the resulting terms for (target_var, 0) and (target_var, 1)
            if !new_coeff_at_0.is_zero() {
                reduced_terms_at_0.push((degree_target, new_coeff_at_0));
            }
            if !new_coeff_at_1.is_zero() {
                reduced_terms_at_1.push((degree_target, new_coeff_at_1));
            }
        }

        // Combine terms from both evaluations (target_var, 0) and (target_var, 1)
        let mut final_terms = Vec::new();
        for (degree, coeff_at_0) in reduced_terms_at_0 {
            if let Some(existing_term) = final_terms.iter_mut().find(|(d, _)| *d == degree) {
                existing_term.1 += coeff_at_0;
            } else {
                final_terms.push((degree, coeff_at_0));
            }
        }
        for (degree, coeff_at_1) in reduced_terms_at_1 {
            if let Some(existing_term) = final_terms.iter_mut().find(|(d, _)| *d == degree) {
                existing_term.1 += coeff_at_1;
            } else {
                final_terms.push((degree, coeff_at_1));
            }
        }

        println!(
            "Final reduced univariate polynomial terms: {:?}",
            final_terms
        );

        UniSparsePolynomial::from_coefficients_vec(final_terms)
    }

    /// Sends the reduced univariate polynomial for the current variable to the Verifier
    pub fn send_polynomial(
        &mut self,
        verifier: &Verifier<F>,
        variable_index: usize,
    ) -> UniSparsePolynomial<F> {
        let polynomial =
            self.reduce_to_univariate(variable_index, verifier.challenge_values.clone());
        polynomial
    }

    /// Calculates the sum of the polynomial over all possible input combinations of 0 and 1
    pub fn sum_over_all_inputs(&self) -> F {
        // Generate all combinations of 0 and 1 for the number of variables
        let combinations = Self::generate_combinations(self.num_variables);

        // Initialize the accumulator for the sum of all evaluations
        let mut sum = F::zero();

        // Iterate over each combination of inputs (0s and 1s)
        for input in combinations {
            // Evaluate the polynomial at the current input and add it to the sum
            let evaluation = self.polynomial.evaluate(&input);
            sum += evaluation;
        }

        // Return the final sum
        sum
    }

    /// Generates all combinations of 0 and 1 for a given number of variables
    fn generate_combinations(num_variables: usize) -> Vec<Vec<F>> {
        (0..(1 << num_variables))
            .map(|i| {
                (0..num_variables)
                    .map(|j| {
                        if (i >> j) & 1 == 1 {
                            F::one()
                        } else {
                            F::zero()
                        }
                    })
                    .collect()
            })
            .collect()
    }
}
