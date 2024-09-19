use ark_ff::Field;
use ark_poly::univariate::SparsePolynomial as UniSparsePolynomial;
use ark_poly::Polynomial;
// use ark_std::rand::RngCore;
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
    pub fn choose_challenge(&mut self) -> () {
        let mut rng = ark_std::test_rng();
        // let challenge = F::rand(&mut rng); // Randomly select 0 or 1// Save the chosen challenge value
        let challenge = F::from(1_u128); // Randomly select 0 or 1// Save the chosen challenge value
        self.challenge_values.push(challenge);
    }

    pub fn verify_polynomial(&self, polynomial: &UniSparsePolynomial<F>) -> bool {
        // Create vectors with 0s and 1s for all variables
        // println!("verification poly: {}", polynomial);
        let sum_at_0 = polynomial.evaluate(&F::zero());
        let sum_at_1 = polynomial.evaluate(&F::one());

        let verified = sum_at_0 + sum_at_1 == self.expected_sum;
        println!(
            "Verifier checks reduced polynomial, evaluated at 0: {}, at 1: {} to be: {}",
            sum_at_0, sum_at_1, self.expected_sum
        );

        verified
    }

    pub fn verify_and_challenge(
        &mut self,
        prover: &mut crate::prover::Prover<F>,
        variable_index: usize,
    ) -> Option<UniSparsePolynomial<F>> {
        self.choose_challenge();

        let reduced_polynomial = prover.send_polynomial(&self, variable_index);

        println!("Reduced Polynomial: {:?}", reduced_polynomial);

        if self.verify_polynomial(&reduced_polynomial) {
            Some(reduced_polynomial)
        } else {
            println!("Verification failed");
            None
        }
    }
}
