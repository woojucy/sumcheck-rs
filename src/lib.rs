pub mod term;
pub mod polynomial;
pub mod prover;
pub mod verifier;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::prover::Prover;
    use super::verifier::Verifier;

    #[test]
    fn test_sumcheck_protocol() {
        let num_variables = 2;  // n-variable polynomial

        // Step 1: Prover generates a polynomial and prints it
        let mut prover = Prover::new(num_variables);
        println!("Generated Polynomial: {}", prover.polynomial);

        // Step 2: Prover calculates the sum over all inputs
        let sum = prover.sum_over_all_inputs();
        println!("Prover calculated sum: {}", sum);

        // Step 3: Verifier sets the expected sum
        let mut verifier = Verifier::new(num_variables, sum);

        // First round: Prover sends the initial polynomial
        // let mut previous_univariate = prover.send_polynomial(&verifier, 0);

        // Continue for each variable
        for i in 1..num_variables {
            // Verifier verifies the current polynomial and chooses the next challenge
            if let Some(reduced_polynomial) = verifier.verify_and_challenge(&mut prover, i) {
                // Update the previous polynomial for the next round
                // previous_univariate = reduced_polynomial;
            } else {
                panic!("Verification failed for variable {}", i);
            }
        }
    }
}
