//! Unit tests for the DataStack type

use tubular::interpreter::stack::DataStack;
use tubular::types::bigint::TubularBigInt;
use proptest::prelude::*;

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn test_data_stack_new() {
        let stack = DataStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.max_depth_reached(), 0);
    }

    #[test]
    fn test_data_stack_with_capacity() {
        let stack = DataStack::with_capacity(10);
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        // Note: We can't directly test capacity, but we can test that it works
    }

    #[test]
    fn test_data_stack_push() {
        let mut stack = DataStack::new();
        let value = TubularBigInt::new(42);

        stack.push(value.clone());
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.max_depth_reached(), 1);
        assert_eq!(stack.peek(), value);
    }

    #[test]
    fn test_data_stack_push_multiple() {
        let mut stack = DataStack::new();

        for i in 1..=5 {
            stack.push(TubularBigInt::new(i));
        }

        assert_eq!(stack.len(), 5);
        assert_eq!(stack.depth(), 5);
        assert_eq!(stack.max_depth_reached(), 5);
        assert_eq!(stack.peek(), TubularBigInt::new(5));
    }

    #[test]
    fn test_data_stack_pop() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(42));

        let popped = stack.pop();
        assert_eq!(popped.to_i64(), Some(42));
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_data_stack_pop_empty() {
        let mut stack = DataStack::new();

        let popped = stack.pop();
        assert_eq!(popped, TubularBigInt::zero());
        assert!(stack.is_empty());
    }

    #[test]
    fn test_data_stack_pop_multiple() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));

        assert_eq!(stack.pop().to_i64(), Some(3));
        assert_eq!(stack.pop().to_i64(), Some(2));
        assert_eq!(stack.pop().to_i64(), Some(1));
        assert_eq!(stack.pop(), TubularBigInt::zero()); // Empty now
    }

    #[test]
    fn test_data_stack_peek() {
        let mut stack = DataStack::new();

        // Empty stack should return zero
        assert_eq!(stack.peek(), TubularBigInt::zero());

        stack.push(TubularBigInt::new(42));
        assert_eq!(stack.peek(), TubularBigInt::new(42));

        stack.push(TubularBigInt::new(100));
        assert_eq!(stack.peek(), TubularBigInt::new(100));

        // Peek shouldn't modify the stack
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), TubularBigInt::new(100));
    }

    #[test]
    fn test_data_stack_peek_depth() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));
        stack.push(TubularBigInt::new(4));
        stack.push(TubularBigInt::new(5));

        assert_eq!(stack.peek_depth(0), TubularBigInt::new(5)); // Top
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(4));
        assert_eq!(stack.peek_depth(2), TubularBigInt::new(3));
        assert_eq!(stack.peek_depth(4), TubularBigInt::new(1)); // Bottom

        // Out of bounds should return zero
        assert_eq!(stack.peek_depth(5), TubularBigInt::zero());
        assert_eq!(stack.peek_depth(10), TubularBigInt::zero());

        // Empty stack
        let empty_stack = DataStack::new();
        assert_eq!(empty_stack.peek_depth(0), TubularBigInt::zero());
    }

    #[test]
    fn test_data_stack_clear() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));

        assert_eq!(stack.len(), 3);

        stack.clear();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        // max_depth should still track the peak
        assert_eq!(stack.max_depth_reached(), 3);
    }

    #[test]
    fn test_data_stack_truncate() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));
        stack.push(TubularBigInt::new(4));
        stack.push(TubularBigInt::new(5));

        stack.truncate(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), TubularBigInt::new(3));

        // Truncate to larger size (should do nothing)
        stack.truncate(10);
        assert_eq!(stack.len(), 3);

        // Truncate to zero
        stack.truncate(0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_data_stack_swap_top_two() {
        let mut stack = DataStack::new();

        // Empty stack
        assert!(!stack.swap_top_two());

        // One element
        stack.push(TubularBigInt::new(1));
        assert!(!stack.swap_top_two());
        assert_eq!(stack.len(), 1);

        // Two elements
        stack.push(TubularBigInt::new(2));
        assert!(stack.swap_top_two());
        assert_eq!(stack.peek(), TubularBigInt::new(1));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(2));

        // More elements
        stack.push(TubularBigInt::new(3));
        assert!(stack.swap_top_two());
        assert_eq!(stack.peek(), TubularBigInt::new(2));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(3));
        assert_eq!(stack.peek_depth(2), TubularBigInt::new(1));
    }

    #[test]
    fn test_data_stack_duplicate() {
        let mut stack = DataStack::new();

        // Empty stack
        assert!(!stack.duplicate());
        assert!(stack.is_empty());

        // One element
        stack.push(TubularBigInt::new(42));
        assert!(stack.duplicate());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), TubularBigInt::new(42));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(42));

        // Multiple elements
        stack.push(TubularBigInt::new(100));
        assert!(stack.duplicate());
        assert_eq!(stack.len(), 4);
        assert_eq!(stack.peek(), TubularBigInt::new(100));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(100));
    }

    #[test]
    fn test_data_stack_pop_n() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));
        stack.push(TubularBigInt::new(4));
        stack.push(TubularBigInt::new(5));

        let popped = stack.pop_n(3);
        assert_eq!(popped.len(), 3);
        assert_eq!(popped[0].to_i64(), Some(5));
        assert_eq!(popped[1].to_i64(), Some(4));
        assert_eq!(popped[2].to_i64(), Some(3));
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), TubularBigInt::new(2));

        // Pop more than available
        let more_popped = stack.pop_n(10);
        assert_eq!(more_popped.len(), 10);
        for val in &more_popped[0..2] {
            assert!(!val.is_zero());
        }
        for val in &more_popped[2..] {
            assert_eq!(val, &TubularBigInt::zero());
        }
        assert!(stack.is_empty());
    }

    #[test]
    fn test_data_stack_push_n() {
        let mut stack = DataStack::new();
        let values = vec![
            TubularBigInt::new(1),
            TubularBigInt::new(2),
            TubularBigInt::new(3),
        ];

        stack.push_n(values.clone());
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), TubularBigInt::new(3));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(2));
        assert_eq!(stack.peek_depth(2), TubularBigInt::new(1));

        // Empty vector
        stack.push_n(Vec::new());
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn test_data_stack_get() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));

        assert_eq!(stack.get(0), Some(&TubularBigInt::new(1)));
        assert_eq!(stack.get(1), Some(&TubularBigInt::new(2)));
        assert_eq!(stack.get(2), Some(&TubularBigInt::new(3)));
        assert_eq!(stack.get(3), None);
    }

    #[test]
    fn test_data_stack_get_from_top() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));

        assert_eq!(stack.get_from_top(0), Some(&TubularBigInt::new(3)));
        assert_eq!(stack.get_from_top(1), Some(&TubularBigInt::new(2)));
        assert_eq!(stack.get_from_top(2), Some(&TubularBigInt::new(1)));
        assert_eq!(stack.get_from_top(3), None);
    }

    #[test]
    fn test_data_stack_as_slice() {
        let mut stack = DataStack::new();
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));

        let slice = stack.as_slice();
        assert_eq!(slice.len(), 3);
        assert_eq!(slice[0], TubularBigInt::new(1));
        assert_eq!(slice[1], TubularBigInt::new(2));
        assert_eq!(slice[2], TubularBigInt::new(3));
    }

    #[test]
    fn test_data_stack_is_within_limit() {
        let mut stack = DataStack::new();

        assert!(stack.is_within_limit(0));
        assert!(stack.is_within_limit(10));

        stack.push(TubularBigInt::new(1));
        assert!(stack.is_within_limit(1));
        assert!(stack.is_within_limit(10));
        assert!(!stack.is_within_limit(0));

        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));
        assert!(stack.is_within_limit(3));
        assert!(!stack.is_within_limit(2));
    }

    #[test]
    fn test_data_stack_max_depth_tracking() {
        let mut stack = DataStack::new();

        // Push some values
        for i in 1..=5 {
            stack.push(TubularBigInt::new(i));
        }
        assert_eq!(stack.max_depth_reached(), 5);

        // Pop some values
        for _ in 0..3 {
            stack.pop();
        }
        assert_eq!(stack.len(), 2);
        // Max depth should still be 5
        assert_eq!(stack.max_depth_reached(), 5);

        // Push more values (but don't exceed previous max)
        stack.push(TubularBigInt::new(6));
        stack.push(TubularBigInt::new(7));
        assert_eq!(stack.len(), 4);
        assert_eq!(stack.max_depth_reached(), 5);

        // Exceed previous max
        stack.push(TubularBigInt::new(8));
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.max_depth_reached(), 5);

        stack.push(TubularBigInt::new(9));
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.max_depth_reached(), 6);
    }

    #[test]
    fn test_data_stack_default() {
        let stack = DataStack::default();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.max_depth_reached(), 0);
    }

    #[test]
    fn test_data_stack_display() {
        let mut stack = DataStack::new();

        assert_eq!(format!("{}", stack), "[]");

        stack.push(TubularBigInt::new(1));
        assert_eq!(format!("{}", stack), "[1]");

        stack.push(TubularBigInt::new(2));
        stack.push(TubularBigInt::new(3));
        let display = format!("{}", stack);
        assert!(display.contains("["));
        assert!(display.contains("]"));
        assert!(display.contains("1"));
        assert!(display.contains("2"));
        assert!(display.contains("3"));
    }

    #[test]
    fn test_data_stack_from_vec_bigint() {
        let values = vec![
            TubularBigInt::new(1),
            TubularBigInt::new(2),
            TubularBigInt::new(3),
        ];

        let stack: DataStack = values.clone().into();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.max_depth_reached(), 3);
        assert_eq!(stack.as_slice(), &values[..]);
    }

    #[test]
    fn test_data_stack_from_vec_i64() {
        let values = vec![1, 2, 3, 4, 5];

        let stack: DataStack = values.clone().into();
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.max_depth_reached(), 5);
        assert_eq!(stack.peek(), TubularBigInt::new(5));
        assert_eq!(stack.get(0), Some(&TubularBigInt::new(1)));
    }

    #[test]
    fn test_data_stack_large_values() {
        let mut stack = DataStack::new();
        let large_value = TubularBigInt::new(i64::MAX);

        stack.push(large_value.clone());
        assert_eq!(stack.peek(), large_value);

        stack.push(TubularBigInt::new(i64::MIN));
        assert_eq!(stack.peek(), TubularBigInt::new(i64::MIN));
    }

    #[test]
    fn test_data_stack_edge_cases() {
        let mut stack = DataStack::new();

        // Test peek_depth 0 on empty stack
        assert_eq!(stack.peek_depth(0), TubularBigInt::zero());

        // Test swap_top_two with exactly two elements
        stack.push(TubularBigInt::new(1));
        stack.push(TubularBigInt::new(2));
        assert!(stack.swap_top_two());
        assert_eq!(stack.peek(), TubularBigInt::new(1));
        assert_eq!(stack.peek_depth(1), TubularBigInt::new(2));

        // Test truncate to current size
        stack.truncate(2);
        assert_eq!(stack.len(), 2);

        // Test pop_n exactly the number available
        let popped = stack.pop_n(2);
        assert_eq!(popped.len(), 2);
        assert!(stack.is_empty());
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_stack_push_pop_consistency(
        values in prop::collection::vec(any::<i64>(), 0..100)
    ) {
        let mut stack = DataStack::new();
        let bigint_values: Vec<TubularBigInt> = values
            .iter()
            .map(|&v| TubularBigInt::new(v))
            .collect();

        // Push all values
        for value in &bigint_values {
            stack.push(value.clone());
        }

        // Pop all values and verify order
        let mut popped_values = Vec::new();
        while !stack.is_empty() {
            popped_values.push(stack.pop());
        }

        // Should be reverse of input
        let expected: Vec<TubularBigInt> = bigint_values.iter().rev().cloned().collect();
        assert_eq!(popped_values, expected);
    }

    #[test]
    fn test_stack_peek_depth_properties(
        values in prop::collection::vec(any::<i64>(), 1..50)
    ) {
        let mut stack = DataStack::new();
        let bigint_values: Vec<TubularBigInt> = values
            .iter()
            .map(|&v| TubularBigInt::new(v))
            .collect();

        for value in &bigint_values {
            stack.push(value.clone());
        }

        // Test all valid depths
        for depth in 0..stack.len() {
            let peeked = stack.peek_depth(depth);
            let expected = bigint_values[bigint_values.len() - 1 - depth].clone();
            assert_eq!(peeked, expected);
        }

        // Test invalid depths
        for depth in stack.len()..stack.len() + 5 {
            assert_eq!(stack.peek_depth(depth), TubularBigInt::zero());
        }
    }

    #[test]
    fn test_stack_max_depth_tracking(
        push_operations in prop::collection::vec(
            prop::bool::weighted(0.7), // 70% push, 30% pop
            0..200
        )
    ) {
        let mut stack = DataStack::new();
        let mut max_seen = 0;

        for should_push in push_operations {
            if should_push {
                stack.push(TubularBigInt::new(1));
                max_seen = max_seen.max(stack.len());
            } else {
                stack.pop();
            }
        }

        assert_eq!(stack.max_depth_reached(), max_seen);
    }

    #[test]
    fn test_stack_swap_properties(
        values in prop::collection::vec(any::<i64>(), 2..10)
    ) {
        let mut stack = DataStack::new();
        for &value in &values {
            stack.push(TubularBigInt::new(value));
        }

        let original_len = stack.len();
        let original_top = stack.peek();
        let original_second = stack.peek_depth(1);

        // Swap top two
        let swap_success = stack.swap_top_two();

        if original_len >= 2 {
            assert!(swap_success);
            assert_eq!(stack.peek(), original_second);
            assert_eq!(stack.peek_depth(1), original_top);
            assert_eq!(stack.len(), original_len);
        } else {
            assert!(!swap_success);
            assert_eq!(stack.len(), original_len);
        }
    }

    #[test]
    fn test_stack_duplicate_properties(
        values in prop::collection::vec(any::<i64>(), 0..10)
    ) {
        let mut stack = DataStack::new();
        for &value in &values {
            stack.push(TubularBigInt::new(value));
        }

        let original_len = stack.len();
        let original_top = stack.peek();

        let duplicate_success = stack.duplicate();

        if original_len > 0 {
            assert!(duplicate_success);
            assert_eq!(stack.len(), original_len + 1);
            assert_eq!(stack.peek(), original_top);
            assert_eq!(stack.peek_depth(1), original_top);
        } else {
            assert!(!duplicate_success);
            assert_eq!(stack.len(), original_len);
        }
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_stack_operations() {
        let mut stack = DataStack::new();
        let start = Instant::now();

        for i in 0..1_000_000 {
            stack.push(TubularBigInt::new(i));
        }

        let push_duration = start.elapsed();
        println!("Stack push (1M): {:?}", push_duration);
        assert!(push_duration.as_millis() < 300);

        let start = Instant::now();
        while !stack.is_empty() {
            stack.pop();
        }
        let pop_duration = start.elapsed();
        println!("Stack pop (1M): {:?}", pop_duration);
        assert!(pop_duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_stack_peek_operations() {
        let mut stack = DataStack::new();
        for i in 0..100_000 {
            stack.push(TubularBigInt::new(i));
        }

        let start = Instant::now();
        for i in 0..1_000_000 {
            let _peek = stack.peek();
            let _depth_peek = stack.peek_depth(i % 100_000);
        }
        let duration = start.elapsed();
        println!("Stack peek operations (2M): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn benchmark_stack_bulk_operations() {
        let values: Vec<TubularBigInt> = (0..10_000)
            .map(TubularBigInt::new)
            .collect();

        let mut stack = DataStack::new();
        let start = Instant::now();
        stack.push_n(values.clone());
        let push_n_duration = start.elapsed();
        println!("Stack push_n (10K): {:?}", push_n_duration);

        let start = Instant::now();
        let _popped = stack.pop_n(10_000);
        let pop_n_duration = start.elapsed();
        println!("Stack pop_n (10K): {:?}", pop_n_duration);

        assert!(push_n_duration.as_millis() < 50);
        assert!(pop_n_duration.as_millis() < 50);
    }

    #[test]
    fn benchmark_stack_access_methods() {
        let mut stack = DataStack::new();
        for i in 0..100_000 {
            stack.push(TubularBigInt::new(i));
        }

        let start = Instant::now();
        for i in 0..1_000_000 {
            let _get = stack.get(i % 100_000);
            let _get_from_top = stack.get_from_top(i % 100_000);
            let _slice = stack.as_slice();
        }
        let duration = start.elapsed();
        println!("Stack access methods (3M): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_stack_complex_operations() {
        let mut stack = DataStack::new();
        for i in 0..10_000 {
            stack.push(TubularBigInt::new(i));
        }

        let start = Instant::now();
        for _ in 0..100_000 {
            stack.swap_top_two();
            stack.duplicate();
            stack.truncate(stack.len() / 2);
            // Rebuild stack
            while stack.len() < 10_000 {
                stack.push(TubularBigInt::new(42));
            }
        }
        let duration = start.elapsed();
        println!("Stack complex operations (400K): {:?}", duration);
        assert!(duration.as_millis() < 1000);
    }
}