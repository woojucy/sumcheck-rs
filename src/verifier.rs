use crate::polynomial::Polynomial;
use crate::utils::*;
use crate::prover::Prover;
use rand::Rng;

pub struct Verifier {
    pub num_variables: usize,
    pub expected_sum: i32,
    pub challenge_values: Vec<i32>,  // Stores the challenge values chosen by the Verifier
}

impl Verifier {
    /// Initializes the Verifier with the expected sum and the number of variables
    pub fn new(num_variables: usize, expected_sum: i32) -> Self {
        Verifier {
            num_variables,
            expected_sum,
            challenge_values: Vec::new(),  // Initially, no challenges have been chosen
        }
    }

    /// Chooses a random challenge value (0 or 1) for the current round
    /// and stores it in the challenge_values list
    pub fn choose_challenge(&mut self) -> i32 {
        let mut rng = rand::thread_rng();
        let challenge_value = rng.gen_range(0..=PRIME);  // Randomly select 0 or 1
        self.challenge_values.push(challenge_value);  // Save the chosen challenge value
        challenge_value
    }

    pub fn verify_polynomial(&self, polynomial: &Polynomial) -> bool {
        // Create vectors with 0s and 1s for all variables
        println!("verification poly: {}", polynomial);
        let sum_at_0 = polynomial.evaluate(&vec![0; self.num_variables]);  // Evaluate at 0
        let sum_at_1 = polynomial.evaluate(&vec![1; self.num_variables]);  // Evaluate at 1
    
        let verified = sum_at_0 + sum_at_1 == self.expected_sum;
        println!(
            "Verifier checks reduced polynomial, evaluated at 0: {}, at 1: {} to be: {}",
            sum_at_0, sum_at_1, self.expected_sum
        );
    
        verified
    }

    pub fn verify_and_challenge(
        &mut self,
        prover: &mut Prover,
        // previous_univariate: &Polynomial,  // The previous round's univariate polynomial
        variable_index: usize
    ) -> Option<Polynomial> {
        // Verifier chooses a new challenge value for the current variable
        let challenge_value = self.choose_challenge();
        
        println!(
            "Verifier sends challenge for variable {}: {}",
            variable_index + 1, challenge_value
        );
    
        // Prover sends the reduced polynomial based on the challenge values
        let reduced_polynomial = prover.interact_with_verifier(self, variable_index);

        if !self.verify_polynomial(&reduced_polynomial) {
            println!(
                "Verification failed"
            );
            return None;
        }
    
        // Update the expected sum for the next challenge round using the challenge_value
        self.expected_sum = reduced_polynomial.evaluate(&vec![challenge_value; 1]);
    
        Some(reduced_polynomial)
    }
    
}