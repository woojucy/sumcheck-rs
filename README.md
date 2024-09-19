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

### File Descriptions

1. **lib.rs**:  
   The main library file that integrates all other modules. It imports `term.rs`, `polynomial.rs`, `prover.rs`, `verifier.rs`, and `utils.rs`. It also contains the test suite to demonstrate the interaction between the Prover and Verifier during the Sumcheck Protocol.

2. **term.rs**:  
   Contains the definition of the `Term` struct, which represents a single term in a polynomial. The `Term` struct includes methods for evaluating terms with given variable assignments, and the `Display` trait is implemented to provide a human-readable output format.

3. **polynomial.rs**:  
   Defines the `Polynomial` struct, which represents a multivariate polynomial composed of several `Term` instances. This module includes methods for polynomial evaluation and reduces polynomials during the Sumcheck Protocol.

4. **prover.rs**:  
   Contains the logic for the `Prover` in the Sumcheck Protocol. The Prover generates random polynomials, computes the sum over all possible binary inputs (0 or 1), and interacts with the Verifier by sending reduced polynomials.

5. **verifier.rs**:  
   Defines the `Verifier` struct, which is responsible for verifying the correctness of the Prover's calculations. It includes methods for generating random challenges and evaluating reduced polynomials to ensure that the Prover’s computations are correct.

6. **utils.rs**:  
   Provides utility functions, such as generating random polynomials with randomly assigned coefficients and exponents. The `generate_random_polynomial` function is central to the Prover’s polynomial generation.

## Features

1. **Random Polynomial Generation**: 
   - Polynomials are generated with a configurable number of variables and terms.
   - Each term has randomly assigned coefficients and exponents, controlled by constants.

2. **Polynomial Evaluation**: 
   - Polynomials can be evaluated for specific variable assignments.
   - The Prover computes the sum of the polynomial over all possible binary inputs.

3. **Sumcheck Protocol**: 
   - The Verifier checks if the sum provided by the Prover is correct.
   - The Verifier can verify the Prover's partial evaluations step by step by issuing challenges.
   - The Prover reduces multivariate polynomials to univariate polynomials as part of the verification process.

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

Two constants are defined globally to control the random polynomial generation:

- **`MAX_DEGREE`**:  
  Controls the maximum degree that any variable can have in a term.

- **`MAX_COEFFICIENT`**:  
  Sets the upper limit for the coefficient values of the terms.

These constants can be easily modified to generate more complex or simpler polynomials.

```rust
const MAX_DEGREE: usize = 3;
const MAX_COEFFICIENT: i32 = 10;
```

### Why Use Global Constants?

Global constants offer a flexible and centralized way to adjust key parameters of the polynomial generation process. By modifying `MAX_DEGREE` and `MAX_COEFFICIENT`, developers can change the complexity of the polynomials without altering the core logic.

## Code Overview

### `Term` Struct (term.rs)

Represents a single term in a polynomial, consisting of a coefficient and a map of variable indices to exponents.

```rust
#[derive(Debug, Clone)]
struct Term {
    coefficient: i32,
    exponents: HashMap<usize, usize>,  // Map of variable index to exponent
}
```

### `Polynomial` Struct (polynomial.rs)

Represents a multivariate polynomial composed of several `Term` instances. Includes methods for evaluation.

```rust
#[derive(Debug, Clone)]
struct Polynomial {
    terms: Vec<Term>,  // A polynomial consists of multiple terms
}
```

### `generate_random_polynomial` Function (utils.rs)

Generates a random polynomial based on the number of variables, using `MAX_DEGREE` and `MAX_COEFFICIENT` for the term structure.

```rust
fn generate_random_polynomial(num_variables: usize) -> Polynomial {
    // Logic for generating random polynomial
}
```

### `Prover` Struct (prover.rs)

Generates a polynomial and computes its sum over all binary inputs. Interacts with the Verifier to complete the Sumcheck Protocol.

```rust
struct Prover {
    polynomial: Polynomial,  // The polynomial function
    num_variables: usize,     // Number of variables in the polynomial
}
```

### `Verifier` Struct (verifier.rs)

Challenges the Prover by verifying the correctness of the Prover's polynomial evaluations and reduced univariate polynomials.

```rust
struct Verifier {
    num_variables: usize,
    expected_sum: i32,
}
```

## Running the Tests

To run the included test, which demonstrates the interaction between the Prover and Verifier:

```bash
cargo test
```

### Example Output

```
Generated Polynomial: 3 * x0^2 + 5 * x1^1 * x2^2
Prover calculated sum: 52
Verifier checks input [0, 0, 1]: Prover returned 0
Verifier checks input [1, 0, 1]: Prover returned 20
...
```

## How to Modify Polynomial Configuration

To modify the polynomial's degree or coefficient range, simply change the `MAX_DEGREE` and `MAX_COEFFICIENT` values in the code:

```rust
const MAX_DEGREE: usize = 4;
const MAX_COEFFICIENT: i32 = 20;
```

## Conclusion

This project provides a clear, modular implementation of the Sumcheck Protocol using randomly generated polynomials. The interaction between the Prover and Verifier demonstrates the verification process, ensuring the correctness of polynomial evaluations. The project is easily configurable via global constants, allowing quick adjustment to polynomial complexity.