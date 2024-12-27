use crate::{polynomial::generate_random_polynomial, verifier::Verifier};
use ark_ff::Field;
use ark_poly::{
    multivariate::{SparsePolynomial, SparseTerm},
    univariate::SparsePolynomial as UniSparsePolynomial,
    Polynomial,
};

pub struct Prover<F: Field> {
    pub polynomial: SparsePolynomial<F, SparseTerm>,
    pub num_variables: usize,
    pub steps: Vec<F>,
}

impl<F: Field> Prover<F> {
    /// Create a new Prover with a randomly generated polynomial
    pub fn new(num_variables: usize, max_degree: usize, max_terms: usize) -> Self {
        let polynomial = generate_random_polynomial(num_variables, max_degree, max_terms);
        Prover {
            polynomial: polynomial.clone(),
            num_variables: polynomial.num_vars,
            steps: Vec::new(),
        }
    }

    /// Create a new Prover with a given polynomial
    pub fn new_with_polynomial(polynomial: SparsePolynomial<F, SparseTerm>) -> Self {
        // let num_variables = max_variables(&polynomial);
        Prover {
            polynomial: polynomial.clone(),
            num_variables: polynomial.num_vars,
            steps: Vec::new(),
        }
    }

    fn reduce_to_univariate(
        &mut self,
        target_var: usize,
        randoms: Vec<F>,
    ) -> UniSparsePolynomial<F> {
        let mut coefficients = vec![F::zero(); self.polynomial.degree() + 1];
        let v = self.num_variables;

        // Iterate over all input combinations for the remaining variables
        for i in 0..2i32.pow((v - target_var - 1) as u32) {
            let mut inputs: Vec<F> = vec![];
            // Add inputs from previous rounds (randoms)
            inputs.extend(&randoms);

            // Generate inputs for the remaining variables
            let mut counter = i;
            for _ in 0..(v - target_var - 1) {
                if counter % 2 == 0 {
                    inputs.push(F::from(0_u32));
                } else {
                    inputs.push(F::from(1_u32));
                }
                counter /= 2;
            }

            // Evaluate the polynomial at the current input combination
            for (coeff, term) in &self.polynomial.terms {
                let mut c_acc = F::one();
                let mut degree_target = 0;
                let mut has_target_var = false; // Flag to check if target_var is in the term

                // Check each term's variables to determine if target_var is included
                for (var_index, var_degree) in term.iter() {
                    if *var_index == target_var {
                        degree_target = *var_degree;
                        has_target_var = true; // Mark that target_var is in the term
                    } else {
                        // Process variables other than target_var
                        c_acc *= inputs[*var_index].pow([*var_degree as u64]);
                    }
                }

                if !has_target_var {
                    println!(
                        "Adding to constant term: coeff = {:?}, c_acc = {:?}",
                        coeff, c_acc
                    );
                    coefficients[0] += *coeff * c_acc;
                } else {
                    println!(
                        "Adding to degree {:?}: coeff = {:?}, c_acc = {:?}",
                        degree_target, coeff, c_acc
                    );
                    coefficients[degree_target] += *coeff * c_acc;
                }
            }
        }

        // Create the univariate polynomial from the coefficients
        UniSparsePolynomial::from_coefficients_vec(coefficients.into_iter().enumerate().collect())
    }

    /// Sends the reduced univariate polynomial for the current variable to the Verifier
    pub fn send_polynomial(
        &mut self,
        verifier: &Verifier<F>,
        variable_index: usize,
    ) -> UniSparsePolynomial<F> {
        self.reduce_to_univariate(variable_index, verifier.challenge_values.clone())
    }

    /// Calculates the sum of the polynomial over all possible input combinations of 0 and 1
    pub fn sum_over_all_inputs(&self) -> F {
        println!("Debug: self.num_variables = {}", self.num_variables);
        // let univariate_poly = self.convert_to_univariate();
        // Generate all combinations of 0 and 1 for the number of variables
        let combinations = Self::generate_combinations(self.num_variables);

        // Initialize the accumulator for the sum of all evaluations
        let mut sum = F::zero();

        // Iterate over each combination of inputs (0s and 1s)
        for input in combinations {
            // Evaluate the polynomial at the current input and add it to the sum
            let evaluation = self.polynomial.evaluate(&input);
            // let evaluation = univariate_poly.evaluate(&input[0]);
            sum += evaluation;

            // Debugging information
            println!(
                "Input = {:?}, evaluation = {:?}, sum = {:?}",
                input, evaluation, sum
            );
        }
        // Final debug information
        println!("Final sum: {:?}", sum);

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
