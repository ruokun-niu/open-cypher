
fn main() {
    use open_cypher::parser::print_pairs;
    use open_cypher::parser::process_pairs;
    use open_cypher::parser::parse;
    // let code = "MATCH (n) WHERE n.name CONTAINS \"s\" RETURN n.name;";
    let code = "MATCH 
                         (v:Vehicle)-[:LOCATED_IN]->(:Zone {type:'Parking Lot'}) 
                         RETURN 
                         elementId(v) AS id, 
                         v.make AS make, 
                         v.model AS model, 
                         v.color AS color, 
                         v.plate AS plate";
    // let code = "MATCH (n) WHERE n.name CONTAINS12 RETURN n.name;";
    match parse(code) {
        Ok(tree) => print_pairs(tree),
        // Ok(tree) => process_pairs(tree),
        Err(err) => eprintln!("ERROR={}", err),
    }
    // use open_cypher::parser::parse_string_literal;
    // let text = "n.name";
    // match parse_string_literal(text) {
    //     Ok(tree) => print_pairs(tree),
    //     Err(err) => eprintln!("ERROR={}", err),
    // }
}
