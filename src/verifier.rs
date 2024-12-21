use ark_ff::Field;
use ark_poly::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::Polynomial;
use rand::thread_rng;

pub struct Verifier<F: Field> {
    pub num_variables: usize,
    pub expected_sum: F,
    pub challenge_values: Vec<F>, // Stores the challenge values chosen by the Verifier
}

impl<F: Field> Verifier<F> {
    /// Initializes the Verifier with the expected sum and the number of variables
    pub fn new(num_variables: usize, expected_sum: F) -> Self {
        Verifier {
            num_variables,
            expected_sum,
            challenge_values: Vec::new(), // Initially, no challenges have been chosen
        }
    }

    /// Chooses a random challenge value (0 or 1) for the current round
    /// and stores it in the challenge_values list
    pub fn choose_challenge(&mut self) {
        // let mut rng = test_rng(); // for test
        let mut rng = thread_rng();
        let challenge = F::rand(&mut rng); // Randomly select a field element
        self.challenge_values.push(challenge);
        println!("Selected random challenge: {:?}", challenge);
    }

    /// Verifies the reduced univariate polynomial by evaluating it at 0 and 1
    /// and checks if the sum of these evaluations matches the expected sum
    pub fn verify_polynomial(&self, polynomial: &UniSparsePolynomial<F>, prev_eval: &F) -> bool {
        let sum_at_0 = polynomial.evaluate(&F::zero());
        let sum_at_1 = polynomial.evaluate(&F::one());

        let verified = sum_at_0 + sum_at_1 == *prev_eval;
        println!(
            "Verifier checks reduced polynomial, evaluated at 0: {}, at 1: {} to be: {}",
            sum_at_0, sum_at_1, prev_eval
        );

        verified
    }

    /// Sends the challenge to the prover and receives the reduced univariate polynomial
    /// Verifies the polynomial and returns it if successful, otherwise returns None
    pub fn verify_and_challenge(
        &mut self,
        prover: &mut crate::prover::Prover<F>,
        variable_index: usize,
        expected_sum: &F,
    ) -> Option<UniSparsePolynomial<F>> {
        // Choose a random challenge value
        self.choose_challenge();

        // Ask the prover to send the reduced polynomial
        let reduced_polynomial = prover.send_polynomial(self, variable_index);

        // Output the reduced polynomial for debugging purposes
        println!("Reduced Polynomial: {:?}", reduced_polynomial);

        // Verify the reduced polynomial
        if self.verify_polynomial(&reduced_polynomial, expected_sum) {
            Some(reduced_polynomial)
        } else {
            println!("Verification failed");
            None
        }
    }
}
