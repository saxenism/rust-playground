mod example;
mod report_generation;

use std::collections::HashSet;
use std::vec;

use solang_parser::pt;

#[derive(Clone)]
enum Node {
    Statement(pt::Statement),
    Expression(pt::Expression),
    ContractPart(pt::ContractPart),
    SourceUnit(pt::SourceUnit),
    SourceUnitPart(pt::SourceUnitPart)
}

impl Node {
    pub fn as_identifier(&self) -> Identifier {
        match self {
            Self::Expression(expression) => return convert_expressions_into_taintoooor_identifier(expression),
            Self::Statement(statement) => return convert_statements_into_taintoooor_identifier(statement),
            Self::ContractPart(contract_part) => return convert_contract_part_into_taintoooor_identifier(contract_part),
            Self::SourceUnit(_) => return Identifier::SourceUnit,
            Self::SourceUnitPart(source_unit_part) => {
                return convert_source_unit_into_taintoooor_identifier(source_unit_part)
            }
        }
    }
}

impl Into<Node> for pt::Statement {
    fn into(self) -> Node {
        Node::Statement(self)
    }
}

impl Into<Node> for Box<pt::Statement> {
    fn into(self) -> Node {
        Node::Statement(*self)
    }
}

impl Into<Node> for pt::Expression {
    fn into(self) -> Node {
        Node::Expression(self)
    }
}

impl Into<Node> for Box<pt::Expression> {
    fn into(self) -> Node {
        Node::Expression(*self)
    }
}

impl Into<Node> for pt::ContractPart {
    fn into(self) -> Node {
        Node::ContractPart(self)
    }
}

impl Into<Node> for pt::SourceUnitPart {
    fn into(self) -> Node {
        Node::SourceUnitPart(self)
    }
}

impl Into<Node> for pt::SourceUnit {
    fn into(self) -> Node {
        Node::SourceUnit(self)
    }
}

#[derive(Hash, Eq, PartialEq)]
enum Identifier {
    //Statement Identifiers
    Args,
    Return,
    Revert,
    RevertNamedArgs,
    Emit,
    Expression,
    Block,
    If,
    While,
    For,
    DoWhile,
    Try,

    //Expression Identifiers
    Add,
    And,
    ArrayLiteral,
    ArraySlice,
    ArraySubscript,
    Assign,
    AssignAdd,
    AssignAnd,
    AssignDivide,
    AssignModulo,
    AssignMultiply,
    AssignOr,
    AssignShiftLeft,
    AssignShiftRight,
    AssignSubtract,
    AssignXor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    ConditionalOperator,
    Complement,
    Delete,
    Divide,
    Equal,
    FunctionCall,
    FunctionCallBlock,
    Less,
    LessEqual,
    List,
    MemberAccess,
    Modulo,
    More,
    MoreEqual,
    Multiply,
    NamedFunctionCall,
    New,
    Not,
    NotEqual,
    Or,
    Parenthesis,
    PostDecrement,
    PostIncrement,
    PreIncrement,
    PreDecrement,
    ShiftLeft,
    ShiftRight,
    Subtract,
    Type,
    Function,
    UnaryMinus,
    UnaryPlus,
    Unit,
    Power,
    BoolLiteral,
    NumberLiteral,
    RationalNumberLiteral,
    HexNumberLiteral,
    HexLiteral,
    StringLiteral,
    AddressLiteral,
    Variable,
    This,

    // from the solang_parser::pt module
    Annotation,
    SourceUnit,
    ContractDefinition,
    EnumDefinition,
    EventDefinition,
    ErrorDefinition,
    FunctionDefinition,
    ImportDirective,
    PragmaDirective,
    StraySemicolon,
    StructDefinition,
    TypeDefinition,
    Using,
    VariableDefinition, 

    // if you are having a bad day
    None
}

// This function is required to be able to parse the abstract syntax tree and get the desired identifier out of it.
fn convert_source_unit_into_taintoooor_identifier(
    source_unit_part: &pt::SourceUnitPart
) -> Identifier {
    match source_unit_part {
        pt::SourceUnitPart::Annotation(_) => Identifier::Annotation,
        pt::SourceUnitPart::ContractDefinition(_) => Identifier::ContractDefinition,
        pt::SourceUnitPart::EnumDefinition(_) => Identifier::EnumDefinition,
        pt::SourceUnitPart::ErrorDefinition(_) => Identifier::ErrorDefinition,
        pt::SourceUnitPart::EventDefinition(_) => Identifier::EventDefinition,
        pt::SourceUnitPart::FunctionDefinition(_) => Identifier::FunctionDefinition,
        pt::SourceUnitPart::ImportDirective(_) => Identifier::ImportDirective,
        pt::SourceUnitPart::PragmaDirective(_, _, _) => Identifier::PragmaDirective,
        pt::SourceUnitPart::StraySemicolon(_) => Identifier::StraySemicolon,
        pt::SourceUnitPart::StructDefinition(_) => Identifier::StructDefinition,
        pt::SourceUnitPart::TypeDefinition(_) => Identifier::TypeDefinition,
        pt::SourceUnitPart::Using(_) => Identifier::Using,
        pt::SourceUnitPart::VariableDefinition(_) => Identifier::VariableDefinition
    }
}

fn convert_contract_part_into_taintoooor_identifier(
    contract_part: &pt::ContractPart
) -> Identifier {
    match contract_part {
        pt::ContractPart::Annotation(_) => Identifier::Annotation,
        pt::ContractPart::EnumDefinition(_) => Identifier::EnumDefinition,
        pt::ContractPart::ErrorDefinition(_) => Identifier::ErrorDefinition,
        pt::ContractPart::EventDefinition(_) => Identifier::EventDefinition,
        pt::ContractPart::FunctionDefinition(_) => Identifier::FunctionDefinition,
        pt::ContractPart::StraySemicolon(_) => Identifier::StraySemicolon,
        pt::ContractPart::StructDefinition(_) => Identifier::StructDefinition,
        pt::ContractPart::TypeDefinition(_) => Identifier::TypeDefinition,
        pt::ContractPart::Using(_) => Identifier::Using,
        pt::ContractPart::VariableDefinition(_) => Identifier::VariableDefinition,
    }
}

fn convert_expressions_into_taintoooor_identifier(
    expression: &pt::Expression
) -> Identifier {
    match expression {
        pt::Expression::Add(_, _, _) => Identifier::Add,
        pt::Expression::And(_, _, _) => Identifier::And,
        pt::Expression::ArrayLiteral(_, _) => Identifier::ArrayLiteral,
        pt::Expression::ArraySlice(_, _, _, _) => Identifier::ArraySlice,
        pt::Expression::ArraySubscript(_, _, _) => Identifier::ArraySubscript,
        pt::Expression::Assign(_, _, _) => Identifier::Assign,
        pt::Expression::AssignAdd(_, _, _) => Identifier::AssignAdd,
        pt::Expression::AssignAnd(_, _, _) => Identifier::AssignAnd,
        pt::Expression::AssignDivide(_, _, _) => Identifier::AssignDivide,
        pt::Expression::AssignModulo(_, _, _) => Identifier::AssignModulo,
        pt::Expression::AssignMultiply(_, _, _) => Identifier::AssignMultiply,
        pt::Expression::AssignOr(_, _, _) => Identifier::AssignOr,
        pt::Expression::AssignShiftLeft(_, _, _) => Identifier::AssignShiftLeft,
        pt::Expression::AssignShiftRight(_, _, _) => Identifier::AssignShiftRight,
        pt::Expression::AssignSubtract(_, _, _) => Identifier::AssignSubtract,
        pt::Expression::AssignXor(_, _, _) => Identifier::AssignXor,
        pt::Expression::BitwiseAnd(_, _, _) => Identifier::BitwiseAnd,
        pt::Expression::BitwiseOr(_, _, _) => Identifier::BitwiseOr,
        pt::Expression::BitwiseXor(_, _, _) => Identifier::BitwiseXor,
        pt::Expression::Complement(_, _) => Identifier::Complement,
        pt::Expression::ConditionalOperator(_, _, _, _) => Identifier::ConditionalOperator,
        pt::Expression::Delete(_, _) => Identifier::Delete,
        pt::Expression::Divide(_, _, _) => Identifier::Divide,
        pt::Expression::Equal(_, _, _) => Identifier::Equal,
        pt::Expression::FunctionCall(_, _, _) => Identifier::FunctionCall,
        pt::Expression::FunctionCallBlock(_, _, _) => Identifier::FunctionCallBlock,
        pt::Expression::Less(_, _, _) => Identifier::Less,
        pt::Expression::LessEqual(_, _, _) => Identifier::LessEqual,
        pt::Expression::List(_, _) => Identifier::List,
        pt::Expression::MemberAccess(_, _, _) => Identifier::MemberAccess,
        pt::Expression::Modulo(_, _, _) => Identifier::Modulo,
        pt::Expression::More(_, _, _) => Identifier::More,
        pt::Expression::MoreEqual(_, _, _) => Identifier::MoreEqual,
        pt::Expression::Multiply(_, _, _) => Identifier::Multiply,
        pt::Expression::NamedFunctionCall(_, _, _) => Identifier::NamedFunctionCall,
        pt::Expression::New(_, _) => Identifier::New,
        pt::Expression::Not(_, _) => Identifier::Not,
        pt::Expression::NotEqual(_, _, _) => Identifier::NotEqual,
        pt::Expression::Or(_, _, _) => Identifier::Or,
        pt::Expression::Parenthesis(_, _) => Identifier::Parenthesis,
        pt::Expression::PostDecrement(_, _) => Identifier::PostDecrement,
        pt::Expression::PostIncrement(_, _) => Identifier::PostIncrement,
        pt::Expression::ShiftLeft(_, _, _) => Identifier::ShiftLeft,
        pt::Expression::ShiftRight(_, _, _) => Identifier::ShiftRight,
        pt::Expression::Subtract(_, _, _) => Identifier::Subtract,
        pt::Expression::Type(_, _) => Identifier::Type,
        pt::Expression::UnaryMinus(_, _) => Identifier::UnaryMinus,
        pt::Expression::UnaryPlus(_, _) => Identifier::UnaryPlus,
        pt::Expression::Unit(_, _, _) => Identifier::Unit,
        pt::Expression::PreIncrement(_, _) => Identifier::PreIncrement,
        pt::Expression::PreDecrement(_, _) => Identifier::PreDecrement,
        pt::Expression::Power(_, _, _) => Identifier::Power,
        pt::Expression::BoolLiteral(_, _) => Identifier::BoolLiteral,
        pt::Expression::NumberLiteral(_, _, _) => Identifier::NumberLiteral,
        pt::Expression::RationalNumberLiteral(_, _, _, _) => Identifier::RationalNumberLiteral,
        pt::Expression::HexNumberLiteral(_, _) => Identifier::HexNumberLiteral,
        pt::Expression::HexLiteral(_) => Identifier::HexLiteral,
        pt::Expression::StringLiteral(_) => Identifier::StringLiteral,
        pt::Expression::AddressLiteral(_, _) => Identifier::AddressLiteral,
        pt::Expression::Variable(_) => Identifier::Variable,
        pt::Expression::This(_) => Identifier::This,
    }
}

fn convert_statements_into_taintoooor_identifier(
    statement: &pt::Statement
) -> Identifier {
    match statement {
        pt::Statement::Args(_, _) => return Identifier::Args,
        pt::Statement::Return(_, _) => return Identifier::Return,
        pt::Statement::Revert(_, _, _) => return Identifier::Revert,
        pt::Statement::Emit(_, _) => return Identifier::Emit,
        pt::Statement::RevertNamedArgs(_, _, _) => return Identifier::RevertNamedArgs,
        pt::Statement::Expression(_, _) => return Identifier::Expression,
        pt::Statement::VariableDefinition(_, _, _) => return Identifier::VariableDefinition,
        pt::Statement::Block {
            loc: _,
            unchecked: _,
            statements: _,
        } => return Identifier::Block,
        pt::Statement::If(_, _, _, _) => return Identifier::If,
        pt::Statement::While(_, _, _) => return Identifier::While,
        pt::Statement::For(_, _, _, _, _) => return Identifier::For,
        pt::Statement::DoWhile(_, _, _) => return Identifier::DoWhile,
        pt::Statement::Try(_, _, _, _) => return Identifier::Try,
        _ => return Identifier::None,
    }
}

fn get_identifier_from_node(identifier: Identifier, node: Node) -> Vec<Node> {
    let mut target_set = HashSet::new();
    target_set.insert(identifier);

    return traverse_node_for_identifiers(&target_set, node);
}

fn traverse_node_for_identifiers(identifiers: &HashSet<Identifier>, node: Node) -> Vec<Node> {
    let mut matches = vec![];

    if identifiers.contains(&node.as_identifier()) {
        matches.push(node.clone());
    }

    match node {
        // This is basically breaking down SourceUnit into SourceUnitPart via recursion
        Node::SourceUnit(source_unit) => {
            for source_unit_part in source_unit.0 {
                matches.append(&mut traverse_node_for_identifiers(identifiers, source_unit_part.into()));
            }
        },
        Node::SourceUnitPart(source_unit_part) => match source_unit_part {
            pt::SourceUnitPart::ContractDefinition(box_contract_definition) => {
                // Traverse contract definition for wanted indentifiers
                for base in box_contract_definition.base {
                    if base.args.is_some() {
                        let args = base.args.unwrap();

                        for arg in args {
                            matches.append(&mut traverse_node_for_identifiers(identifiers, arg.into()));
                        }
                    }
                }
            }
        }
    }

    matches
}

fn parse_contract() {
    let contract_text:&str = r#"        
    pragma solidity ^0.8.16;

    // This contract is written by Marvin's soldiers

    contract SimpleStore {
        uint x;

        // Should probably change this to onlyOwner from public
        function set(uint newValue) public {
            x = newValue;
        }
        
        // Can be declared as view
        function get() returns (uint) {
            return x;
        }
    }"#;

    let source_unit = solang_parser::parse(contract_text, 0).unwrap().0;

    // println!("{:#?}", source_unit);

    let source_unit_items = source_unit.0;

    for item in source_unit_items.iter() {
        println!("{:#?}", item);
    }

}

fn main() {
    // example::view_example_contract();

    example::view_parsed_example_contract();

    // report_generation::generate_report();

    parse_contract();

    // let is_keyword = solang_parser::lexer::is_keyword("struct");
    // println!("{}", is_keyword);
}