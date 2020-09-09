use crate::compiler::program_parser::program;
use crate::compiler::tokens::Token;
use crate::compiler::visitor::Visitor;

use indexmap::IndexSet;
use nom::types::CompleteStr;

pub mod expression_parsers;
pub mod factor_parsers;
pub mod operand_parsers;
pub mod operator_parsers;
pub mod program_parser;
pub mod term_parsers;
pub mod tokens;
pub mod visitor;

pub struct Compiler {
    free_reg: IndexSet<u8>,
    used_reg: IndexSet<u8>,
    assembly: Vec<String>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            free_reg: (0..32).collect(),
            used_reg: IndexSet::new(),
            assembly: vec![],
        }
    }

    pub fn compile(&mut self, source: &str) -> String {
        let (_, tree) = program(CompleteStr(source)).unwrap();
        self.visit_token(&tree);
        self.assembly.join("\n")
    }
}

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            // Arithmetic
            Token::AdditionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("add %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::SubtractionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("sub %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::MultiplicationOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("mul %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::DivisionOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("div %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }

            // Comparative
            Token::EqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("eq %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::NotEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("neq %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::GreaterThanOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("gt %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::GreaterThanEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("gte %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::LessThanOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("lt %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }
            Token::LessThanEqualsOp => {
                let result_reg = self.free_reg.pop().unwrap();
                let right_reg = self.used_reg.pop().unwrap();
                let left_reg = self.used_reg.pop().unwrap();
                self.assembly
                    .push(format!("lte %{} %{} %{}", result_reg, left_reg, right_reg));
                self.free_reg.insert(left_reg);
                self.free_reg.insert(right_reg);
                self.used_reg.insert(result_reg);
            }

            Token::Compare { left, op, right } => {
                self.visit_token(left);
                self.visit_token(right);
                self.visit_token(op);
            }

            Token::Assign { ident: _, expr } => {
                // First visit the rhs to make sure we do what we need
                // to find the value we need.
                self.visit_token(expr);
                let _result_reg = self.used_reg.pop().unwrap();
                // TODO: Allocate storage for the identifier and write value to heap.

                // TODO: Map identifier name to heap offset.
            }

            Token::Real { value } => {
                let next_reg = self.free_reg.pop().unwrap();
                let line = format!("load %{} #{:.2}", next_reg, value);
                self.assembly.push(line);
                self.used_reg.insert(next_reg);
            }
            Token::Factor { ref value } => self.visit_token(value),
            Token::Term {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for factor in right {
                    self.visit_token(&factor.1);
                    self.visit_token(&factor.0);
                }
            }
            Token::Expression {
                ref left,
                ref right,
            } => {
                self.visit_token(left);
                for term in right {
                    self.visit_token(&term.1);
                    self.visit_token(&term.0);
                }
            }
            Token::Program { ref expressions } => {
                self.assembly.push(".data".into());
                self.assembly.push(".code".into());
                for expr in expressions {
                    self.visit_token(expr);
                }
                self.assembly.push("halt".into());
            }
        }
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use indexmap::indexset;

    fn generate_test_program(listing: &str) -> Token {
        let (_, tree) = program(CompleteStr(listing)).unwrap();
        tree
    }

    #[test]
    fn test_addition() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 + 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "add %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_subtraction() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 - 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "sub %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_multiplication() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 * 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "mul %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_division() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 / 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "div %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 + 4.1 eq 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #4.10",
                "add %29 %31 %30",
                "load %30 #3.40",
                "eq %31 %29 %30",
                "halt"
            ]
        );
        let expected_free: IndexSet<u8> = (0..31).collect();
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {31});
    }

    #[test]
    fn test_not_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 neq 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "neq %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_greater_than() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 gt 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "gt %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_greater_than_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 gte 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "gte %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_less_than() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 lt 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "lt %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }

    #[test]
    fn test_less_than_equals() {
        let mut compiler = Compiler::new();
        let test_program = generate_test_program("1.2 lte 3.4");
        compiler.visit_token(&test_program);
        assert_eq!(
            compiler.assembly,
            vec![
                ".data",
                ".code",
                "load %31 #1.20",
                "load %30 #3.40",
                "lte %29 %31 %30",
                "halt"
            ]
        );
        let mut expected_free: IndexSet<u8> = (0..29).collect();
        expected_free.insert(30);
        expected_free.insert(31);
        assert_eq!(compiler.free_reg, expected_free);
        assert_eq!(compiler.used_reg, indexset! {29});
    }
}