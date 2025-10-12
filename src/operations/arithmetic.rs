use crate::types::bigint::TubularBigInt;
use crate::types::error::{Result, ExecError, InitError};
use crate::interpreter::droplet::Droplet;
use crate::interpreter::stack::DataStack;

/// Arithmetic and stack operations for Tubular programs
pub struct ArithmeticOperations;

impl ArithmeticOperations {
    /// Process numeric literal (0-9) and set droplet value
    pub fn process_numeric_literal(droplet: &mut Droplet, literal: char) -> Result<()> {
        if !('0'..='9').contains(&literal) {
            return Err(InitError::InvalidCharacter(literal, droplet.position).into());
        }

        let value = literal.to_digit(10).unwrap() as i64;
        droplet.set_value(TubularBigInt::new(value));
        Ok(())
    }

    /// Process stack operations (push, pop, duplicate)
    pub fn process_stack_operation(
        operation: char,
        droplet: &mut Droplet,
        stack: &mut DataStack,
    ) -> Result<()> {
        match operation {
            ':' => Self::push(droplet, stack),
            ';' => Self::pop(droplet, stack),
            'd' => Self::duplicate(stack),
            'A' => Self::add(droplet, stack),
            'S' => Self::subtract(droplet, stack),
            'M' => Self::multiply(droplet, stack),
            'D' => Self::divide(droplet, stack),
            '=' => Self::equals(droplet, stack),
            '<' => Self::less_than(droplet, stack),
            '>' => Self::greater_than(droplet, stack),
            '%' => Self::modulo(droplet, stack),
            '+' => Self::increment(droplet),
            '~' => Self::decrement(droplet),
            _ => Err(ExecError::InvalidOperation(operation).into()),
        }
    }

    /// Push (:) - Push droplet value to stack
    fn push(droplet: &Droplet, stack: &mut DataStack) -> Result<()> {
        stack.push(droplet.value.clone());
        Ok(())
    }

    /// Pop (;) - Pop value from stack to droplet
    fn pop(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        droplet.set_value(stack.pop_or_zero());
        Ok(())
    }

    /// Duplicate (d) - Duplicate top stack value
    fn duplicate(stack: &mut DataStack) -> Result<()> {
        if stack.is_empty() {
            stack.push(TubularBigInt::zero());
        } else {
            let top = stack.peek();
            stack.push(top);
        }
        Ok(())
    }

    /// Add (A) - Pop two values, add them, push result to droplet
    fn add(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = a + b;
        droplet.set_value(result);
        Ok(())
    }

    /// Subtract (S) - Pop two values, subtract, push result to droplet
    fn subtract(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = a - b;
        droplet.set_value(result);
        Ok(())
    }

    /// Multiply (M) - Pop two values, multiply, push result to droplet
    fn multiply(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = a * b;
        droplet.set_value(result);
        Ok(())
    }

    /// Divide (D) - Pop two values, divide, push result to droplet (division by zero = 0)
    fn divide(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = if b.is_zero() {
            TubularBigInt::zero()
        } else {
            a / b
        };
        droplet.set_value(result);
        Ok(())
    }

    /// Equals (=) - Pop two values, compare equality, push result to droplet
    fn equals(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = if a == b {
            TubularBigInt::one()
        } else {
            TubularBigInt::zero()
        };
        droplet.set_value(result);
        Ok(())
    }

    /// Less Than (<) - Pop two values, compare, push result to droplet
    fn less_than(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = if a < b {
            TubularBigInt::one()
        } else {
            TubularBigInt::zero()
        };
        droplet.set_value(result);
        Ok(())
    }

    /// Greater Than (>) - Pop two values, compare, push result to droplet
    fn greater_than(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = if a > b {
            TubularBigInt::one()
        } else {
            TubularBigInt::zero()
        };
        droplet.set_value(result);
        Ok(())
    }

    /// Modulo (%) - Pop two values, compute modulo, push result to droplet (modulo by zero = 0)
    fn modulo(droplet: &mut Droplet, stack: &mut DataStack) -> Result<()> {
        let b = stack.pop_or_zero();
        let a = stack.pop_or_zero();
        let result = if b.is_zero() {
            TubularBigInt::zero()
        } else {
            a % b
        };
        droplet.set_value(result);
        Ok(())
    }

    /// Increment (+) - Increment droplet value by 1
    fn increment(droplet: &mut Droplet) -> Result<()> {
        droplet.set_value(droplet.value.clone() + TubularBigInt::new(1));
        Ok(())
    }

    /// Decrement (~) - Decrement droplet value by 1
    fn decrement(droplet: &mut Droplet) -> Result<()> {
        droplet.set_value(droplet.value.clone() - TubularBigInt::new(1));
        Ok(())
    }

    /// Check if a character is an arithmetic operation
    pub fn is_arithmetic_operation(symbol: char) -> bool {
        matches!(symbol,
            ':' | ';' | 'd' | 'A' | 'S' | 'M' | 'D' | '=' | '<' | '>' | '%' | '+' | '~'
        )
    }

    /// Check if a character is a numeric literal
    pub fn is_numeric_literal(symbol: char) -> bool {
        ('0'..='9').contains(&symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::coordinate::Coordinate;
    use crate::types::direction::Direction;

    fn create_test_droplet(id: u64, value: i64) -> Droplet {
        Droplet::new(id, Coordinate::new(0, 0), Direction::Down)
            .with_value(TubularBigInt::new(value))
    }

    #[test]
    fn test_numeric_literals() {
        let mut droplet = create_test_droplet(0, 0);

        ArithmeticOperations::process_numeric_literal(&mut droplet, '5').unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(5));

        ArithmeticOperations::process_numeric_literal(&mut droplet, '9').unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(9));
    }

    #[test]
    fn test_stack_push_pop() {
        let mut droplet = create_test_droplet(0, 42);
        let mut stack = DataStack::new();

        // Push value
        ArithmeticOperations::push(&droplet, &mut stack).unwrap();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.peek(), TubularBigInt::new(42));

        // Pop value
        droplet.set_value(TubularBigInt::zero());
        ArithmeticOperations::pop(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(42));
        assert_eq!(stack.depth(), 0);
    }

    #[test]
    fn test_stack_underflow() {
        let mut droplet = create_test_droplet(0, 0);
        let mut stack = DataStack::new();

        // Pop from empty stack should give 0
        ArithmeticOperations::pop(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::zero());

        // Duplicate on empty stack should push 0
        ArithmeticOperations::duplicate(&mut stack).unwrap();
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.peek(), TubularBigInt::zero());
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut droplet = create_test_droplet(0, 0);
        let mut stack = DataStack::new();

        // Test addition: 5 + 3 = 8
        stack.push(TubularBigInt::new(5));
        stack.push(TubularBigInt::new(3));
        ArithmeticOperations::add(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(8));

        // Test subtraction: 10 - 4 = 6
        stack.push(TubularBigInt::new(10));
        stack.push(TubularBigInt::new(4));
        ArithmeticOperations::subtract(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(6));

        // Test multiplication: 3 * 7 = 21
        stack.push(TubularBigInt::new(3));
        stack.push(TubularBigInt::new(7));
        ArithmeticOperations::multiply(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(21));
    }

    #[test]
    fn test_division_by_zero() {
        let mut droplet = create_test_droplet(0, 0);
        let mut stack = DataStack::new();

        // 5 / 0 = 0 (division by zero protection)
        stack.push(TubularBigInt::new(5));
        stack.push(TubularBigInt::zero());
        ArithmeticOperations::divide(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::zero());
    }

    #[test]
    fn test_comparison_operations() {
        let mut droplet = create_test_droplet(0, 0);
        let mut stack = DataStack::new();

        // 5 < 10 = 1 (true)
        stack.push(TubularBigInt::new(5));
        stack.push(TubularBigInt::new(10));
        ArithmeticOperations::less_than(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::one());

        // 10 > 5 = 1 (true)
        stack.push(TubularBigInt::new(10));
        stack.push(TubularBigInt::new(5));
        ArithmeticOperations::greater_than(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::one());

        // 7 = 7 = 1 (true)
        stack.push(TubularBigInt::new(7));
        stack.push(TubularBigInt::new(7));
        ArithmeticOperations::equals(&mut droplet, &mut stack).unwrap();
        assert_eq!(droplet.value, TubularBigInt::one());
    }

    #[test]
    fn test_increment_decrement() {
        let mut droplet = create_test_droplet(0, 5);

        ArithmeticOperations::increment(&mut droplet).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(6));

        ArithmeticOperations::decrement(&mut droplet).unwrap();
        assert_eq!(droplet.value, TubularBigInt::new(5));
    }
}