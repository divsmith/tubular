//! Property-based tests for stack operations
//! Tests invariants and properties of stack operations using proptest

use proptest::prelude::*;
use tubular::operations::arithmetic::ArithmeticOperations;
use tubular::interpreter::stack::DataStack;
use tubular::interpreter::droplet::Droplet;
use tubular::types::coordinate::Coordinate;
use tubular::types::direction::Direction;
use tubular::types::bigint::TubularBigInt;

fn create_test_droplet(id: u64, value: i64) -> Droplet {
    let mut droplet = Droplet::new(id, Coordinate::new(0, 0), Direction::Down);
    droplet.set_value(TubularBigInt::new(value));
    droplet
}

#[cfg(test)]
mod stack_properties {
    use super::*;

    #[test]
    fn test_push_pop_property() {
        // Property: push followed by pop should return the original value
        proptest!(|(value in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, value);

            // Push the value
            ArithmeticOperations::push(&droplet, &mut stack).unwrap();
            assert_eq!(stack.depth(), 1);
            assert_eq!(stack.peek(), TubularBigInt::new(value));

            // Pop the value
            droplet.set_value(TubularBigInt::new(999)); // Change droplet value
            ArithmeticOperations::pop(&mut droplet, &mut stack).unwrap();

            // Should get original value back
            assert_eq!(droplet.value, TubularBigInt::new(value));
            assert_eq!(stack.depth(), 0);
        });
    }

    #[test]
    fn test_duplicate_property() {
        // Property: duplicate should create two copies of the top value
        proptest!(|(value in any::<i64>())| {
            let mut stack = DataStack::new();
            let droplet = create_test_droplet(0, value);

            // Push a value
            stack.push(TubularBigInt::new(value));
            assert_eq!(stack.depth(), 1);

            // Duplicate it
            ArithmeticOperations::duplicate(&mut stack).unwrap();

            // Should now have two identical values
            assert_eq!(stack.depth(), 2);
            assert_eq!(stack.peek(), TubularBigInt::new(value));

            // Pop both values and verify they're the same
            let first = stack.pop_or_zero();
            let second = stack.pop_or_zero();
            assert_eq!(first, TubularBigInt::new(value));
            assert_eq!(second, TubularBigInt::new(value));
        });
    }

    #[test]
    fn test_addition_commutativity() {
        // Property: A + B = B + A for addition
        proptest!(|(a in any::<i64>(), b in any::<i64>())| {
            let mut stack1 = DataStack::new();
            let mut stack2 = DataStack::new();
            let mut droplet1 = create_test_droplet(0, 0);
            let mut droplet2 = create_test_droplet(1, 0);

            // Stack 1: a then b
            stack1.push(TubularBigInt::new(a));
            stack1.push(TubularBigInt::new(b));
            ArithmeticOperations::add(&mut droplet1, &mut stack1).unwrap();
            let result1 = droplet1.value.clone();

            // Stack 2: b then a
            stack2.push(TubularBigInt::new(b));
            stack2.push(TubularBigInt::new(a));
            ArithmeticOperations::add(&mut droplet2, &mut stack2).unwrap();
            let result2 = droplet2.value.clone();

            assert_eq!(result1, result2);
        });
    }

    #[test]
    fn test_multiplication_commutativity() {
        // Property: A * B = B * A for multiplication
        proptest!(|(a in any::<i64>(), b in any::<i64>())| {
            let mut stack1 = DataStack::new();
            let mut stack2 = DataStack::new();
            let mut droplet1 = create_test_droplet(0, 0);
            let mut droplet2 = create_test_droplet(1, 0);

            // Stack 1: a then b
            stack1.push(TubularBigInt::new(a));
            stack1.push(TubularBigInt::new(b));
            ArithmeticOperations::multiply(&mut droplet1, &mut stack1).unwrap();
            let result1 = droplet1.value.clone();

            // Stack 2: b then a
            stack2.push(TubularBigInt::new(b));
            stack2.push(TubularBigInt::new(a));
            ArithmeticOperations::multiply(&mut droplet2, &mut stack2).unwrap();
            let result2 = droplet2.value.clone();

            assert_eq!(result1, result2);
        });
    }

    #[test]
    fn test_associativity_property() {
        // Property: (A + B) + C = A + (B + C) for addition
        proptest!(|(a in any::<i64>(), b in any::<i64>(), c in any::<i64>())| {
            let mut stack1 = DataStack::new();
            let mut stack2 = DataStack::new();
            let mut droplet1 = create_test_droplet(0, 0);
            let mut droplet2 = create_test_droplet(1, 0);

            // Left associative: (A + B) + C
            stack1.push(TubularBigInt::new(a));
            stack1.push(TubularBigInt::new(b));
            ArithmeticOperations::add(&mut droplet1, &mut stack1).unwrap();
            stack1.push(droplet1.value.clone());
            stack1.push(TubularBigInt::new(c));
            ArithmeticOperations::add(&mut droplet1, &mut stack1).unwrap();
            let left_result = droplet1.value.clone();

            // Right associative: A + (B + C)
            stack2.push(TubularBigInt::new(b));
            stack2.push(TubularBigInt::new(c));
            ArithmeticOperations::add(&mut droplet2, &mut stack2).unwrap();
            stack2.push(TubularBigInt::new(a));
            stack2.push(droplet2.value.clone());
            ArithmeticOperations::add(&mut droplet2, &mut stack2).unwrap();
            let right_result = droplet2.value.clone();

            assert_eq!(left_result, right_result);
        });
    }

    #[test]
    fn test_identity_property() {
        // Property: A + 0 = A and A * 1 = A
        proptest!(|(value in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, 0);

            // Test addition identity: A + 0 = A
            stack.push(TubularBigInt::new(value));
            stack.push(TubularBigInt::new(0));
            ArithmeticOperations::add(&mut droplet, &mut stack).unwrap();
            assert_eq!(droplet.value, TubularBigInt::new(value));

            // Test multiplication identity: A * 1 = A
            stack.clear();
            stack.push(TubularBigInt::new(value));
            stack.push(TubularBigInt::new(1));
            ArithmeticOperations::multiply(&mut droplet, &mut stack).unwrap();
            assert_eq!(droplet.value, TubularBigInt::new(value));
        });
    }

    #[test]
    fn test_comparison_properties() {
        proptest!(|(a in any::<i64>(), b in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet_eq = create_test_droplet(0, 0);
            let mut droplet_lt = create_test_droplet(1, 0);
            let mut droplet_gt = create_test_droplet(2, 0);

            // Test equality property: A = A should be true (1)
            stack.push(TubularBigInt::new(a));
            stack.push(TubularBigInt::new(a));
            ArithmeticOperations::equals(&mut droplet_eq, &mut stack).unwrap();
            assert_eq!(droplet_eq.value, TubularBigInt::new(1));

            // Test less than property consistency
            stack.clear();
            stack.push(TubularBigInt::new(a));
            stack.push(TubularBigInt::new(b));
            ArithmeticOperations::less_than(&mut droplet_lt, &mut stack).unwrap();

            stack.clear();
            stack.push(TubularBigInt::new(b));
            stack.push(TubularBigInt::new(a));
            ArithmeticOperations::greater_than(&mut droplet_gt, &mut stack).unwrap();

            // A < B should equal B > A
            assert_eq!(droplet_lt.value, droplet_gt.value);
        });
    }

    #[test]
    fn test_stack_underflow_safety() {
        // Property: Stack operations should handle underflow gracefully
        proptest!(|(initial_values: Vec<i64>)| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, 0);

            // Push initial values
            for &value in &initial_values {
                stack.push(TubularBigInt::new(value));
            }

            // Try to pop one more value than was pushed
            for _ in 0..=initial_values.len() {
                ArithmeticOperations::pop(&mut droplet, &mut stack).unwrap();
            }

            // Should have droplet value as zero (safe default)
            assert_eq!(droplet.value, TubularBigInt::zero());
            assert_eq!(stack.depth(), 0);
        });
    }

    #[test]
    fn test_stack_depth_monotonicity() {
        // Property: Stack depth should never exceed number of pushes
        proptest!(|(operations: Vec<String>)| {
            let mut stack = DataStack::new();
            let mut max_depth = 0;
            let mut push_count = 0;

            for op in operations.iter().take(50) { // Limit test size
                match op.as_str() {
                    "push" => {
                        push_count += 1;
                        stack.push(TubularBigInt::new(42));
                    }
                    "pop" => {
                        stack.pop_or_zero();
                    }
                    "duplicate" => {
                        if stack.depth() > 0 {
                            ArithmeticOperations::duplicate(&mut stack).unwrap();
                        }
                    }
                    _ => continue,
                }
                max_depth = max_depth.max(stack.depth());

                // Stack depth should never exceed total pushes
                assert!(stack.depth() <= push_count);
            }

            // Maximum depth reached should not exceed pushes
            assert!(max_depth <= push_count);
        });
    }

    #[test]
    fn test_zero_division_safety() {
        // Property: Division by zero should return zero safely
        proptest!(|(numerator in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, 0);

            // Push numerator and zero divisor
            stack.push(TubularBigInt::new(numerator));
            stack.push(TubularBigInt::new(0));

            // Should not panic and should return zero
            ArithmeticOperations::divide(&mut droplet, &mut stack).unwrap();
            assert_eq!(droplet.value, TubularBigInt::zero());

            // Stack should be empty after operation
            assert_eq!(stack.depth(), 0);
        });
    }

    #[test]
    fn test_modulo_zero_safety() {
        // Property: Modulo by zero should return zero safely
        proptest!(|(numerator in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, 0);

            // Push numerator and zero divisor
            stack.push(TubularBigInt::new(numerator));
            stack.push(TubularBigInt::new(0));

            // Should not panic and should return zero
            ArithmeticOperations::modulo(&mut droplet, &mut stack).unwrap();
            assert_eq!(droplet.value, TubularBigInt::zero());

            // Stack should be empty after operation
            assert_eq!(stack.depth(), 0);
        });
    }

    #[test]
    fn test_subtraction_inverse_property() {
        // Property: (A - B) + B = A (when no overflow/underflow)
        proptest!(|(a in any::<i64>(), b in any::<i64>())| {
            let mut stack = DataStack::new();
            let mut droplet = create_test_droplet(0, 0);

            // First: A - B
            stack.push(TubularBigInt::new(a));
            stack.push(TubularBigInt::new(b));
            ArithmeticOperations::subtract(&mut droplet, &mut stack).unwrap();
            let diff = droplet.value.clone();

            // Then: (A - B) + B
            stack.push(diff);
            stack.push(TubularBigInt::new(b));
            ArithmeticOperations::add(&mut droplet, &mut stack).unwrap();

            // Should equal original A (modulo overflow behavior)
            let expected = TubularBigInt::new(a);
            assert_eq!(droplet.value, expected);
        });
    }
}