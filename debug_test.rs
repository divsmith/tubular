// Simple test to debug the hanging issue
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting debug test...");

    println!("Reading file...");
    let content = fs::read_to_string("examples/test_basic.tb")?;
    println!("File read successfully: {} bytes", content.len());

    // Try to create the parser
    println!("Creating parser...");
    let parser = tubular::parser::grid_parser::GridParser::new();
    println!("Parser created");

    // Try to parse
    println!("Parsing content...");
    let grid = parser.parse_string(&content)?;
    println!("Parsing successful!");

    Ok(())
}