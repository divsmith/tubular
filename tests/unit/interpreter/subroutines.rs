//! Unit tests for the CallStack and StackFrame types

use tubular::interpreter::subroutines::{CallStack, StackFrame};
use tubular::types::Coordinate;
use tubular::types::direction::Direction;
use proptest::prelude::*;

#[cfg(test)]
mod stack_frame_tests {
    use super::*;

    #[test]
    fn test_stack_frame_new() {
        let position = Coordinate::new(5, 10);
        let direction = Direction::Right;

        let frame = StackFrame::new(position, direction);

        assert_eq!(frame.return_position, position);
        assert_eq!(frame.return_direction, direction);
    }

    #[test]
    fn test_stack_frame_equality() {
        let pos = Coordinate::new(1, 2);
        let dir = Direction::Up;

        let frame1 = StackFrame::new(pos, dir);
        let frame2 = StackFrame::new(pos, dir);
        let frame3 = StackFrame::new(Coordinate::new(3, 4), dir);

        assert_eq!(frame1, frame2);
        assert_ne!(frame1, frame3);
    }

    #[test]
    fn test_stack_frame_debug() {
        let frame = StackFrame::new(Coordinate::new(1, 2), Direction::Right);
        let debug_str = format!("{:?}", frame);
        assert!(debug_str.contains("StackFrame"));
        assert!(debug_str.contains("return_position"));
        assert!(debug_str.contains("return_direction"));
    }
}

#[cfg(test)]
mod call_stack_tests {
    use super::*;

    #[test]
    fn test_call_stack_new() {
        let stack = CallStack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.depth(), 0);
        assert_eq!(stack.max_depth_reached(), 0);
    }

    #[test]
    fn test_call_stack_with_capacity() {
        let stack = CallStack::with_capacity(10);
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        // Can't directly test capacity, but should work without panics
    }

    #[test]
    fn test_call_stack_push() {
        let mut stack = CallStack::new();
        let frame = StackFrame::new(Coordinate::new(1, 2), Direction::Right);

        stack.push(frame.clone());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.depth(), 1);
        assert_eq!(stack.max_depth_reached(), 1);
        assert_eq!(stack.peek(), Some(&frame));
    }

    #[test]
    fn test_call_stack_push_return() {
        let mut stack = CallStack::new();
        let position = Coordinate::new(5, 10);
        let direction = Direction::Down;

        stack.push_return(position, direction);
        assert_eq!(stack.len(), 1);

        let frame = stack.peek().unwrap();
        assert_eq!(frame.return_position, position);
        assert_eq!(frame.return_direction, direction);
    }

    #[test]
    fn test_call_stack_pop() {
        let mut stack = CallStack::new();
        let frame = StackFrame::new(Coordinate::new(1, 2), Direction::Right);

        stack.push(frame.clone());
        let popped = stack.pop();
        assert_eq!(popped, Some(frame));
        assert!(stack.is_empty());

        // Pop from empty stack
        let popped_empty = stack.pop();
        assert_eq!(popped_empty, None);
    }

    #[test]
    fn test_call_stack_pop_or_none() {
        let mut stack = CallStack::new();
        let frame = StackFrame::new(Coordinate::new(3, 4), Direction::Left);

        stack.push(frame.clone());
        let popped = stack.pop_or_none();
        assert_eq!(popped, Some(frame));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_call_stack_peek() {
        let mut stack = CallStack::new();
        assert_eq!(stack.peek(), None);

        let frame1 = StackFrame::new(Coordinate::new(1, 1), Direction::Up);
        let frame2 = StackFrame::new(Coordinate::new(2, 2), Direction::Down);

        stack.push(frame1);
        assert_eq!(stack.peek(), Some(&frame1));

        stack.push(frame2.clone());
        assert_eq!(stack.peek(), Some(&frame2));

        stack.pop();
        assert_eq!(stack.peek(), Some(&frame1));
    }

    #[test]
    fn test_call_stack_multiple_operations() {
        let mut stack = CallStack::new();

        for i in 0..5 {
            stack.push_return(
                Coordinate::new(i, i * 2),
                match i % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                }
            );
        }

        assert_eq!(stack.len(), 5);
        assert_eq!(stack.max_depth_reached(), 5);

        for i in (0..5).rev() {
            let popped = stack.pop().unwrap();
            assert_eq!(popped.return_position, Coordinate::new(i, i * 2));
        }

        assert!(stack.is_empty());
        assert_eq!(stack.max_depth_reached(), 5); // Max depth preserved
    }

    #[test]
    fn test_call_stack_clear() {
        let mut stack = CallStack::new();

        for i in 0..10 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        assert_eq!(stack.len(), 10);
        assert_eq!(stack.max_depth_reached(), 10);

        stack.clear();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.max_depth_reached(), 10); // Max depth preserved
    }

    #[test]
    fn test_call_stack_truncate() {
        let mut stack = CallStack::new();

        for i in 0..10 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        stack.truncate(5);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack.peek().unwrap().return_position, Coordinate::new(4, 4));

        // Truncate to larger size (should do nothing)
        stack.truncate(10);
        assert_eq!(stack.len(), 5);

        // Truncate to zero
        stack.truncate(0);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_call_stack_get() {
        let mut stack = CallStack::new();

        for i in 0..5 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        assert_eq!(stack.get(0), Some(&StackFrame::new(Coordinate::new(0, 0), Direction::Right)));
        assert_eq!(stack.get(4), Some(&StackFrame::new(Coordinate::new(4, 4), Direction::Right)));
        assert_eq!(stack.get(5), None);
    }

    #[test]
    fn test_call_stack_get_from_top() {
        let mut stack = CallStack::new();

        for i in 0..5 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        assert_eq!(stack.get_from_top(0), Some(&StackFrame::new(Coordinate::new(4, 4), Direction::Right)));
        assert_eq!(stack.get_from_top(4), Some(&StackFrame::new(Coordinate::new(0, 0), Direction::Right)));
        assert_eq!(stack.get_from_top(5), None);
    }

    #[test]
    fn test_call_stack_swap_top_two() {
        let mut stack = CallStack::new();

        // Empty stack
        assert!(!stack.swap_top_two());

        // One element
        stack.push_return(Coordinate::new(1, 1), Direction::Right);
        assert!(!stack.swap_top_two());
        assert_eq!(stack.len(), 1);

        // Two elements
        stack.push_return(Coordinate::new(2, 2), Direction::Left);
        assert!(stack.swap_top_two());
        assert_eq!(stack.peek().unwrap().return_position, Coordinate::new(1, 1));
        assert_eq!(stack.get_from_top(1).unwrap().return_position, Coordinate::new(2, 2));

        // More elements
        stack.push_return(Coordinate::new(3, 3), Direction::Up);
        assert!(stack.swap_top_two());
        assert_eq!(stack.peek().unwrap().return_position, Coordinate::new(1, 1));
        assert_eq!(stack.get_from_top(1).unwrap().return_position, Coordinate::new(3, 3));
        assert_eq!(stack.get_from_top(2).unwrap().return_position, Coordinate::new(2, 2));
    }

    #[test]
    fn test_call_stack_as_slice() {
        let mut stack = CallStack::new();

        for i in 0..5 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        let slice = stack.as_slice();
        assert_eq!(slice.len(), 5);
        assert_eq!(slice[0].return_position, Coordinate::new(0, 0));
        assert_eq!(slice[4].return_position, Coordinate::new(4, 4));
    }

    #[test]
    fn test_call_stack_is_within_limit() {
        let mut stack = CallStack::new();

        assert!(stack.is_within_limit(0));
        assert!(stack.is_within_limit(10));

        stack.push_return(Coordinate::new(1, 1), Direction::Right);
        assert!(stack.is_within_limit(1));
        assert!(stack.is_within_limit(10));
        assert!(!stack.is_within_limit(0));

        stack.push_return(Coordinate::new(2, 2), Direction::Left);
        assert!(stack.is_within_limit(2));
        assert!(!stack.is_within_limit(1));
    }

    #[test]
    fn test_call_stack_max_depth_tracking() {
        let mut stack = CallStack::new();

        // Push some frames
        for i in 1..=5 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }
        assert_eq!(stack.max_depth_reached(), 5);

        // Pop some frames
        for _ in 0..3 {
            stack.pop();
        }
        assert_eq!(stack.len(), 2);
        // Max depth should still be 5
        assert_eq!(stack.max_depth_reached(), 5);

        // Push more frames (but don't exceed previous max)
        stack.push_return(Coordinate::new(6, 6), Direction::Up);
        stack.push_return(Coordinate::new(7, 7), Direction::Down);
        assert_eq!(stack.len(), 4);
        assert_eq!(stack.max_depth_reached(), 5);

        // Exceed previous max
        stack.push_return(Coordinate::new(8, 8), Direction::Left);
        stack.push_return(Coordinate::new(9, 9), Direction::Right);
        assert_eq!(stack.len(), 6);
        assert_eq!(stack.max_depth_reached(), 6);
    }

    #[test]
    fn test_call_stack_iteration() {
        let mut stack = CallStack::new();

        for i in 0..5 {
            stack.push_return(Coordinate::new(i, i), Direction::Right);
        }

        let positions: Vec<_> = stack.iter()
            .map(|frame| frame.return_position)
            .collect();

        assert_eq!(positions, vec![
            Coordinate::new(0, 0),
            Coordinate::new(1, 1),
            Coordinate::new(2, 2),
            Coordinate::new(3, 3),
            Coordinate::new(4, 4),
        ]);
    }

    #[test]
    fn test_call_stack_contains_position() {
        let mut stack = CallStack::new();

        stack.push_return(Coordinate::new(1, 1), Direction::Right);
        stack.push_return(Coordinate::new(5, 10), Direction::Left);
        stack.push_return(Coordinate::new(3, 7), Direction::Up);

        assert!(stack.contains_position(Coordinate::new(1, 1)));
        assert!(stack.contains_position(Coordinate::new(5, 10)));
        assert!(stack.contains_position(Coordinate::new(3, 7)));
        assert!(!stack.contains_position(Coordinate::new(2, 2)));
        assert!(!stack.contains_position(Coordinate::new(0, 0)));
    }

    #[test]
    fn test_call_stack_filter_by_direction() {
        let mut stack = CallStack::new();

        stack.push_return(Coordinate::new(1, 1), Direction::Right);
        stack.push_return(Coordinate::new(2, 2), Direction::Right);
        stack.push_return(Coordinate::new(3, 3), Direction::Left);
        stack.push_return(Coordinate::new(4, 4), Direction::Up);
        stack.push_return(Coordinate::new(5, 5), Direction::Right);

        let right_frames = stack.filter_by_direction(Direction::Right);
        assert_eq!(right_frames.len(), 3);

        let left_frames = stack.filter_by_direction(Direction::Left);
        assert_eq!(left_frames.len(), 1);
        assert_eq!(left_frames[0].return_position, Coordinate::new(3, 3));

        let down_frames = stack.filter_by_direction(Direction::Down);
        assert_eq!(down_frames.len(), 0);
    }

    #[test]
    fn test_call_stack_default() {
        let stack = CallStack::default();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.max_depth_reached(), 0);
    }

    #[test]
    fn test_call_stack_display() {
        let mut stack = CallStack::new();

        stack.push_return(Coordinate::new(1, 1), Direction::Right);
        stack.push_return(Coordinate::new(2, 2), Direction::Left);

        let display = format!("{}", stack);
        assert!(display.contains("["));
        assert!(display.contains("]"));
        assert!(display.contains("(1, 1) >"));
        assert!(display.contains("(2, 2) <"));
    }

    #[test]
    fn test_call_stack_from_vec_frames() {
        let frames = vec![
            StackFrame::new(Coordinate::new(1, 1), Direction::Right),
            StackFrame::new(Coordinate::new(2, 2), Direction::Left),
        ];

        let stack: CallStack = frames.clone().into();
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.max_depth_reached(), 2);
        assert_eq!(stack.as_slice(), &frames[..]);
    }

    #[test]
    fn test_call_stack_from_vec_pairs() {
        let pairs = vec![
            (Coordinate::new(1, 1), Direction::Right),
            (Coordinate::new(2, 2), Direction::Left),
            (Coordinate::new(3, 3), Direction::Up),
        ];

        let stack: CallStack = pairs.into();
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.max_depth_reached(), 3);
        assert_eq!(stack.get(0).unwrap().return_position, Coordinate::new(1, 1));
        assert_eq!(stack.get(0).unwrap().return_direction, Direction::Right);
        assert_eq!(stack.get(1).unwrap().return_position, Coordinate::new(2, 2));
        assert_eq!(stack.get(1).unwrap().return_direction, Direction::Left);
        assert_eq!(stack.get(2).unwrap().return_position, Coordinate::new(3, 3));
        assert_eq!(stack.get(2).unwrap().return_direction, Direction::Up);
    }

    #[test]
    fn test_call_stack_edge_cases() {
        let mut stack = CallStack::new();

        // Test with negative coordinates
        stack.push_return(Coordinate::new(-5, -3), Direction::Down);
        stack.push_return(Coordinate::new(10, -7), Direction::Left);

        assert_eq!(stack.len(), 2);
        assert!(stack.contains_position(Coordinate::new(-5, -3)));
        assert!(stack.contains_position(Coordinate::new(10, -7)));

        // Test truncate to current size
        stack.truncate(2);
        assert_eq!(stack.len(), 2);

        // Test get_from_top on single element
        stack.pop();
        assert_eq!(stack.get_from_top(0).unwrap().return_position, Coordinate::new(-5, -3));
        assert_eq!(stack.get_from_top(1), None);
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_call_stack_push_pop_consistency(
        frames in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<u8>()),
            0..50
        )
    ) {
        let mut stack = CallStack::new();
        let stack_frames: Vec<StackFrame> = frames
            .iter()
            .map(|(x, y, dir)| {
                let direction = match dir % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
                StackFrame::new(Coordinate::new(*x, *y), direction)
            })
            .collect();

        // Push all frames
        for frame in &stack_frames {
            stack.push(frame.clone());
        }

        // Pop all frames and verify order (LIFO)
        let mut popped_frames = Vec::new();
        while !stack.is_empty() {
            popped_frames.push(stack.pop().unwrap());
        }

        // Should be reverse of input
        let expected: Vec<StackFrame> = stack_frames.iter().rev().cloned().collect();
        assert_eq!(popped_frames, expected);
    }

    #[test]
    fn test_call_stack_max_depth_tracking(
        operations in prop::collection::vec(
            prop::bool::weighted(0.7), // 70% push, 30% pop
            0..100
        )
    ) {
        let mut stack = CallStack::new();
        let mut max_seen = 0;

        for should_push in operations {
            if should_push {
                stack.push_return(
                    Coordinate::new(1, 1),
                    Direction::Right
                );
                max_seen = max_seen.max(stack.len());
            } else {
                stack.pop();
            }
        }

        assert_eq!(stack.max_depth_reached(), max_seen);
    }

    #[test]
    fn test_call_stack_get_from_top_properties(
        frames in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<u8>()),
            1..20
        )
    ) {
        let mut stack = CallStack::new();
        let stack_frames: Vec<StackFrame> = frames
            .iter()
            .map(|(x, y, dir)| {
                let direction = match dir % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
                StackFrame::new(Coordinate::new(*x, *y), direction)
            })
            .collect();

        for frame in &stack_frames {
            stack.push(frame.clone());
        }

        // Test all valid indices from top
        for i in 0..stack.len() {
            let from_top = stack.get_from_top(i).unwrap();
            let expected = &stack_frames[stack_frames.len() - 1 - i];
            assert_eq!(from_top, expected);
        }

        // Test invalid indices
        for i in stack.len()..stack.len() + 5 {
            assert_eq!(stack.get_from_top(i), None);
        }
    }

    #[test]
    fn test_call_stack_contains_position_properties(
        frames in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<u8>()),
            0..20
        )
    ) {
        let mut stack = CallStack::new();
        let positions: Vec<Coordinate> = frames
            .iter()
            .map(|(x, y, _)| Coordinate::new(*x, *y))
            .collect();

        for (x, y, dir) in &frames {
            let direction = match dir % 4 {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Left,
                _ => Direction::Right,
            };
            stack.push_return(Coordinate::new(*x, *y), direction);
        }

        // Check that all positions are contained
        for position in &positions {
            assert!(stack.contains_position(*position));
        }

        // Check that some random positions are not contained
        for i in 0..10 {
            let random_pos = Coordinate::new(i * 100 + 1000, i * 200 + 2000);
            if !positions.contains(&random_pos) {
                assert!(!stack.contains_position(random_pos));
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
    fn benchmark_call_stack_operations() {
        let mut stack = CallStack::new();
        let start = Instant::now();

        // Push operations
        for i in 0..100_000 {
            stack.push_return(
                Coordinate::new(i as isize, i as isize),
                match i % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                }
            );
        }

        let push_duration = start.elapsed();
        println!("CallStack push (100K): {:?}", push_duration);
        assert!(push_duration.as_millis() < 300);

        // Pop operations
        let start = Instant::now();
        while !stack.is_empty() {
            stack.pop();
        }
        let pop_duration = start.elapsed();
        println!("CallStack pop (100K): {:?}", pop_duration);
        assert!(pop_duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_call_stack_access_methods() {
        let mut stack = CallStack::new();
        for i in 0..10_000 {
            stack.push_return(Coordinate::new(i as isize, i as isize), Direction::Right);
        }

        let start = Instant::now();
        for i in 0..100_000 {
            let _get = stack.get(i % 10_000);
            let _get_from_top = stack.get_from_top(i % 10_000);
            let _peek = stack.peek();
            let _contains = stack.contains_position(Coordinate::new((i % 100) as isize, (i % 100) as isize));
        }
        let duration = start.elapsed();
        println!("CallStack access methods (400K): {:?}", duration);
        assert!(duration.as_millis() < 500);
    }

    #[test]
    fn benchmark_call_stack_iteration() {
        let mut stack = CallStack::new();
        for i in 0..10_000 {
            stack.push_return(Coordinate::new(i as isize, i as isize), Direction::Right);
        }

        let start = Instant::now();
        for _ in 0..1_000 {
            let _count = stack.iter().count();
            let _positions: Vec<_> = stack.iter()
                .map(|frame| frame.return_position)
                .collect();
        }
        let duration = start.elapsed();
        println!("CallStack iteration (10K iterations): {:?}", duration);
        assert!(duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_call_stack_filter() {
        let mut stack = CallStack::new();
        for i in 0..10_000 {
            stack.push_return(
                Coordinate::new(i as isize, i as isize),
                match i % 4 {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                }
            );
        }

        let start = Instant::now();
        for _ in 0..1_000 {
            let _right_frames = stack.filter_by_direction(Direction::Right);
            let _left_frames = stack.filter_by_direction(Direction::Left);
            let _up_frames = stack.filter_by_direction(Direction::Up);
            let _down_frames = stack.filter_by_direction(Direction::Down);
        }
        let duration = start.elapsed();
        println!("CallStack filter (4K filters): {:?}", duration);
        assert!(duration.as_millis() < 1000);
    }
}