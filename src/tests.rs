#[cfg(test)]
mod tests {
    use crate::{
        polynomial::max_variables, prover::Prover, verifier::Verifier, MAX_DEGREE,
        MAX_NUM_VARIABLES, MAX_TERMS,
    };
    use ark_poly::{
        multivariate::{SparsePolynomial, SparseTerm, Term},
        // univariate::SparsePolynomial as UniSparsePolynomial,
        DenseMVPolynomial,
        Polynomial,
    };
    use ark_test_curves::fp128::Fq;

    #[test]
    fn test_sumcheck_protocol() {
        let max_num_variables: usize = MAX_NUM_VARIABLES;
        let max_degree: usize = MAX_DEGREE;
        let max_terms: usize = MAX_TERMS;

        // Step 1: Prover generates a polynomial
        let mut prover = Prover::<Fq>::new(max_num_variables, max_degree, max_terms);
        println!("Generated Polynomial: {:?}", prover.polynomial);

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {:?}", sum);

        let num_variables = prover.num_variables;

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(num_variables, sum);
        let mut eval = sum;

        // Step 4: Perform Sumcheck protocol rounds
        for i in 0..num_variables {
            println!("challenge: {:?}", verifier.challenge_values);
            let i_poly = prover.reduce_to_univariate(i, &verifier.challenge_values);
            if let Some(current_eval) = verifier.verify_and_challenge(&i_poly, i, &eval) {
                println!("Round {} succeeded", i + 1);
                // println!("challenge: {:?}", verifier.challenge_values);
                // sum = ith_poly.evaluate(&verifier.challenge_values[i])
                eval = current_eval;
            } else {
                assert!(false, "Verification failed at round {}", i);
            }
        }
        let init_poly = prover.polynomial;
        assert!(
            init_poly.evaluate(&verifier.challenge_values) == eval,
            "Initial evaluated value does not match the sum."
        );
    }

    #[test]
    fn test_prover_with_given_polynomial() {
        // Define a specific polynomial
        let poly = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (Fq::from(3), SparseTerm::new(vec![(0, 3), (1, 1)])),
                (Fq::from(3), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (Fq::from(2), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );

        // Create a Prover instance using the predefined polynomial
        let mut prover = Prover::<Fq>::new_with_polynomial(poly);
        let num_variables = max_variables(&prover.polynomial);

        println!(
            "Prover initialized with Polynomial: {:?}",
            prover.polynomial
        );

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {:?}", sum);

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(num_variables, sum);
        let mut eval = sum;

        // Step 4: Perform Sumcheck protocol rounds
        for i in 0..num_variables {
            println!("challenge: {:?}", verifier.challenge_values);
            let i_poly = prover.reduce_to_univariate(i, &verifier.challenge_values);
            if let Some(current_eval) = verifier.verify_and_challenge(&i_poly, i, &eval) {
                println!("Round {} succeeded", i + 1);
                // println!("challenge: {:?}", verifier.challenge_values);
                // sum = ith_poly.evaluate(&verifier.challenge_values[i])
                eval = current_eval;
            } else {
                assert!(false, "Verification failed at round {}", i);
            }
        }
        let init_poly = prover.polynomial;
        assert!(
            init_poly.evaluate(&verifier.challenge_values) == eval,
            "Initial evaluated value does not match the sum."
        );
    }
}
