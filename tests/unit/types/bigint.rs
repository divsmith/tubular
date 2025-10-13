//! Unit tests for the TubularBigInt type

use tubular::types::bigint::TubularBigInt;
use num_bigint::BigInt;
use num_traits::{Zero, Signed, ToPrimitive};
use proptest::prelude::*;

#[cfg(test)]
mod bigint_tests {
    use super::*;

    #[test]
    fn test_tubular_bigint_new() {
        let bigint = TubularBigInt::new(42);
        assert_eq!(bigint.to_i64(), Some(42));
    }

    #[test]
    fn test_tubular_bigint_zero() {
        let zero = TubularBigInt::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.to_i64(), Some(0));
    }

    #[test]
    fn test_tubular_bigint_one() {
        let one = TubularBigInt::one();
        assert!(!one.is_zero());
        assert!(one.is_positive());
        assert_eq!(one.to_i64(), Some(1));
    }

    #[test]
    fn test_tubular_bigint_from_bigint() {
        let big = BigInt::from(12345678901234567890i128);
        let tubular = TubularBigInt::from_bigint(big.clone());
        assert_eq!(tubular.as_bigint(), &big);
    }

    #[test]
    fn test_tubular_bigint_into_bigint() {
        let tubular = TubularBigInt::new(123);
        let big: BigInt = tubular.into_bigint();
        assert_eq!(big, BigInt::from(123));
    }

    #[test]
    fn test_tubular_bigint_as_bigint() {
        let tubular = TubularBigInt::new(456);
        let big_ref = tubular.as_bigint();
        assert_eq!(big_ref, &BigInt::from(456));
    }

    #[test]
    fn test_tubular_bigint_as_bigint_mut() {
        let mut tubular = TubularBigInt::new(100);
        {
            let big_mut = tubular.as_bigint_mut();
            *big_mut += 50;
        }
        assert_eq!(tubular.to_i64(), Some(150));
    }

    #[test]
    fn test_increment() {
        let mut bigint = TubularBigInt::new(5);
        bigint.increment();
        assert_eq!(bigint.to_i64(), Some(6));

        // Test chaining
        let mut bigint = TubularBigInt::new(0);
        bigint.increment().increment().increment();
        assert_eq!(bigint.to_i64(), Some(3));
    }

    #[test]
    fn test_decrement() {
        let mut bigint = TubularBigInt::new(5);
        bigint.decrement();
        assert_eq!(bigint.to_i64(), Some(4));

        // Test chaining
        let mut bigint = TubularBigInt::new(10);
        bigint.decrement().decrement().decrement();
        assert_eq!(bigint.to_i64(), Some(7));
    }

    #[test]
    fn test_is_zero() {
        assert!(TubularBigInt::zero().is_zero());
        assert!(TubularBigInt::new(0).is_zero());
        assert!(!TubularBigInt::new(1).is_zero());
        assert!(!TubularBigInt::new(-1).is_zero());
    }

    #[test]
    fn test_is_positive() {
        assert!(TubularBigInt::new(1).is_positive());
        assert!(TubularBigInt::new(100).is_positive());
        assert!(!TubularBigInt::new(0).is_positive());
        assert!(!TubularBigInt::new(-1).is_positive());
    }

    #[test]
    fn test_is_negative() {
        assert!(TubularBigInt::new(-1).is_negative());
        assert!(TubularBigInt::new(-100).is_negative());
        assert!(!TubularBigInt::new(0).is_negative());
        assert!(!TubularBigInt::new(1).is_negative());
    }

    #[test]
    fn test_abs() {
        assert_eq!(TubularBigInt::new(5).abs().to_i64(), Some(5));
        assert_eq!(TubularBigInt::new(-5).abs().to_i64(), Some(5));
        assert_eq!(TubularBigInt::new(0).abs().to_i64(), Some(0));
    }

    #[test]
    fn test_to_i64() {
        assert_eq!(TubularBigInt::new(42).to_i64(), Some(42));
        assert_eq!(TubularBigInt::new(-42).to_i64(), Some(-42));
        assert_eq!(TubularBigInt::zero().to_i64(), Some(0));

        // Test large number that fits in i64
        let big = TubularBigInt::from_bigint(BigInt::from(i64::MAX));
        assert_eq!(big.to_i64(), Some(i64::MAX));

        // Test number that doesn't fit in i64
        let too_big = TubularBigInt::from_bigint(BigInt::from(i64::MAX) + 1);
        assert_eq!(too_big.to_i64(), None);
    }

    #[test]
    fn test_to_usize() {
        assert_eq!(TubularBigInt::new(42).to_usize(), Some(42));
        assert_eq!(TubularBigInt::zero().to_usize(), Some(0));

        // Test negative number returns None
        assert_eq!(TubularBigInt::new(-1).to_usize(), None);

        // Test large number
        let big = TubularBigInt::from_bigint(BigInt::from(usize::MAX));
        assert_eq!(big.to_usize(), Some(usize::MAX));
    }

    #[test]
    fn test_to_char() {
        assert_eq!(TubularBigInt::new(65).to_char(), Some('A'));
        assert_eq!(TubularBigInt::new(97).to_char(), Some('a'));
        assert_eq!(TubularBigInt::new(0).to_char(), Some('\0'));
        assert_eq!(TubularBigInt::new(u32::MAX as i64).to_char(), None); // Too large
        assert_eq!(TubularBigInt::new(-1).to_char(), None); // Negative
    }

    #[test]
    fn test_from_char() {
        assert_eq!(TubularBigInt::from_char('A').to_i64(), Some(65));
        assert_eq!(TubularBigInt::from_char('a').to_i64(), Some(97));
        assert_eq!(TubularBigInt::from_char('\0').to_i64(), Some(0));
        assert_eq!(TubularBigInt::from_char('😀').to_i64(), Some(0x1F600)); // Emoji
    }

    #[test]
    fn test_safe_div() {
        let a = TubularBigInt::new(10);
        let b = TubularBigInt::new(2);
        let zero = TubularBigInt::zero();

        // Normal division
        assert_eq!(a.safe_div(&b).to_i64(), Some(5));

        // Division by zero returns zero
        assert_eq!(a.safe_div(&zero), TubularBigInt::zero());

        // Test with negative numbers
        let neg_a = TubularBigInt::new(-10);
        assert_eq!(neg_a.safe_div(&b).to_i64(), Some(-5));
    }

    #[test]
    fn test_safe_mod() {
        let a = TubularBigInt::new(10);
        let b = TubularBigInt::new(3);
        let zero = TubularBigInt::zero();

        // Normal modulo
        assert_eq!(a.safe_mod(&b).to_i64(), Some(1));

        // Modulo by zero returns zero
        assert_eq!(a.safe_mod(&zero), TubularBigInt::zero());

        // Test with negative numbers
        let neg_a = TubularBigInt::new(-10);
        assert_eq!(neg_a.safe_mod(&b).to_i64(), Some(-1)); // BigInt modulo preserves sign
    }

    #[test]
    fn test_default() {
        let default = TubularBigInt::default();
        assert!(default.is_zero());
        assert_eq!(default, TubularBigInt::zero());
    }

    #[test]
    fn test_from_i64() {
        let bigint: TubularBigInt = 42i64.into();
        assert_eq!(bigint.to_i64(), Some(42));

        let bigint: TubularBigInt = (-123).into();
        assert_eq!(bigint.to_i64(), Some(-123));
    }

    #[test]
    fn test_from_bigint_conversion() {
        let big = BigInt::from(12345);
        let tubular: TubularBigInt = big.clone().into();
        assert_eq!(tubular.as_bigint(), &big);
    }

    #[test]
    fn test_equality() {
        let a = TubularBigInt::new(42);
        let b = TubularBigInt::new(42);
        let c = TubularBigInt::new(43);

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_ordering() {
        let a = TubularBigInt::new(1);
        let b = TubularBigInt::new(2);
        let c = TubularBigInt::new(1);

        assert!(a < b);
        assert!(b > a);
        assert!(a <= b);
        assert!(b >= a);
        assert!(a <= c);
        assert!(a >= c);
    }

    #[test]
    fn test_display() {
        let bigint = TubularBigInt::new(12345);
        assert_eq!(format!("{}", bigint), "12345");

        let neg_bigint = TubularBigInt::new(-12345);
        assert_eq!(format!("{}", neg_bigint), "-12345");

        let zero = TubularBigInt::zero();
        assert_eq!(format!("{}", zero), "0");
    }

    #[test]
    fn test_debug() {
        let bigint = TubularBigInt::new(42);
        let debug_str = format!("{:?}", bigint);
        assert!(debug_str.contains("TubularBigInt"));
    }

    #[test]
    fn test_addition() {
        let a = TubularBigInt::new(5);
        let b = TubularBigInt::new(3);
        let result = a + b;
        assert_eq!(result.to_i64(), Some(8));
    }

    #[test]
    fn test_subtraction() {
        let a = TubularBigInt::new(5);
        let b = TubularBigInt::new(3);
        let result = a - b;
        assert_eq!(result.to_i64(), Some(2));
    }

    #[test]
    fn test_multiplication() {
        let a = TubularBigInt::new(5);
        let b = TubularBigInt::new(3);
        let result = a * b;
        assert_eq!(result.to_i64(), Some(15));
    }

    #[test]
    fn test_division_safe() {
        let a = TubularBigInt::new(10);
        let b = TubularBigInt::new(2);
        let result = a / b; // Uses safe_div
        assert_eq!(result.to_i64(), Some(5));

        let zero = TubularBigInt::zero();
        let result = a / zero; // Uses safe_div
        assert_eq!(result, TubularBigInt::zero());
    }

    #[test]
    fn test_modulo_safe() {
        let a = TubularBigInt::new(10);
        let b = TubularBigInt::new(3);
        let result = a % b; // Uses safe_mod
        assert_eq!(result.to_i64(), Some(1));

        let zero = TubularBigInt::zero();
        let result = a % zero; // Uses safe_mod
        assert_eq!(result, TubularBigInt::zero());
    }

    #[test]
    fn test_large_numbers() {
        let big = TubularBigInt::from_bigint(BigInt::from(1000000000000i64) * BigInt::from(1000000000000i64));
        assert!(big.to_i64().is_none()); // Too large for i64
        assert!(!big.is_zero());
        assert!(big.is_positive());
    }

    #[test]
    fn test_negative_operations() {
        let a = TubularBigInt::new(-5);
        let b = TubularBigInt::new(3);

        assert_eq!((a + b).to_i64(), Some(-2));
        assert_eq!((a - b).to_i64(), Some(-8));
        assert_eq!((a * b).to_i64(), Some(-15));
        assert_eq!((a / b).to_i64(), Some(-1)); // Integer division
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_addition_commutative(a in any::<i64>(), b in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);

        assert_eq!(tub_a.clone() + tub_b.clone(), tub_b + tub_a);
    }

    #[test]
    fn test_addition_associative(a in any::<i64>(), b in any::<i64>(), c in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);
        let tub_c = TubularBigInt::new(c);

        assert_eq!((tub_a.clone() + tub_b.clone()) + tub_c.clone(), tub_a + (tub_b + tub_c));
    }

    #[test]
    fn test_subtraction_properties(a in any::<i64>(), b in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);

        // a - b + b should equal a (for small values)
        let result = (tub_a.clone() - tub_b.clone()) + tub_b;
        // Note: This might not always hold due to large number arithmetic
        if let (Some(a_res), Some(result_res)) = (result.to_i64(), tub_a.to_i64()) {
            assert_eq!(a_res, result_res);
        }
    }

    #[test]
    fn test_multiplication_commutative(a in any::<i64>(), b in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);

        assert_eq!(tub_a.clone() * tub_b.clone(), tub_b * tub_a);
    }

    #[test]
    fn test_multiplication_distributive(a in any::<i64>(), b in any::<i64>(), c in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);
        let tub_c = TubularBigInt::new(c);

        assert_eq!(tub_a.clone() * (tub_b.clone() + tub_c.clone()), (tub_a * tub_b) + (tub_a * tub_c));
    }

    #[test]
    fn test_increment_decrement_roundtrip(value in any::<i64>()) {
        let mut tub = TubularBigInt::new(value);
        tub.increment();
        tub.decrement();
        assert_eq!(tub.to_i64(), Some(value));
    }

    #[test]
    fn test_abs_properties(value in any::<i64>()) {
        let tub = TubularBigInt::new(value);
        let abs_tub = tub.abs();

        // Absolute value should always be non-negative
        assert!(!abs_tub.is_negative());

        // Abs of zero should be zero
        if tub.is_zero() {
            assert_eq!(abs_tub, tub);
        }

        // Abs of abs should be abs
        assert_eq!(abs_tub.abs(), abs_tub);
    }

    #[test]
    fn test_safe_div_properties(a in any::<i64>(), b in any::<i64>()) {
        let tub_a = TubularBigInt::new(a);
        let tub_b = TubularBigInt::new(b);

        let result = tub_a.safe_div(&tub_b);

        // Division by zero should return zero
        if tub_b.is_zero() {
            assert_eq!(result, TubularBigInt::zero());
        } else {
            // For small numbers, result should be floor division
            if let (Some(a_res), Some(b_res), Some(result_res)) = (tub_a.to_i64(), tub_b.to_i64(), result.to_i64()) {
                if b_res != 0 {
                    assert_eq!(result_res, a_res / b_res);
                }
            }
        }
    }

    #[test]
    fn test_char_conversion_roundtrip(ch in any::<char>()) {
        if let Some(code_point) = ch as u32 as i64 {
            let tub = TubularBigInt::from_char(ch);
            let back_char = tub.to_char();

            // Roundtrip should work for valid characters
            if let Some(converted) = back_char {
                assert_eq!(converted, ch);
            }
        }
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_creation() {
        let start = Instant::now();
        for i in 0..1_000_000 {
            let _tub = TubularBigInt::new(i);
        }
        let duration = start.elapsed();
        println!("TubularBigInt creation (1M): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_arithmetic() {
        let a = TubularBigInt::new(1000);
        let b = TubularBigInt::new(42);
        let start = Instant::now();

        for _ in 0..1_000_000 {
            let _sum = a.clone() + b.clone();
            let _diff = a.clone() - b.clone();
            let _prod = a.clone() * b.clone();
            let _div = a.safe_div(&b);
            let _mod = a.safe_mod(&b);
        }

        let duration = start.elapsed();
        println!("Arithmetic operations (5M): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn benchmark_increment_decrement() {
        let mut tub = TubularBigInt::new(0);
        let start = Instant::now();

        for i in 0..1_000_000 {
            tub.increment();
            if i % 2 == 0 {
                tub.decrement();
            }
        }

        let duration = start.elapsed();
        println!("Increment/decrement (2M): {:?}", duration);
        assert!(duration.as_millis() < 100);
        assert_eq!(tub.to_i64(), Some(500_000));
    }

    #[test]
    fn benchmark_large_number_operations() {
        let big = BigInt::from(1000000000000i64) * BigInt::from(1000000000000i64);
        let tub_a = TubularBigInt::from_bigint(big.clone());
        let tub_b = TubularBigInt::from_bigint(big.clone() + 1);
        let start = Instant::now();

        for _ in 0..100_000 {
            let _sum = tub_a.clone() + tub_b.clone();
            let _diff = tub_b.clone() - tub_a.clone();
            let _prod = tub_a.clone() * tub_b.clone();
        }

        let duration = start.elapsed();
        println!("Large number operations (300K): {:?}", duration);
        assert!(duration.as_millis() < 1000);
    }
}