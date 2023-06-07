use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    As,
    Return,
    Relationship,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::As => write!(f, "AS"),
            Operator::Return => write!(f, "RETURN"),
            Operator::Relationship => write!(f, "->"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: node
pub enum Node {
    Str(String),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    RelationshipExpr {
        rel_type: Operator,
        from: Box<Node>,
        to: Box<Node>,
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Str(s) => write!(f, "{}", s),
            Node::UnaryExpr { op, child } => write!(f, "{}{}", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
            Node::RelationshipExpr { rel_type, from, to } => write!(f, "{} {} {}", from, rel_type, to),
        }
    }
}
