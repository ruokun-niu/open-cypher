extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pairs, Pair};
use pest_derive::Parser;
use std::{collections::HashMap, str::FromStr};


#[derive(Parser)]
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

#[derive(PartialEq, Debug, Default)]
pub struct Node {
    variable: String,
    labels: Vec<String>,
    properties: HashMap<String, String>,
}


impl Node {
    pub fn new(
        variable: String,
        labels: Vec<String>,
        properties: HashMap<String, String>,
    ) -> Self {
        Self {
            variable,
            labels,
            properties,
        }
    }

    // Getters
    pub fn variable(&self) -> &str {
        self.variable.as_str()
    }

    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.labels.iter().map(|label| (*label).as_str())
    }

    pub fn properties(&self) -> impl Iterator<Item = (&str, &String)> {
        self.properties.iter().map(|(k, v)| (k.as_str(), v))
    }

}

#[derive(PartialEq, Debug, Default)]
struct Relationship {
    variable: String,
    source: String,
    target: String,
    properties: HashMap<String, String>,
}

impl Relationship {
    pub fn new(
        variable: String,
        source: String,
        target: String,
        properties: HashMap<String, String>,
    ) -> Self {
        Self {
            variable,
            source,
            target,
            properties,
        }
    }

    pub fn variable(&self) -> &str {
        self.variable.as_str()
    }
 
    pub fn source(&self) -> &str {
        self.source.as_str()
    }

    pub fn target(&self) -> &str {
        self.target.as_str()
    }

    pub fn properties(&self) -> impl Iterator<Item = (&str, &String)> {
        self.properties.iter().map(|(k, v)| (k.as_str(), v))
    }
}


pub fn test_struct() -> Node {
    return Node::new(
        "n".to_string(),
        vec!["Person".to_string(), "Car".to_string()],
        HashMap::new(),
    );
}

pub fn process_pairs(pairs: Pairs<Rule>) {
    let mut query_info:HashMap<String, String> = HashMap::new();
    println!("process_pairs");
    let p = pairs
        .into_iter()
        .next()
        .expect("An error!");
    _process_pairs(p, 0, &mut query_info);
}

pub fn _process_pairs(pair: Pair<Rule>, depth: usize, mut query_info: &mut HashMap<String, String>) {
    if pair.as_rule() == Rule::whitespace {
        return;
    }

    // println!("depth={}", depth);
    let currRule = format_string(pair.as_rule());
    match currRule.as_str(){
        "SinglePartQuery" => {
            query_info.insert("Query type".to_string(), currRule.to_string());
            println!("found rule={:?}", currRule);
        }
        "Match" => {
            let pair_result = trim_whitespace(pair.as_str());
            println!("Found the match clause: {:?}", pair_result);
            query_info.insert("Match".to_string(), pair_result);
            process_relationship(pair);
        }
        "Return" => {
            let pair_result = trim_whitespace(pair.as_str());
            println!("Found the return clause: {:?}", pair_result);
            query_info.insert("Return".to_string(), pair_result);
        }
        "RelationshipTypes" => {
            let pair_result = trim_whitespace(pair.as_str());
            println!("Found the relationship types: {:?}", pair_result);
            // query_info.insert("RelationshipTypes".to_string(), pair_result);
        }
        "NodePattern" => {
            let pair_result = trim_whitespace(pair.as_str());
            println!("Found the node pattern: {:?}", pair_result);
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


pub fn process_relationship(pairs: Pair<Rule>) -> Option<&Relationship> {
    if pair.as_rule() == Rule::whitespace {
        return None;
    }
    let currRule = format_string(pair.as_rule());
    if currRule == "RelationshipPattern" {
        let currPattern = trim_whitespace(pair.as_str());
        let mut right_to_left = true;
        if currPattern.contains("<-") {
            right_to_left = false;
        }
        let mut rel = Relationship::new("".to_string(), ..Default::default());

        return rel;
    }
    None
}


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

#[derive(Debug, PartialEq)]
pub enum CypherValue {
    Float(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
}

impl From<f64> for CypherValue {
    fn from(value: f64) -> Self {
        CypherValue::Float(value)
    }
}

impl From<i64> for CypherValue {
    fn from(value: i64) -> Self {
        CypherValue::Integer(value)
    }
}

impl From<String> for CypherValue {
    fn from(value: String) -> Self {
        CypherValue::String(value)
    }
}

impl From<&str> for CypherValue {
    fn from(value: &str) -> Self {
        CypherValue::String(value.into())
    }
}

impl From<bool> for CypherValue {
    fn from(value: bool) -> Self {
        CypherValue::Boolean(value)
    }
}