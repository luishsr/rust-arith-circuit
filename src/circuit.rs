use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub enum Gate {
    Input(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Const(i32),
}

#[derive(Debug)]
pub struct Node {
    gate: Gate,
    value: Option<i32>,
}

impl Node {
    pub fn new(gate: Gate) -> Self {
        Node { gate, value: None }
    }

    pub fn evaluate(&mut self, inputs: &HashMap<String, i32>) -> i32 {
        if let Some(val) = self.value {
            return val;
        }

        let result = match &mut self.gate {
            Gate::Input(name) => *inputs.get(name).unwrap(),
            Gate::Add(left, right) => left.evaluate(inputs) + right.evaluate(inputs),
            Gate::Sub(left, right) => left.evaluate(inputs) - right.evaluate(inputs),
            Gate::Mul(left, right) => left.evaluate(inputs) * right.evaluate(inputs),
            Gate::Div(left, right) => left.evaluate(inputs) / right.evaluate(inputs),
            Gate::Const(val) => *val,
        };

        self.value = Some(result);
        result
    }

    pub fn to_solidity(&self, counter: &mut usize) -> String {
        match &self.gate {
            Gate::Input(name) => format!("{}[{}]", name, counter),
            Gate::Add(left, right) => {
                let left_sol = left.to_solidity(counter);
                *counter += 1;
                let right_sol = right.to_solidity(counter);
                *counter += 1;
                format!("{} + {}", left_sol, right_sol)
            }
            Gate::Sub(left, right) => {
                let left_sol = left.to_solidity(counter);
                *counter += 1;
                let right_sol = right.to_solidity(counter);
                *counter += 1;
                format!("{} - {}", left_sol, right_sol)
            }
            Gate::Mul(left, right) => {
                let left_sol = left.to_solidity(counter);
                *counter += 1;
                let right_sol = right.to_solidity(counter);
                *counter += 1;
                format!("{} * {}", left_sol, right_sol)
            }
            Gate::Div(left, right) => {
                let left_sol = left.to_solidity(counter);
                *counter += 1;
                let right_sol = right.to_solidity(counter);
                *counter += 1;
                format!("{} / {}", left_sol, right_sol)
            }
            Gate::Const(val) => format!("{}", val),
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            gate: match &self.gate {
                Gate::Input(name) => Gate::Input(name.clone()),
                Gate::Add(left, right) => Gate::Add(left.clone(), right.clone()),
                Gate::Sub(left, right) => Gate::Sub(left.clone(), right.clone()),
                Gate::Mul(left, right) => Gate::Mul(left.clone(), right.clone()),
                Gate::Div(left, right) => Gate::Div(left.clone(), right.clone()),
                Gate::Const(val) => Gate::Const(*val),
            },
            value: self.value,
        }
    }
}

pub struct ArithmeticCircuit {
    root: Node,
}

impl ArithmeticCircuit {
    pub fn new(root: Node) -> Self {
        ArithmeticCircuit { root }
    }

    pub fn from_expression(expr: &str) -> Self {
        let root = Self::parse_expression(expr).expect("Failed to parse expression").1;
        ArithmeticCircuit::new(root)
    }

    fn parse_expression(input: &str) -> IResult<&str, Node> {
        let (input, init) = Self::parse_term(input)?;
        fold_many0(
            pair(alt((char('+'), char('-'))), Self::parse_term),
            move || init.clone(),
            |acc, (op, val): (char, Node)| {
                if op == '+' {
                    Node::new(Gate::Add(Box::new(acc), Box::new(val)))
                } else {
                    Node::new(Gate::Sub(Box::new(acc), Box::new(val)))
                }
            },
        )(input)
    }

    fn parse_term(input: &str) -> IResult<&str, Node> {
        let (input, init) = Self::parse_factor(input)?;
        fold_many0(
            pair(alt((char('*'), char('/'))), Self::parse_factor),
            move || init.clone(),
            |acc, (op, val): (char, Node)| {
                if op == '*' {
                    Node::new(Gate::Mul(Box::new(acc), Box::new(val)))
                } else {
                    Node::new(Gate::Div(Box::new(acc), Box::new(val)))
                }
            },
        )(input)
    }

    fn parse_factor(input: &str) -> IResult<&str, Node> {
        alt((
            map_res(digit1, |digit_str: &str| {
                i32::from_str(digit_str).map(|val| Node::new(Gate::Const(val)))
            }),
            map(Self::parse_identifier, |id: &str| {
                Node::new(Gate::Input(id.to_string()))
            }),
            delimited(
                char('('),
                Self::parse_expression,
                char(')'),
            ),
        ))(input)
    }

    fn parse_identifier(input: &str) -> IResult<&str, &str> {
        nom::character::complete::alpha1(input)
    }

    pub fn evaluate(&mut self, inputs: &[(&str, i32)]) -> i32 {
        let input_map: HashMap<String, i32> = inputs.iter().cloned().map(|(k, v)| (k.to_string(), v)).collect();
        self.root.evaluate(&input_map)
    }

    pub fn to_solidity(&self) -> String {
        let mut counter = 0;
        let body = self.root.to_solidity(&mut counter);
        format!(
            r#"
pragma solidity ^0.8.0;

contract ArithmeticCircuit {{
    function verify(int256[] memory x, int256[] memory y) public pure returns (int256) {{
        return {};
    }}
}}
"#,
            body
        )
    }
}
