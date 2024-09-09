pub fn add(left: usize, right: usize) -> usize {
    left + right
}
// Define a trait for polynomial evaluation and summation
pub trait Polynomial {
    // Evaluate the polynomial at a given value x
    fn evaluate(&self, x: f64) -> f64;

    // Summation of a list of polynomials evaluated at a given value x
    fn summation(polynomials: &[Self], x: f64) -> f64
    where
        Self: Sized,
    {
        polynomials.iter().map(|poly| poly.evaluate(x)).sum()
    }
}

// Example implementation for a simple polynomial represented by coefficients
pub struct SimplePolynomial {
    pub coefficients: Vec<f64>, // Polynomial coefficients from lowest to highest degree
}

// Implementing the Polynomial trait for SimplePolynomial
impl Polynomial for SimplePolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(i, &coef)| coef * x.powi(i as i32)) // coef * x^i
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Example usage
        let poly1 = SimplePolynomial {
            coefficients: vec![1.0, 2.0, 3.0], // 1 + 2x + 3x^2
        };
    
        let poly2 = SimplePolynomial {
            coefficients: vec![0.0, 1.0, 1.0], // x + x^2
        };
    
        let polynomials: Vec<SimplePolynomial> = vec![poly1, poly2];
    
        // Evaluate each polynomial at x = 2
        for (i, poly) in polynomials.iter().enumerate() {
            println!("Polynomial {} evaluated at x = 2: {}", i + 1, poly.evaluate(2.0));
        }
    
        // Summation of all polynomials evaluated at x = 2
        let sum = SimplePolynomial::summation(&polynomials, 2.0);
        println!("Summation of all polynomials at x = 2: {}", sum);
    }
}
