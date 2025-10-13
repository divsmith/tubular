use tubular::parser::GridParser;

fn main() {
    let parser = GridParser::new();
    let content = "@|72,!";
    let grid = parser.parse_string(content).unwrap();

    println!("Grid size: {}", grid.size());
    println!("Grid start: {:?}", grid.start);

    for (coord, cell) in grid.iter() {
        println!("Cell at {}: '{}'", coord, cell.symbol);
    }
}