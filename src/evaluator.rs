struct Evaluator {
    // Treated like a stack
    stack: Vec<f32>,
}

impl Evaluator {
    fn new() -> Evaluator {
        Evaluator { stack: vec![] }
    }

    fn evaluate(mut self, instructions: &str) -> f32 {
        // split the instructions string at every space
        // iterate through the set of instructions
        // match on the optional for parsing to `f32`
        // if it can match,
        //      then push onto the stack
        // if it cannot, try matching on the operators set { "+", "-", "*", "/" }
        // if it can match,
        //      then pop two from the stack and evaluate them with the operator
        // otherwise `panic`
        4.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_constructs() {
        // Given
        let evaluator = Evaluator::new();

        let expected = vec![];

        // When
        let actual = evaluator.stack;

        // Then
        assert_eq!(expected, actual);
    }
}
