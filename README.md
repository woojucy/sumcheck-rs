# Sumcheck Protocol Implementation

This project implements a simplified version of the **Sumcheck Protocol** using randomly generated multivariate polynomials. The protocol involves a **Prover** that generates and evaluates a polynomial, and a **Verifier** that checks the correctness of the Prover's computations. The core functionality includes generating random polynomials, evaluating them, reducing multivariate polynomials to univariate polynomials, and verifying the sum over all possible inputs.

## Features

1. **Random Polynomial Generation**: 
   - Polynomials are randomly generated with a configurable number of variables and terms.
   - Each term has randomly assigned coefficients and exponents.

2. **Polynomial Evaluation**: 
   - Polynomials can be evaluated for specific variable assignments.
   - The Prover computes the sum of the polynomial over all possible binary inputs (0 or 1).

3. **Sumcheck Protocol**: 
   - The Verifier checks if the sum provided by the Prover is correct.
   - The Verifier can also verify the Prover's partial evaluations step by step.
   - The Prover reduces the multivariate polynomial to a univariate polynomial by fixing values for the variables based on challenges provided by the Verifier.
   - The Verifier uses these challenges to verify that the Prover's evaluations are correct.

4. **Global Constants**: 
   - The maximum degree for any variable (`MAX_DEGREE`) and the maximum coefficient (`MAX_COEFFICIENT`) are defined as global constants. These can be easily modified to adjust the behavior of the polynomial generation.

## Sumcheck Protocol Process

The **Sumcheck Protocol** follows a multi-round interactive process between a **Prover** and a **Verifier**. The main steps of the protocol are outlined below:

1. **Prover's Initial Step**:
   - The Prover computes the sum of the polynomial for all possible binary inputs of the variables. This is the sum of evaluating the polynomial for each combination of 0s and 1s for the variables.
   - The Prover sends this sum to the Verifier.

2. **Verifier's Challenge and Prover's Response**:
   - The Verifier selects one variable and challenges the Prover to reduce the polynomial to a univariate polynomial by fixing the values of all variables except the challenged one.
   - The Prover reduces the multivariate polynomial to a univariate polynomial by fixing the other variables to 0 and 1 based on the challenge.
   - The Prover sends the reduced polynomial (which is now a univariate polynomial in the challenged variable) to the Verifier.

3. **Verification of Reduced Polynomials**:
   - The Verifier checks if the Prover's reduced polynomial is correct by evaluating it at `0` and `1` and comparing the results with the expected sum for that round.
   - This process continues iteratively for each variable until all variables are fixed, and the polynomial is fully evaluated and verified.

4. **Final Check**:
   - In the final step, the Verifier evaluates the reduced polynomial with all variables fixed and checks if it matches the expected sum from the previous round.

## Global Constants for Configuration

Two global constants are used to configure the random polynomial generation:

- **`MAX_DEGREE`**: 
  - This constant sets the maximum degree (exponent) that any variable in a term can have. By default, this is set to `3`.
  
- **`MAX_COEFFICIENT`**: 
  - This constant sets the maximum value for the coefficient of any term. By default, this is set to `10`.

These constants are defined using the `const` keyword, making them immutable and accessible across the entire project. You can adjust these constants to suit different configurations for the random polynomial generation.

```rust
// Define the maximum degree for any variable and maximum coefficient value as constants
const MAX_DEGREE: usize = 3;
const MAX_COEFFICIENT: i32 = 10;
```

### Why Use Global Constants?

Global constants provide an easy way to configure and modify important parameters of the polynomial generation process. By defining `MAX_DEGREE` and `MAX_COEFFICIENT` as constants, we ensure that they can be accessed throughout the code without the risk of unintended modifications. Additionally, it allows for easy adjustment of the polynomial properties (e.g., generating more complex or simpler polynomials) without having to modify the core logic.

## Code Overview

### `Term` Struct

The `Term` struct represents a single term in a polynomial. Each term has a `coefficient` and a `HashMap` that maps variable indices to their respective exponents.

- **Fields**:
  - `coefficient`: The coefficient of the term (e.g., `3` in `3 * x^2`).
  - `exponents`: A map where the key is the variable index, and the value is the exponent of that variable (e.g., `x^2` would have `2` as its exponent).

```rust
#[derive(Debug, Clone)]
struct Term {
    coefficient: i32,
    exponents: HashMap<usize, usize>,  // Map of variable index to exponent
}
```

### `Polynomial` Struct

The `Polynomial` struct represents a multivariate polynomial, which is a collection of terms. The `Polynomial` struct includes methods to evaluate the polynomial based on given inputs.

- **Fields**:
  - `terms`: A `Vec<Term>` containing all terms of the polynomial.

```rust
#[derive(Debug, Clone)]
struct Polynomial {
    terms: Vec<Term>,  // A polynomial consists of multiple terms
}
```

### `generate_random_polynomial` Function

This function generates a random polynomial with a specified number of variables. The function uses the global constants `MAX_DEGREE` and `MAX_COEFFICIENT` to control the maximum degree of each variable and the maximum value for the term coefficients.

```rust
fn generate_random_polynomial(num_variables: usize) -> Polynomial {
    let mut rng = rand::thread_rng();
    let num_terms = rng.gen_range(1..=5);  // Random number of terms between 1 and 5
    let mut terms = Vec::new();

    // Generate random terms
    for _ in 0..num_terms {
        let mut exponents = HashMap::new();
        let num_vars_in_term = rng.gen_range(1..=num_variables);
        let vars_in_term = rand::seq::index::sample(&mut rng, num_variables, num_vars_in_term).into_vec();
        for &var_index in &vars_in_term {
            let degree = rng.gen_range(1..=MAX_DEGREE);  // Use MAX_DEGREE constant
            exponents.insert(var_index, degree);
        }
        let coefficient = rng.gen_range(1..MAX_COEFFICIENT);  // Use MAX_COEFFICIENT constant
        terms.push(Term {
            coefficient,
            exponents,
        });
    }

    Polynomial { terms }
}
```

### `Prover` Struct

The `Prover` generates a random polynomial and computes its sum over all binary inputs (0 or 1). The sum is computed recursively by evaluating the polynomial for each possible combination of inputs.

```rust
struct Prover {
    polynomial: Polynomial,  // The polynomial function
    num_variables: usize,     // Number of variables in the polynomial
}
```

### `Verifier` Struct

The `Verifier` checks the correctness of the Prover's computation. It first verifies the sum over all inputs, and then checks partial evaluations step by step.

```rust
struct Verifier {
    num_variables: usize,
    expected_sum: i32,
}
```

## Running the Tests

This project includes a test that demonstrates the interaction between the `Prover` and the `Verifier`. The test generates a random polynomial, computes the sum, and verifies the result.

To run the tests, use:

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

## How to Modify the Polynomial Configuration

To change the behavior of polynomial generation (e.g., increase the maximum degree of variables or the maximum value of coefficients), simply modify the values of `MAX_DEGREE` and `MAX_COEFFICIENT` constants in the code:

```rust
const MAX_DEGREE: usize = 4;  // Increase max degree to 4
const MAX_COEFFICIENT: i32 = 20;  // Increase max coefficient to 20
```

After making the changes, re-run the tests to see the updated behavior.

## Conclusion

This project provides a simplified implementation of the Sumcheck Protocol using randomly generated multivariate polynomials. The Prover and Verifier interact through polynomial evaluation and sum verification, demonstrating the fundamental principles of the protocol. The project is easily configurable via global constants, allowing for quick adjustments to polynomial complexity.
