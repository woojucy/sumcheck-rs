#[cfg(test)]
mod tests {
    use crate::{polynomial::max_variables, prover::Prover, verifier::Verifier};
    use ark_poly::{
        multivariate::{SparsePolynomial, SparseTerm, Term},
        DenseMVPolynomial,
    };
    use ark_test_curves::fp128::Fq;

    #[test]
    fn test_sumcheck_protocol() {
        let num_variables = 2;
        let max_degree: usize = 1;
        let max_terms: usize = 3;

        // Step 1: Prover generates a polynomial
        let mut prover = Prover::<Fq>::new(num_variables, max_degree, max_terms);
        println!("Generated Polynomial: {:?}", prover.polynomial);

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {:?}", sum);

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(num_variables, sum);

        // Step 4: Perform Sumcheck protocol rounds
        for i in 0..num_variables {
            if let Some(_) = verifier.verify_and_challenge(&mut prover, i) {
                println!("Round {} succeeded", i + 1);
            } else {
                println!("Verification failed at round {}", i + 1);
                assert!(false, "Verification failed at round {}", i + 1);
            }
        }

        // If the test completes without failure, the protocol works as expected
        assert!(true);
    }

    #[test]
    fn test_prover_with_given_polynomial() {
        // Define a specific polynomial
        let poly = SparsePolynomial::from_coefficients_vec(
            3,
            vec![
                (Fq::from(1), SparseTerm::new(vec![(0, 1)])),
                (Fq::from(1), SparseTerm::new(vec![(0, 1), (1, 1)])),
                // (Fq::from(1), SparseTerm::new(vec![(1, 1), (0, 1)])),
                // (Fq::from(5), SparseTerm::new(vec![])),
            ],
        );

        // Create a Prover instance using the predefined polynomial
        let mut prover = Prover::<Fq>::new_with_polynomial(poly);

        println!(
            "Prover initialized with Polynomial: {:?}",
            prover.polynomial
        );

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {:?}", sum);

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(max_variables(&prover.polynomial), sum);

        // Step 4: Perform Sumcheck protocol rounds
        let num_variables = max_variables(&prover.polynomial);
        for i in 0..num_variables {
            if let Some(_) = verifier.verify_and_challenge(&mut prover, i) {
                println!("Round {} succeeded", i + 1);
            } else {
                println!("Verification failed at round {}", i + 1);
                assert!(false, "Verification failed at round {}", i + 1);
            }
        }

        // If the test completes without failure, the protocol works as expected
        assert!(true);
    }
}
