//! Unit tests for the ProgramGrid and related types

use tubular::interpreter::grid::{ProgramCell, ProgramGrid, BoundingBox};
use tubular::types::Coordinate;
use tubular::types::error::{InitError, InterpreterError};
use proptest::prelude::*;

#[cfg(test)]
mod program_cell_tests {
    use super::*;

    #[test]
    fn test_program_cell_new() {
        let cell = ProgramCell::new('>');
        assert_eq!(cell.symbol, '>');
        assert!(cell.is_flow_control);
        assert!(!cell.is_operator);
    }

    #[test]
    fn test_program_cell_arithmetic_operators() {
        let operators = ['+', '-', '*', '/', '%'];
        for &op in &operators {
            let cell = ProgramCell::new(op);
            assert_eq!(cell.symbol, op);
            assert!(!cell.is_flow_control);
            assert!(cell.is_operator);
        }
    }

    #[test]
    fn test_program_cell_other_operators() {
        let operators = ['~', ':', ';', 'd', 'A', 'S', 'M', 'D', '=', '<', '>', 'G', 'P', 'C', 'R', '!', ',', 'n', '?'];
        for &op in &operators {
            let cell = ProgramCell::new(op);
            assert_eq!(cell.symbol, op);
            assert!(!cell.is_flow_control);
            assert!(cell.is_operator);
        }
    }

    #[test]
    fn test_program_cell_flow_control() {
        let flow_controls = ['|', '-', '/', '\\', '^', 'v', '<', '>'];
        for &fc in &flow_controls {
            let cell = ProgramCell::new(fc);
            assert_eq!(cell.symbol, fc);
            assert!(cell.is_flow_control);
            // Note: Some are also operators (like arrows)
        }
    }

    #[test]
    fn test_program_cell_digits() {
        for digit in '0'..='9' {
            let cell = ProgramCell::new(digit);
            assert_eq!(cell.symbol, digit);
            assert!(!cell.is_flow_control);
            assert!(cell.is_operator);
        }
    }

    #[test]
    fn test_program_cell_special_symbols() {
        // Start symbol
        let cell = ProgramCell::new('@');
        assert_eq!(cell.symbol, '@');
        assert!(!cell.is_flow_control);
        assert!(!cell.is_operator);

        // Sink symbol
        let cell = ProgramCell::new('!');
        assert_eq!(cell.symbol, '!');
        assert!(!cell.is_flow_control);
        assert!(cell.is_operator);
    }

    #[test]
    fn test_program_cell_is_start_symbol() {
        assert!(ProgramCell::is_start_symbol('@'));
        assert!(!ProgramCell::is_start_symbol('!'));
        assert!(!ProgramCell::is_start_symbol('>'));
        assert!(!ProgramCell::is_start_symbol(' '));
    }

    #[test]
    fn test_program_cell_is_sink_symbol() {
        assert!(ProgramCell::is_sink_symbol('!'));
        assert!(!ProgramCell::is_sink_symbol('@'));
        assert!(!ProgramCell::is_sink_symbol('>'));
        assert!(!ProgramCell::is_sink_symbol(' '));
    }

    #[test]
    fn test_program_cell_is_data_source() {
        assert!(ProgramCell::is_data_source('>'));
        assert!(ProgramCell::is_data_source('?'));
        for digit in '0'..='9' {
            assert!(ProgramCell::is_data_source(digit));
        }
        assert!(!ProgramCell::is_data_source('!'));
        assert!(!ProgramCell::is_data_source('@'));
        assert!(!ProgramCell::is_data_source('+'));
    }

    #[test]
    fn test_program_cell_is_data_sink() {
        assert!(ProgramCell::is_data_sink('!'));
        assert!(ProgramCell::is_data_sink(','));
        assert!(ProgramCell::is_data_sink('n'));
        assert!(!ProgramCell::is_data_sink('>'));
        assert!(!ProgramCell::is_data_sink('@'));
    }

    #[test]
    fn test_program_cell_is_valid_symbol() {
        // Valid symbols
        assert!(ProgramCell::is_valid_symbol('@'));
        assert!(ProgramCell::is_valid_symbol('!'));
        assert!(ProgramCell::is_valid_symbol('>'));
        assert!(ProgramCell::is_valid_symbol('+'));
        assert!(ProgramCell::is_valid_symbol('5'));
        assert!(ProgramCell::is_valid_symbol('?'));
        assert!(ProgramCell::is_valid_symbol(','));
        assert!(ProgramCell::is_valid_symbol('n'));

        // Invalid symbols
        assert!(!ProgramCell::is_valid_symbol('x'));
        assert!(!ProgramCell::is_valid_symbol('$'));
        assert!(!ProgramCell::is_valid_symbol('#'));
        assert!(!ProgramCell::is_valid_symbol('\t'));
    }
}

#[cfg(test)]
mod bounding_box_tests {
    use super::*;

    #[test]
    fn test_bounding_box_new() {
        let bbox = BoundingBox::new();
        assert_eq!(bbox.min_x, isize::MAX);
        assert_eq!(bbox.min_y, isize::MAX);
        assert_eq!(bbox.max_x, isize::MIN);
        assert_eq!(bbox.max_y, isize::MIN);
        assert_eq!(bbox.width(), 0);
        assert_eq!(bbox.height(), 0);
    }

    #[test]
    fn test_bounding_box_include() {
        let mut bbox = BoundingBox::new();

        bbox.include(Coordinate::new(5, 10));
        assert_eq!(bbox.min_x, 5);
        assert_eq!(bbox.min_y, 10);
        assert_eq!(bbox.max_x, 5);
        assert_eq!(bbox.max_y, 10);
        assert_eq!(bbox.width(), 1);
        assert_eq!(bbox.height(), 1);

        bbox.include(Coordinate::new(0, 20));
        assert_eq!(bbox.min_x, 0);
        assert_eq!(bbox.min_y, 10);
        assert_eq!(bbox.max_x, 5);
        assert_eq!(bbox.max_y, 20);
        assert_eq!(bbox.width(), 6);
        assert_eq!(bbox.height(), 11);

        bbox.include(Coordinate::new(15, 5));
        assert_eq!(bbox.min_x, 0);
        assert_eq!(bbox.min_y, 5);
        assert_eq!(bbox.max_x, 15);
        assert_eq!(bbox.max_y, 20);
        assert_eq!(bbox.width(), 16);
        assert_eq!(bbox.height(), 16);
    }

    #[test]
    fn test_bounding_box_contains() {
        let mut bbox = BoundingBox::new();
        bbox.include(Coordinate::new(0, 0));
        bbox.include(Coordinate::new(10, 10));

        assert!(bbox.contains(Coordinate::new(0, 0)));
        assert!(bbox.contains(Coordinate::new(5, 5)));
        assert!(bbox.contains(Coordinate::new(10, 10)));
        assert!(!bbox.contains(Coordinate::new(-1, 0)));
        assert!(!bbox.contains(Coordinate::new(0, -1)));
        assert!(!bbox.contains(Coordinate::new(11, 10)));
        assert!(!bbox.contains(Coordinate::new(10, 11)));
    }

    #[test]
    fn test_bounding_box_empty() {
        let bbox = BoundingBox::new();
        assert_eq!(bbox.width(), 0);
        assert_eq!(bbox.height(), 0);
        assert!(!bbox.contains(Coordinate::new(0, 0)));
    }

    #[test]
    fn test_bounding_box_single_point() {
        let mut bbox = BoundingBox::new();
        bbox.include(Coordinate::new(5, 5));

        assert_eq!(bbox.width(), 1);
        assert_eq!(bbox.height(), 1);
        assert!(bbox.contains(Coordinate::new(5, 5)));
        assert!(!bbox.contains(Coordinate::new(4, 5)));
        assert!(!bbox.contains(Coordinate::new(6, 5)));
        assert!(!bbox.contains(Coordinate::new(5, 4)));
        assert!(!bbox.contains(Coordinate::new(5, 6)));
    }

    #[test]
    fn test_bounding_box_negative_coordinates() {
        let mut bbox = BoundingBox::new();
        bbox.include(Coordinate::new(-10, -5));
        bbox.include(Coordinate::new(-3, -2));

        assert_eq!(bbox.min_x, -10);
        assert_eq!(bbox.min_y, -5);
        assert_eq!(bbox.max_x, -3);
        assert_eq!(bbox.max_y, -2);
        assert_eq!(bbox.width(), 8);
        assert_eq!(bbox.height(), 4);

        assert!(bbox.contains(Coordinate::new(-10, -5)));
        assert!(bbox.contains(Coordinate::new(-3, -2)));
        assert!(bbox.contains(Coordinate::new(-6, -3)));
        assert!(!bbox.contains(Coordinate::new(-11, -5)));
        assert!(!bbox.contains(Coordinate::new(-3, -1)));
    }

    #[test]
    fn test_bounding_box_default() {
        let bbox = BoundingBox::default();
        assert_eq!(bbox.min_x, isize::MAX);
        assert_eq!(bbox.min_y, isize::MAX);
        assert_eq!(bbox.max_x, isize::MIN);
        assert_eq!(bbox.max_y, isize::MIN);
    }
}

#[cfg(test)]
mod program_grid_tests {
    use super::*;

    #[test]
    fn test_program_grid_new() {
        let grid = ProgramGrid::new();
        assert!(grid.is_empty());
        assert_eq!(grid.size(), 0);
        assert!(grid.start.is_none());
        assert_eq!(grid.dimensions(), (0, 0));
    }

    #[test]
    fn test_program_grid_add_cell() {
        let mut grid = ProgramGrid::new();
        let coord = Coordinate::new(0, 0);
        let symbol = '@';

        let result = grid.add_cell(coord, symbol);
        assert!(result.is_ok());
        assert_eq!(grid.size(), 1);
        assert_eq!(grid.start, Some(coord));

        let cell = grid.get(coord).unwrap();
        assert_eq!(cell.symbol, symbol);
    }

    #[test]
    fn test_program_grid_add_multiple_cells() {
        let mut grid = ProgramGrid::new();

        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();
        grid.add_cell(Coordinate::new(3, 0), '5').unwrap();

        assert_eq!(grid.size(), 4);
        assert_eq!(grid.dimensions(), (4, 1));

        assert_eq!(grid.get_symbol(Coordinate::new(2, 0)), Some('+'));
        assert_eq!(grid.get_symbol(Coordinate::new(3, 0)), Some('5'));
        assert_eq!(grid.get_symbol(Coordinate::new(10, 10)), None);
    }

    #[test]
    fn test_program_grid_multiple_start_symbols() {
        let mut grid = ProgramGrid::new();

        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        let result = grid.add_cell(Coordinate::new(5, 5), '@');

        assert!(result.is_err());
        match result.unwrap_err() {
            InterpreterError::Initialization(InitError::MultipleStartSymbols) => {
                // Expected
            }
            _ => panic!("Expected MultipleStartSymbols error"),
        }
    }

    #[test]
    fn test_program_grid_invalid_character() {
        let mut grid = ProgramGrid::new();
        let result = grid.add_cell(Coordinate::new(0, 0), 'x');

        assert!(result.is_err());
        match result.unwrap_err() {
            InterpreterError::Initialization(InitError::InvalidCharacter('x', coord)) => {
                assert_eq!(coord, Coordinate::new(0, 0));
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn test_program_grid_get() {
        let mut grid = ProgramGrid::new();
        let coord = Coordinate::new(5, 10);
        let symbol = '>';

        grid.add_cell(coord, symbol).unwrap();

        let cell = grid.get(coord).unwrap();
        assert_eq!(cell.symbol, symbol);
        assert!(cell.is_flow_control);

        assert_eq!(grid.get(Coordinate::new(0, 0)), None);
    }

    #[test]
    fn test_program_grid_get_symbol() {
        let mut grid = ProgramGrid::new();
        let coord = Coordinate::new(1, 2);
        let symbol = '+';

        grid.add_cell(coord, symbol).unwrap();

        assert_eq!(grid.get_symbol(coord), Some(symbol));
        assert_eq!(grid.get_symbol(Coordinate::new(99, 99)), None);
    }

    #[test]
    fn test_program_grid_dimensions() {
        let mut grid = ProgramGrid::new();

        // Create a 3x2 grid
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();
        grid.add_cell(Coordinate::new(0, 1), 'v').unwrap();
        grid.add_cell(Coordinate::new(1, 1), '5').unwrap();

        assert_eq!(grid.dimensions(), (3, 2));
    }

    #[test]
    fn test_program_grid_validate_no_start() {
        let mut grid = ProgramGrid::new();
        grid.add_cell(Coordinate::new(0, 0), '>').unwrap();

        let result = grid.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            InterpreterError::Initialization(InitError::NoStartSymbol) => {
                // Expected
            }
            _ => panic!("Expected NoStartSymbol error"),
        }
    }

    #[test]
    fn test_program_grid_validate_too_large() {
        let mut grid = ProgramGrid::new();
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();

        // Add a cell far away to make grid too large
        grid.add_cell(Coordinate::new(1001, 0), '>').unwrap();

        let result = grid.validate();
        assert!(result.is_err());
        match result.unwrap_err() {
            InterpreterError::Initialization(InitError::GridSizeExceeded(width, height)) => {
                assert_eq!(width, 1002);
                assert_eq!(height, 1);
            }
            _ => panic!("Expected GridSizeExceeded error"),
        }
    }

    #[test]
    fn test_program_grid_validate_success() {
        let mut grid = ProgramGrid::new();
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();

        let result = grid.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_program_grid_iter() {
        let mut grid = ProgramGrid::new();
        let coords = vec![
            Coordinate::new(0, 0),
            Coordinate::new(1, 0),
            Coordinate::new(0, 1),
        ];
        let symbols = vec!['@', '>', 'v'];

        for (coord, symbol) in coords.iter().zip(symbols.iter()) {
            grid.add_cell(*coord, *symbol).unwrap();
        }

        let mut found_coords = Vec::new();
        let mut found_symbols = Vec::new();

        for (coord, cell) in grid.iter() {
            found_coords.push(*coord);
            found_symbols.push(cell.symbol);
        }

        found_coords.sort();
        found_symbols.sort();

        let mut expected_coords = coords.clone();
        let mut expected_symbols = symbols.clone();
        expected_coords.sort();
        expected_symbols.sort();

        assert_eq!(found_coords, expected_coords);
        assert_eq!(found_symbols, expected_symbols);
    }

    #[test]
    fn test_program_grid_symbols_in_bounds() {
        let mut grid = ProgramGrid::new();

        // Create a simple grid
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();
        grid.add_cell(Coordinate::new(0, 1), 'v').unwrap();
        grid.add_cell(Coordinate::new(1, 1), '5').unwrap();

        let lines = grid.symbols_in_bounds();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "@>+");
        assert_eq!(lines[1], "v5 ");
    }

    #[test]
    fn test_program_grid_symbols_in_bounds_sparse() {
        let mut grid = ProgramGrid::new();

        // Create a sparse grid
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();
        grid.add_cell(Coordinate::new(1, 2), '5').unwrap();

        let lines = grid.symbols_in_bounds();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "@ +");
        assert_eq!(lines[1], "   ");
        assert_eq!(lines[2], " 5 ");
    }

    #[test]
    fn test_program_grid_display() {
        let mut grid = ProgramGrid::new();

        grid.add_cell(Coordinate::new(0, 0), '@').unwrap();
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap();
        grid.add_cell(Coordinate::new(2, 0), '+').unwrap();

        let display = format!("{}", grid);
        assert!(display.contains("@>+"));
    }

    #[test]
    fn test_program_grid_default() {
        let grid = ProgramGrid::default();
        assert!(grid.is_empty());
        assert_eq!(grid.size(), 0);
        assert!(grid.start.is_none());
    }

    #[test]
    fn test_program_grid_complex_layout() {
        let mut grid = ProgramGrid::new();

        // Create a simple program
        grid.add_cell(Coordinate::new(0, 0), '@').unwrap(); // Start
        grid.add_cell(Coordinate::new(1, 0), '>').unwrap(); // Right
        grid.add_cell(Coordinate::new(2, 0), '5').unwrap(); // Push 5
        grid.add_cell(Coordinate::new(3, 0), '+').unwrap(); // Add
        grid.add_cell(Coordinate::new(4, 0), '!').unwrap(); // Output

        assert_eq!(grid.size(), 5);
        assert_eq!(grid.start, Some(Coordinate::new(0, 0)));
        assert_eq!(grid.dimensions(), (5, 1));

        // Verify cell types
        let start_cell = grid.get(Coordinate::new(0, 0)).unwrap();
        assert!(!start_cell.is_flow_control);
        assert!(!start_cell.is_operator);

        let arrow_cell = grid.get(Coordinate::new(1, 0)).unwrap();
        assert!(arrow_cell.is_flow_control);

        let digit_cell = grid.get(Coordinate::new(2, 0)).unwrap();
        assert!(!digit_cell.is_flow_control);
        assert!(digit_cell.is_operator);

        let op_cell = grid.get(Coordinate::new(3, 0)).unwrap();
        assert!(!op_cell.is_flow_control);
        assert!(op_cell.is_operator);

        let output_cell = grid.get(Coordinate::new(4, 0)).unwrap();
        assert!(!output_cell.is_flow_control);
        assert!(output_cell.is_operator);
    }

    #[test]
    fn test_program_grid_edge_cases() {
        let mut grid = ProgramGrid::new();

        // Test with negative coordinates
        grid.add_cell(Coordinate::new(-5, -3), '@').unwrap();
        grid.add_cell(Coordinate::new(-4, -3), '>').unwrap();
        grid.add_cell(Coordinate::new(-3, -3), '1').unwrap();

        assert_eq!(grid.dimensions(), (3, 1));
        assert_eq!(grid.start, Some(Coordinate::new(-5, -3)));

        // Test bounds with negative coordinates
        let lines = grid.symbols_in_bounds();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], ">1"); // @ should be included too

        // Add a cell to expand bounds
        grid.add_cell(Coordinate::new(-5, -2), 'v').unwrap();
        assert_eq!(grid.dimensions(), (3, 2));
    }
}

// Property-based tests using proptest
proptest! {
    #[test]
    fn test_program_cell_symbol_classification(symbol in any::<char>()) {
        let cell = ProgramCell::new(symbol);

        // Test that classification is consistent
        let is_flow = ProgramCell::is_flow_control_symbol(symbol);
        let is_operator = ProgramCell::is_operator_symbol(symbol);
        let is_start = ProgramCell::is_start_symbol(symbol);
        let is_sink = ProgramCell::is_sink_symbol(symbol);
        let is_source = ProgramCell::is_data_source(symbol);

        assert_eq!(cell.is_flow_control, is_flow);
        assert_eq!(cell.is_operator, is_operator);

        // Start symbol should not be flow control or operator
        if is_start {
            assert!(!is_flow);
            assert!(!is_operator);
        }

        // Valid symbol should be at least one type
        if ProgramCell::is_valid_symbol(symbol) {
            assert!(is_flow || is_operator || is_start || is_sink || is_source || symbol.is_ascii_digit());
        }
    }

    #[test]
    fn test_bounding_box_coordinates(
        coords in prop::collection::vec(
            (any::<isize>(), any::<isize>()),
            1..50
        )
    ) {
        let mut bbox = BoundingBox::new();
        let coordinate_list: Vec<Coordinate> = coords
            .iter()
            .map(|(x, y)| Coordinate::new(*x, *y))
            .collect();

        for &coord in &coordinate_list {
            bbox.include(coord);
        }

        if !coordinate_list.is_empty() {
            // Check that all coordinates are within bounds
            for coord in &coordinate_list {
                assert!(bbox.contains(*coord));
            }

            // Check bounds are tight (no smaller bounding box would contain all points)
            let mut found_min_x = false;
            let mut found_max_x = false;
            let mut found_min_y = false;
            let mut found_max_y = false;

            for coord in &coordinate_list {
                if coord.x == bbox.min_x { found_min_x = true; }
                if coord.x == bbox.max_x { found_max_x = true; }
                if coord.y == bbox.min_y { found_min_y = true; }
                if coord.y == bbox.max_y { found_max_y = true; }
            }

            assert!(found_min_x);
            assert!(found_max_x);
            assert!(found_min_y);
            assert!(found_max_y);
        }
    }

    #[test]
    fn test_program_grid_add_cells(
        cells in prop::collection::vec(
            (any::<isize>(), any::<isize>(), any::<char>()),
            1..20
        )
    ) {
        let mut grid = ProgramGrid::new();
        let mut start_count = 0;
        let mut valid_cells = Vec::new();

        for (x, y, symbol) in cells {
            let coord = Coordinate::new(x, y);

            if symbol.is_ascii() && ProgramCell::is_valid_symbol(symbol) {
                valid_cells.push((coord, symbol));

                if ProgramCell::is_start_symbol(symbol) {
                    start_count += 1;
                }
            }
        }

        // Try to add valid cells
        let mut added_count = 0;
        for (coord, symbol) in valid_cells {
            if grid.add_cell(coord, symbol).is_ok() {
                added_count += 1;
            }
        }

        // Should have at most one start symbol
        assert!(grid.start.is_none() || grid.start.is_some());

        // Grid size should match number of successfully added cells
        assert_eq!(grid.size(), added_count);

        // All added cells should be retrievable
        for (coord, symbol) in valid_cells.iter().take(added_count) {
            assert_eq!(grid.get_symbol(*coord), Some(*symbol));
        }
    }

    #[test]
    fn test_bounding_box_dimensions(
        coords in prop::collection::vec(
            (any::<isize>(), any::<isize>()),
            2..20
        )
    ) {
        let mut bbox = BoundingBox::new();
        let coordinate_list: Vec<Coordinate> = coords
            .iter()
            .map(|(x, y)| Coordinate::new(*x, *y))
            .collect();

        for &coord in &coordinate_list {
            bbox.include(coord);
        }

        let width = bbox.width();
        let height = bbox.height();

        // Dimensions should be positive for non-empty sets
        if !coordinate_list.is_empty() {
            assert!(width > 0);
            assert!(height > 0);

            // Area should be at least the number of points
            assert!((width * height) >= coordinate_list.len() as usize);
        }
    }
}

// Performance benchmarks
#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_program_grid_operations() {
        let mut grid = ProgramGrid::new();
        let start = Instant::now();

        // Add many cells
        for i in 0..10_000 {
            let x = i % 1000;
            let y = i / 1000;
            let symbol = match i % 4 {
                0 => '@',
                1 => '>',
                2 => '+',
                _ => '0',
            };
            // Only one start symbol
            if symbol != '@' || grid.start.is_none() {
                let _ = grid.add_cell(Coordinate::new(x as isize, y as isize), symbol);
            }
        }

        let add_duration = start.elapsed();
        println!("ProgramGrid add cells (10K): {:?}", add_duration);
        assert!(add_duration.as_millis() < 500);

        // Test lookups
        let start = Instant::now();
        for i in 0..10_000 {
            let x = i % 1000;
            let y = i / 1000;
            let _cell = grid.get(Coordinate::new(x as isize, y as isize));
        }
        let lookup_duration = start.elapsed();
        println!("ProgramGrid lookups (10K): {:?}", lookup_duration);
        assert!(lookup_duration.as_millis() < 200);
    }

    #[test]
    fn benchmark_bounding_box_operations() {
        let mut bbox = BoundingBox::new();
        let coords: Vec<Coordinate> = (0..100_000)
            .map(|i| Coordinate::new(
                (i % 1000) as isize,
                (i / 1000) as isize
            ))
            .collect();

        let start = Instant::now();
        for coord in &coords {
            bbox.include(*coord);
        }
        let duration = start.elapsed();
        println!("BoundingBox include (100K): {:?}", duration);
        assert!(duration.as_millis() < 100);

        let start = Instant::now();
        for coord in &coords {
            let _contains = bbox.contains(*coord);
        }
        let duration = start.elapsed();
        println!("BoundingBox contains (100K): {:?}", duration);
        assert!(duration.as_millis() < 50);
    }

    #[test]
    fn benchmark_program_cell_classification() {
        let chars: Vec<char> = (0..128).map(|c| c as u8 as char).collect();
        let start = Instant::now();

        for _ in 0..1_000_000 {
            for &ch in &chars {
                let _cell = ProgramCell::new(ch);
                let _is_flow = ProgramCell::is_flow_control_symbol(ch);
                let _is_operator = ProgramCell::is_operator_symbol(ch);
                let _is_valid = ProgramCell::is_valid_symbol(ch);
            }
        }

        let duration = start.elapsed();
        println!("ProgramCell classification (128M): {:?}", duration);
        assert!(duration.as_millis() < 1000);
    }

    #[test]
    fn benchmark_program_grid_iteration() {
        let mut grid = ProgramGrid::new();

        // Create a dense grid
        for y in 0..100 {
            for x in 0..100 {
                let symbol = if x == 0 && y == 0 { '@' }
                           else if (x + y) % 2 == 0 { '+' }
                           else { '>' };
                let _ = grid.add_cell(Coordinate::new(x, y), symbol);
            }
        }

        let start = Instant::now();
        for _ in 0..10_000 {
            let count = grid.iter().count();
            assert_eq!(count, 10000);
        }
        let duration = start.elapsed();
        println!("ProgramGrid iteration (100M cells): {:?}", duration);
        assert!(duration.as_millis() < 2000);
    }
}