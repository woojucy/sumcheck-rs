#[cfg(test)]
mod tests {
    use crate::{prover::Prover, verifier::Verifier};
    use ark_test_curves::fp128::Fq;

    #[test]
    fn test_sumcheck_protocol() {
        let num_variables = 2;
        let max_degree: usize = 1;

        // Step 1: Prover generates a polynomial
        let mut prover = Prover::<Fq>::new(num_variables, max_degree);
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
}
