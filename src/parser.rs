extern crate pest;
extern crate pest_derive;

use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pairs, Pair};
use pest_derive::Parser;
use std::{collections::HashMap, fmt, str::FromStr};


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



pub fn process_pairs(pair: Pairs<Rule>) {
    println!("process_pairs");
    println!("pair={:?}", pair);
    // println!("rule={:?},str={:?}", pair.as_rule(), pair.as_str());
}

pub fn print_pairs(pairs: Pairs<Rule>) {
    let p = pairs
        .into_iter()
        .next()
        .expect("An error!");
    _print_pairs(p, 0);
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

pub(crate) struct Node {
    pub(crate) variable: Option<String>,
    pub(crate) labels: Vec<String>,
    pub(crate) properties: HashMap<String, CypherValue>,
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