use crate::{polynomial::generate_random_polynomial, verifier::Verifier};
use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm, Term},
    univariate::SparsePolynomial as UniSparsePolynomial,
    DenseMVPolynomial, Polynomial,
};
use rand::{thread_rng, Rng};

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

    fn reduce_to_univariate(&mut self, step: usize, randoms: Vec<F>) -> UniSparsePolynomial<F> {
        let random: F = match randoms.get(step) {
            Some(&value) => value,
            None => panic!("Index out of bounds: step value is {}, but randoms length is {}", step, randoms.len()),
        };
        self.steps.push(random);
        let mut reduced_terms = Vec::new();

        // Handle the two cases: when the challenged variable is 0 and 1
        for binary_value in [F::zero(), F::one()] {
            let mut temp_terms = Vec::new();

            // Iterate over each term in the polynomial
            for (coeff, term) in &self.polynomial.terms {
                let mut new_coeff = *coeff;
                let mut degree = 0;

                // Iterate over the variables in the term
                for (var_index, var_degree) in term.iter() {
                    if *var_index > step {
                        degree = *var_degree; // Variable to reduce
                        new_coeff *= binary_value.pow(&[*var_degree as u64]);
                    } else if *var_index < step {
                        // Multiply by random challenge for other variables
                        new_coeff *= random.pow(&[*var_degree as u64]);
                    } else {
                        // Variables from future steps are ignored
                        continue;
                    }
                }

                if !new_coeff.is_zero() {
                    temp_terms.push((degree, new_coeff));
                }
            }

            // Sum the terms with the same exponents
            for (degree, new_coeff) in temp_terms {
                if let Some(existing_term) = reduced_terms.iter_mut().find(|(d, _)| *d == degree) {
                    existing_term.1 += new_coeff;
                } else {
                    reduced_terms.push((degree, new_coeff));
                }
            }
        }

        UniSparsePolynomial::from_coefficients_vec(reduced_terms)
    }

    /// Sends the reduced univariate polynomial for the current variable to the Verifier
    pub fn send_polynomial(
        &mut self,
        verifier: &Verifier<F>,
        variable_index: usize,
    ) -> UniSparsePolynomial<F> {
        let polynomial =
            self.reduce_to_univariate(variable_index, verifier.challenge_values.clone());
        // println!("Prover sends reduced polynomial for variable x{}", variable_index + 1);
        // println!("Polynomial: {}", polynomial);
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
