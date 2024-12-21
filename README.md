# Sumcheck Protocol Implementation

This project implements a simplified version of the **Sumcheck Protocol** using randomly generated multivariate polynomials. The protocol involves a **Prover** that generates and evaluates a polynomial, and a **Verifier** that checks the correctness of the Prover's computations. The core functionality includes generating random polynomials, evaluating them, reducing multivariate polynomials to univariate polynomials, and verifying the sum over all possible inputs.

## Project Structure

The project is organized into several modules, each encapsulating a specific functionality related to polynomial generation, evaluation, and the Sumcheck Protocol. The modules are divided into separate files for improved readability and maintainability:

```
/src
 ├── lib.rs
 ├── prover.rs
 ├── verifier.rs
 ├── polynomial.rs
 └── tests.rs
```

## File Descriptions

#### `prover.rs`
Implements the Prover logic for the Sumcheck Protocol. Key components include:
- **Struct**:
  ```rust
  pub struct Prover<F: Field> {
      pub polynomial: SparsePolynomial<F, SparseTerm>,
      pub num_variables: usize,
      pub steps: Vec<F>,
  }
  ```
- **Methods**: Constructors for initializing with a random or given polynomial, methods for reducing polynomials to univariate forms, and functions for calculating sums over inputs.

#### `verifier.rs`
Defines the Verifier functionality, crucial for ensuring the correctness of the Prover's computations. Key components include:
- **Struct**:
  ```rust
  pub struct Verifier<F: Field> {
      pub num_variables: usize,
      pub expected_sum: F,
      pub challenge_values: Vec<F>,
  }
  ```
- **Methods**: Initialization, random challenge generation, polynomial verification, and a method to facilitate rounds of challenges and verifications.

#### `polynomial.rs`
Manages polynomial structures and operations essential to the protocol. Key components include:
- **Struct**:
  ```rust
  #[derive(Debug, Clone)]
  struct Polynomial {
      terms: Vec<Term>,  // A polynomial consists of multiple terms
  }
  ```
- **Functionality**: Includes methods for generating random polynomials and evaluating them. The structure of terms within a polynomial is managed to support various operations.

These descriptions provide a quick overview of each module's role within the project, highlighting the structures and key methods involved. If further detail is needed or any adjustments are required, please let me know!

## Sumcheck Protocol Process

The **Sumcheck Protocol** follows a multi-round interaction between a **Prover** and a **Verifier**, which involves the following steps:

1. **Prover's Initial Step**:
   - The Prover computes the sum of the polynomial for all possible binary inputs of the variables.
   - The Prover sends this sum to the Verifier.

2. **Verifier's Challenge and Prover's Response**:
   - The Verifier challenges the Prover by asking for a reduction of the polynomial, fixing values of all variables except one.
   - The Prover responds with a univariate polynomial for the challenged variable.

3. **Verification of Reduced Polynomials**:
   - The Verifier checks the reduced polynomial by evaluating it at `0` and `1` and comparing the results with the expected sum.
   - This process continues for each variable until all variables are verified.

4. **Final Check**:
   - The Verifier evaluates the final reduced polynomial to confirm correctness.

## Global Constants for Configuration

`lib.rs` defines key global constants for the polynomial calculations. These constants can be easily modified to generate more complex or simpler polynomials.

```rust
const MAX_DEGREE: usize = 3; //
const MAX_TERMS: usize = 10; // Controls the maximum degree that any variable can have in a term.
const MAX_NUM_VARIABLES: usize = 2; 
```

## Running the Tests

`tests.rs` contains automated tests to ensure that the Sumcheck Protocol, as implemented across different components, functions correctly. To run all tests, use the following command:

```bash
cargo test
```

If you want to run a specific test and see the output as the test progresses, you can use the '--nocapture option'. For example, to run the 'test_sumcheck_protocol' test and view the inputs and outputs:

```bash
cargo test test_sumcheck_protocol -- --nocapture
```
