struct Evaluator {
    // Treated like a stack
    stack: Vec<f32>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { stack: vec![] }
    }

    pub fn evaluate(mut self, instructions: &str) -> f32 {
        // split the instructions string at every space
        // iterate through the set of instructions
        // match on the optional for parsing to `f32`
        // if it can match,
        //      then push onto the stack
        // if it cannot, try matching on the operators set { "+", "-", "*", "/" }
        // if it can match,
        //      then pop two from the stack and evaluate them with the operator
        // otherwise `panic`

        for instruction in instructions.split(" ") {
            match instruction {
                "+" => &self.add(),
                "-" => &self.subtract(),
                "*" => &self.multiply(),
                "/" => &self.divide(),
                _ => {
                    let value: f32 = instruction.parse().unwrap();
                    &self.push(value)
                }
            }
            // match instruction {

            // }
        }

        4.0
    }

    /// Pushes a value onto the stack in Evaluator
    fn push(mut self, value: f32) {
        self.stack.push(value);
    }

    /// Pops the last and the next-to-last numbers on the stack,
    /// adds the last to the next-to-last number and returns the sum
    fn add(mut self) {
        let last = self.stack.pop().unwrap();
        let next_to_last = self.stack.pop().unwrap();

        self.push(next_to_last + last);
    }

    /// Pops the last and the next-to-last numbers on the stack,
    /// subtracts the last from the next-to-last number and returns the difference
    fn subtract(mut self) {
        let last = self.stack.pop().unwrap();
        let next_to_last = self.stack.pop().unwrap();

        self.push(next_to_last - last);
    }

    /// Pops the last and the next-to-last numbers on the stack,
    /// multiplies the last and the next-to-last number and returns the product
    fn multiply(mut self) {
        let last = self.stack.pop().unwrap();
        let next_to_last = self.stack.pop().unwrap();

        self.push(next_to_last * last);
    }

    /// Pops the last and the next-to-last numbers on the stack,
    /// multiplies the last and the next-to-last number and returns the product
    fn divide(mut self) {
        let last = self.stack.pop().unwrap();
        let next_to_last = self.stack.pop().unwrap();

        self.push(next_to_last / last);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs_as_non_operable() {
        // Given
        let evaluator = Evaluator::new();

        let expected: Vec<f32> = vec![];

        // When
        let actual = evaluator.stack;

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_adds() {
        // Given
        let evaluator = Evaluator {
            stack: vec![1.0, 2.0],
        };

        let expected = 3.0;

        // When
        evaluator.add();
        let actual = evaluator.stack.pop().unwrap();

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_subtracts() {
        // Given
        let evaluator = Evaluator {
            stack: vec![1.0, 2.0],
        };

        let expected = -1.0;

        // When
        evaluator.subtract();
        let actual = evaluator.stack.pop().unwrap();

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_multiplies() {
        // Given
        let evaluator = Evaluator {
            stack: vec![1.0, 2.0],
        };

        let expected = 2.0;

        // When
        evaluator.multiply();
        let actual = evaluator.stack.pop().unwrap();

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_divides() {
        // Given
        let evaluator = Evaluator {
            stack: vec![1.0, 2.0],
        };

        let expected = 0.5;

        // When
        evaluator.divide();
        let actual = evaluator.stack.pop().unwrap();

        // Then
        assert_eq!(expected, actual);
    }
}
