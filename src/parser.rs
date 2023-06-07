extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pairs, Pair};
use pest_derive::Parser;
use std::{collections::HashMap, str::FromStr};
use bet::BeTree;
use regex::Regex;
use crate::ast::{Node, Operator};

#[derive(pest_derive::Parser)]
#[grammar = "cypher.pest"]
pub struct CypherParser;

pub fn parse(code: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    CypherParser::parse(Rule::Cypher, code)
    // let pairs = CypherParser::parse(Rule::Cypher, code);
    // process_pairs(pairs);
}

pub fn parse_string_literal(code: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    CypherParser::parse(Rule::UnescapedSymbolicName, code)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QueryOperator {
    As, 
}

pub fn bet_parse(input: &str) -> BeTree<QueryOperator,&str> {
    let mut expTree = BeTree::new();
    let components: Vec<_> = input
        .split(',')
        .map(|substring| {
            let trimmed = substring.trim();
            let parts: Vec<_> = trimmed.split("AS").map(|part| part.trim()).collect();
            parts
        })
        .collect();
    for component in components {
        // expTree.open_par();
        expTree.push_atom(component[0]);
        expTree.push_operator(QueryOperator::As);
        expTree.push_atom(component[1]);
        // expTree.close_par();
        // if component == "AS" {
        //     expTree.push_operator(QueryOperator::As);
        // } else {
        //     expTree.push_atom(component);
        // }
    }
    expTree.print_tree();
    expTree
    // let as_regex = Regex::new(r"\bAS\b").unwrap();
    // let mut expTree = BeTree::new();
    // let matches: Vec<_> = as_regex.find_iter(input).collect();

    // for mat in matches {
    //     expTree.push_operator(QueryOperator::As);
    // }
    // expTree
}

//Playing around with ast enums
// pub fn parseAST(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
//     let mut ast = vec![];
//     println!("Reached0");
//     let pairs = CypherParser::parse(Rule::Cypher, source)?;
//     // let p = pairs
//     //     .into_iter()
//     //     .next()
//     //     .unwrap();
//     // println!("Reached2");
//     // for pair in pairs{
//     //     println!("Reached1");
//     //     ast.push(build_ast_from_expr(pair));
//     // }
//     // println!("Rule:    {:?}", p.as_rule());
//     // if let Rule::ProjectionItem = p.as_rule() {
//     //     println!("reached");
//     //     ast.push(build_ast_from_expr(p));
//     // }
//     Ok(ast)
// }

pub fn build_ast_from_expr(pair: Pair<Rule>) -> Node {
    // Make this recursive
    match pair.as_rule() {
        Rule::ProjectionItem => {
            println!("{:?}- {:?}", pair.as_rule(), pair.as_str());
            //build an AST node here
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let _ = pair.next().unwrap();
            let op = pair.next().unwrap();
            let _ = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            // let lhsNode = Node::Str(lhspair.as_str().to_string());
            // let rhsNode = Node::Str(rhspair.as_str().to_string());
            let lhsNode = build_ast_from_expr(lhspair);
            let rhsNode = build_ast_from_expr(rhspair); 
            let centerNode = Node::BinaryExpr {
                op: match op.as_str() {
                    "AS" => Operator::As,
                    _ => panic!("Unknown operator"),
                },
                lhs: Box::new(lhsNode),
                rhs: Box::new(rhsNode),
            };
            println!("centerNode: {:?}", centerNode);
            centerNode
        }
        Rule::Expression => {
            let currNode = Node::Str(pair.as_str().to_string());
            currNode
        }
        Rule::Variable => {
            let currNode = Node::Str(pair.as_str().to_string());
            currNode
        }
        Rule::PatternElement => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            println!("PatternElement: {:?}- {:?}", lhspair.as_rule(), lhspair.as_str());
            Node::Str("null".to_string())
        }
        _ => Node::Str("null".to_string()),
    }
}  



pub fn process_pairs(pairs: Pairs<Rule>) {
    let mut query_info:HashMap<String, String> = HashMap::new();
    println!("process_pairs");
    let p = pairs
        .into_iter()
        .next()
        .expect("An error!");
    _process_pairs(p, 0, &mut query_info);
    for (key, value) in &query_info {
        println!("key: {}, Value: {}", key, value);
    }
}

pub fn _process_pairs(pair: Pair<Rule>, depth: usize, mut query_info: &mut HashMap<String, String>) {
    if pair.as_rule() == Rule::whitespace {
        return;
    }

    // println!("depth={}", depth);
    let currRule = format_string(pair.as_rule());
    match currRule.as_str(){
        "SinglePartQuery" => {
            println!("found rule={:?}", currRule);
        }
        "ProjectionItem" => {
            println!("parsing into AST");
            let ast = build_ast_from_expr(pair.clone());
        }
        "Match" => {
            let pair_result = trim_whitespace(pair.as_str());
            // println!("Found the match clause: {:?}", pair_result);
            query_info.insert("match".to_string(), pair_result);
            // process_relationship(pair.clone());
        }
        "Return" => {
            let pair_result = trim_whitespace(pair.as_str());
            // println!("Found the return clause: {:?}", pair_result);
            query_info.insert("return".to_string(), pair_result);
        }
        "RelationshipTypes" => {
            let pair_result = trim_whitespace(pair.as_str());
            // println!("Found the relationship types: {:?}", pair_result);
            query_info.insert("relationship_types".to_string(), pair_result);
        }
        "NodePattern" => {
            let pair_result = trim_whitespace(pair.as_str());
            // println!("Found the node pattern: {:?}", pair_result);
            if !query_info.contains_key("node_pattern") {
                query_info.insert("node_pattern".to_string(), pair_result);
            } else {
                let mut node_pattern = query_info.get_mut("node_pattern").unwrap();
                node_pattern.push_str(", ");
                node_pattern.push_str(pair_result.as_str());
                // query_info.insert("node_pattern".to_string(), node_pattern.to_string());
            }
        }
        _ => (),
    }
    let pad = " ".repeat(depth);
    // println!("{}- Rule: {:?}", pad, pair.as_rule());
    // println!("{}- {:?} {:?}", pad, pair.as_rule(), pair.as_str());

    pair
        .into_inner()
        .map(|p: Pair<Rule>| _process_pairs(p, depth + 1, &mut query_info))
        .count();
}


// pub fn process_relationship(pair: Pair<Rule>) {
//     if pair.as_rule() == Rule::whitespace {
//         return;
//     }
//     let currRule = format_string(pair.as_rule());
//     let mut query_info:HashMap<String, String> = HashMap::new();
//     println!("Curr rule: {:?}", currRule);
//     let p = pair
//         .into_iter()
//         .next()
//         .expect("An error!");
//     _process_relationship(p.clone(), &mut query_info);
//     // if currRule == "RelationshipPattern" {
//     //     let currPattern = trim_whitespace(pair.as_str());
//     //     println!("Found the relationship pattern: {:?}", currPattern);
//     //     let mut right_to_left = true;
//     //     if currPattern.contains("<-") {
//     //         right_to_left = false;
//     //     }
//     //     // let mut rel = Relationship::new("".to_string(), /* String */, /* String */, /* HashMap<String, String> */);

//     //     // return rel;
//     // }
//     // None
// }

// fn _process_relationship(pair: Pair<Rule>, mut query_info: &mut HashMap<String, String>) {
//     let currRule = format_string(pair.as_rule());
//     if currRule == "RelationshipPattern" {
//         let currPattern = trim_whitespace(pair.as_str());
//         let (prefix, value) = match currPattern.split_once(':') {
//             Some((prefix, value)) => (prefix, value),
//             None => ("", ""),
//         };
//         query_info.insert("relationship_variable".to_string(), prefix.to_string());
//         query_info.insert("relationship_label".to_string(), value.to_string());
//     } else if currRule == "Variable" {
//         let currPattern = trim_whitespace(pair.as_str());
//         if !query_info.contains_key("node_variable_1") {
//             query_info.insert("node_variable_1".to_string(), currPattern.to_string());
//         } else {
//             query_info.insert("node_variable_2".to_string(), currPattern.to_string());
//         }
//     } else if currRule == "LabelName" {
//         let label_name = trim_whitespace(pair.as_str());
//         if !query_info.contains_key("node_label_1") {
//             query_info.insert("node_label_1".to_string(), label_name.to_string());
//         } else {
//             query_info.insert("node_label_2".to_string(), label_name.to_string());
//         }
//     } else if currRule == "RightArrowHead" || currRule == "LeftArrowHead" {
//         if currRule == "RightArrowHead" {
//             query_info.insert("direction".to_string(), "right".to_string());
//         } else {
//             query_info.insert("direction".to_string(), "left".to_string());
//         }
//     } else if currRule == "Properties" {
//         let properties = trim_whitespace(pair.as_str());
//         if !query_info.contains_key("node_properties_1") {
//             query_info.insert("node_properties_1".to_string(), properties.to_string());
//         } else {
//             query_info.insert("node_properties_2".to_string(), properties.to_string());
//         }
//     }
//     pair.into_inner()
//         .map(|p: Pair<Rule>| _process_relationship(p, &mut query_info))
//         .count();
// }

pub fn print_pairs(pairs: Pairs<Rule>) {
    let p = pairs
        .into_iter()
        .next()
        .expect("An error!");
    _print_pairs(p, 0);
}

fn format_string(rule: Rule) -> String {
    format!("{:?}", rule)
}

fn trim_whitespace(s: &str) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}
fn _print_pairs(pair: Pair<Rule>, depth: usize) {
    // If its a whitespace, skip it
    if pair.as_rule() == Rule::whitespace {
        return;
    }
    println!("depth={}", depth);
    // if depth == 4 {
    //     std::process::exit(1);
    // }
    let pad = " ".repeat(depth);
    println!("{}- Rule: {:?}", pad, pair.as_rule());
    println!("{}- {:?} {:?}", pad, pair.as_rule(), pair.as_str());

    pair
        .into_inner()
        .map(|p: Pair<Rule>| _print_pairs(p, depth + 1))
        .count();
}


// #[derive(PartialEq, Debug, Default)]
// pub struct Node {
//     variable: String,
//     labels: Vec<String>,
//     properties: HashMap<String, String>,
// }


// impl Node {
//     pub fn new(
//         variable: String,
//         labels: Vec<String>,
//         properties: HashMap<String, String>,
//     ) -> Self {
//         Self {
//             variable,
//             labels,
//             properties,
//         }
//     }

//     // Getters
//     pub fn variable(&self) -> &str {
//         self.variable.as_str()
//     }

//     pub fn labels(&self) -> impl Iterator<Item = &str> {
//         self.labels.iter().map(|label| (*label).as_str())
//     }

//     pub fn properties(&self) -> impl Iterator<Item = (&str, &String)> {
//         self.properties.iter().map(|(k, v)| (k.as_str(), v))
//     }

// }

// #[derive(PartialEq, Debug, Default)]
// struct Relationship {
//     variable: String,
//     source: String,
//     target: String,
//     properties: HashMap<String, String>,
// }

// impl Relationship {
//     pub fn new(
//         variable: String,
//         source: String,
//         target: String,
//         properties: HashMap<String, String>,
//     ) -> Self {
//         Self {
//             variable,
//             source,
//             target,
//             properties,
//         }
//     }

//     pub fn variable(&self) -> &str {
//         self.variable.as_str()
//     }
 
//     pub fn source(&self) -> &str {
//         self.source.as_str()
//     }

//     pub fn target(&self) -> &str {
//         self.target.as_str()
//     }

//     pub fn properties(&self) -> impl Iterator<Item = (&str, &String)> {
//         self.properties.iter().map(|(k, v)| (k.as_str(), v))
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum CypherValue {
//     Float(f64),
//     Integer(i64),
//     String(String),
//     Boolean(bool),
// }

// impl From<f64> for CypherValue {
//     fn from(value: f64) -> Self {
//         CypherValue::Float(value)
//     }
// }

// impl From<i64> for CypherValue {
//     fn from(value: i64) -> Self {
//         CypherValue::Integer(value)
//     }
// }

// impl From<String> for CypherValue {
//     fn from(value: String) -> Self {
//         CypherValue::String(value)
//     }
// }

// impl From<&str> for CypherValue {
//     fn from(value: &str) -> Self {
//         CypherValue::String(value.into())
//     }
// }

// impl From<bool> for CypherValue {
//     fn from(value: bool) -> Self {
//         CypherValue::Boolean(value)
//     }
// }