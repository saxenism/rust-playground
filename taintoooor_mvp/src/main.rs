mod example;
mod report_generation;

use std::collections::HashSet;
use std::vec;

use solang_parser::pt;

#[derive(Debug, Clone)]
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

    pub fn expression(self) -> Option<pt::Expression> {
        match self {
            Self::Expression(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn statement(self) -> Option<pt::Statement> {
        match self {
            Self::Statement(statement) => Some(statement),
            _ => None,
        }
    }

    pub fn source_unit(self) -> Option<pt::SourceUnit> {
        match self {
            Self::SourceUnit(source_unit) => Some(source_unit),
            _ => None,
        }
    }

    pub fn source_unit_part(self) -> Option<pt::SourceUnitPart> {
        match self {
            Self::SourceUnitPart(source_unit_part) => Some(source_unit_part),
            _ => None,
        }
    }

    pub fn is_source_unit_part(&self) -> bool {
        if let Self::SourceUnitPart(_) = self {
            true
        } else {
            false
        }
    }

    pub fn contract_part(self) -> Option<pt::ContractPart> {
        match self {
            Self::ContractPart(contract_part) => Some(contract_part),
            _ => None,
        }
    }

    pub fn is_contract_part(&self) -> bool {
        if let Self::ContractPart(_) = self {
            true
        } else {
            false
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
    Ternary, //called `ConditionalOperator` in the newer versions,
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
    //Annotation, => This is included in the newer versions
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
        // pt::ContractPart::Annotation(_) => Identifier::Annotation,
        // No need to worry about Annotation, it is included in the newer versions

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
        pt::Expression::Ternary(_, _, _, _) => Identifier::Ternary,
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
        Node::SourceUnit(source_unit) => {
            for source_unit_part in source_unit.0 {
                // Since SourceUnit is basically a vector of SourceUnitParts, so we break it down via recursion
                matches.append(&mut traverse_node_for_identifiers(identifiers, source_unit_part.into()));
            }
        }, 
        Node::SourceUnitPart(source_unit_part) => match source_unit_part { // since source_unit_part can itself be of many types
            pt::SourceUnitPart::ContractDefinition(box_contract_definition) => {
                // Traverse the contract definition for required identifiers
                for base in box_contract_definition.base {
                    if base.args.is_some() {
                        let args = base.args.unwrap();

                        for arg in args {
                            matches.append(& mut traverse_node_for_identifiers(identifiers, arg.into()));
                        }
                    }
                }

                for part in box_contract_definition.parts {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, part.into()));
                }
            },
            pt::SourceUnitPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, error_parameter.ty.into()));
                }
            },
            pt::SourceUnitPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, event_parameter.ty.into()));
                }
            },
            pt::SourceUnitPart::FunctionDefinition(box_function_definition) => {
                // Params
                for (_, option_param) in box_function_definition.params {
                    if option_param.is_some() {
                        matches.append(&mut traverse_node_for_identifiers(identifiers, option_param.unwrap().ty.into()));
                    }
                }
                // Return params
                for(_, return_param) in box_function_definition.returns {
                    if return_param.is_some() {
                        matches.append(&mut traverse_node_for_identifiers(identifiers, return_param.unwrap().ty.into()));
                    }
                }
                // Function Body
                if box_function_definition.body.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_function_definition.body.unwrap().into()));
                }
            },
            pt::SourceUnitPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_variable_definition.ty.into()));

                if box_variable_definition.initializer.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_variable_definition.initializer.unwrap().into()));
                }
            },
            pt::SourceUnitPart::Using(box_using) => {
                if box_using.ty.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_using.ty.unwrap().into()));
                }
            },
            pt::SourceUnitPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_type_definition.ty.into()));
            },
            pt::SourceUnitPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, variable_declaration.ty.into()));
                }
            }
            _ => {
            /*
                let warning_text = r#"
                This module currently does not support discovery of the following SourceUnitPart identifiers:
                1. Pragma Directive
                2. Stray Semicolon
                3. EnumDefinition
                4. Import directive
                5. Annotation
                "#;
                println!("{}", warning_text);
            */
            }
        },
        Node::Expression(expression) => match expression {
            pt::Expression::Add(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::And(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::ArrayLiteral(_, vec_expression) => { // An ArrayLiteral is a vector of Expressions and we break that down via recursion
                for expression in vec_expression {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
                }
            },
            pt::Expression::Assign(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignAdd(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignDivide(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignModulo(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignMultiply(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignOr(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignSubtract(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::AssignXor(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::BitwiseAnd(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::BitwiseOr(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::BitwiseXor(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Complement(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Delete(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Divide(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Equal(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::FunctionCall(_, box_expression, vec_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                for expression in vec_expression {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
                }
            },
            pt::Expression::FunctionCallBlock(_, box_expression, box_statement) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                matches.append(&mut traverse_node_for_identifiers(identifiers, box_statement.into()));
            },
            pt::Expression::Less(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::LessEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::List(_, parameter_list) => {
                for (_, option_parameter) in parameter_list {
                    if option_parameter.is_some() {
                        let parameter = option_parameter.unwrap();
                        matches.append(&mut traverse_node_for_identifiers(identifiers, parameter.ty.into()));
                    }
                }
            },
            pt::Expression::MemberAccess(_, box_expression, _) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Modulo(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::More(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::MoreEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Multiply(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::NamedFunctionCall(_, box_expression, vec_named_argument) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                for named_argument in vec_named_argument {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        named_argument.expr.into(),
                    ));
                }
            },
            pt::Expression::New(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Not(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::NotEqual(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Or(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Parenthesis(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::PostDecrement(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::PostIncrement(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::ShiftLeft(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::ShiftRight(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::Subtract(_, box_expression, box_expression_1) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
            },
            pt::Expression::UnaryMinus(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::UnaryPlus(_, box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Unit(_, box_expression, _) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
            },
            pt::Expression::Type(_, ty) => match ty {
                pt::Type::Mapping(_, box_expression, box_expression_1) => {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
                }

                pt::Type::Function {
                    params,
                    attributes,
                    returns,
                } => {
                    for param in params {
                        if param.1.is_some() {
                            matches.append(&mut traverse_node_for_identifiers(
                                identifiers,
                                param.1.unwrap().ty.into(),
                            ));
                        }
                    }

                    for attribute in attributes {
                        match attribute {
                            pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                if base.args.is_some() {
                                    for arg in base.args.unwrap() {
                                        matches.append(&mut traverse_node_for_identifiers(
                                            identifiers,
                                            arg.into(),
                                        ));
                                    }
                                }
                            },
                            pt::FunctionAttribute::NameValue(_, _, expression) => {
                                matches
                                    .append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
                            },
                            _ => {}
                        }
                    }

                    if returns.is_some() {
                        let (parameter_list, function_attributes) = returns.unwrap();

                        for (_, option_parameter) in parameter_list {
                            if option_parameter.is_some() {
                                matches.append(&mut traverse_node_for_identifiers(
                                    identifiers,
                                    option_parameter.unwrap().ty.into(),
                                ));
                            }
                        }

                        for attribute in function_attributes {
                            match attribute {
                                pt::FunctionAttribute::BaseOrModifier(_, base) => {
                                    if base.args.is_some() {
                                        for arg in base.args.unwrap() {
                                            matches.append(&mut traverse_node_for_identifiers(
                                                identifiers,
                                                arg.into(),
                                            ));
                                        }
                                    }
                                }

                                pt::FunctionAttribute::NameValue(_, _, expression) => {
                                    matches.append(&mut traverse_node_for_identifiers(
                                        identifiers,
                                        expression.into(),
                                    ));
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            },
            pt::Expression::Ternary(_, box_expression, box_expression_1, box_expression_2) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_1.into()));
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression_2.into()));
            },
            pt::Expression::ArraySlice(
                _,
                box_expression,
                option_box_expression,
                option_box_expression_1,
            ) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                if option_box_expression.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_expression.unwrap().into(),
                    ));
                }

                if option_box_expression_1.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_expression_1.unwrap().into(),
                    ));
                }
            },
            pt::Expression::ArraySubscript(_, box_expression, option_box_expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_expression.into()));

                if option_box_expression.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_expression.unwrap().into(),
                    ));
                }
            },
            _ => {
            /*
                let warning_text = r#"
                    This module currently does not support discovery of the following Expression identifiers:
                    1. Address literal
                    2. Bool literal
                    3. Hex literal
                    4. Hex number literal
                    5. Number literal
                    6. Rational number literal
                    7. String literal
                    8. This
                    9. Variable
                "#;
                println!("{}", warning_text);
            */
            }
        },
        Node::Statement(statement) => match statement {
            pt::Statement::Args(_, named_arguments) => {
                for argument in named_arguments {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, argument.expr.into()));
                }
            },
            pt::Statement::Return(_, option_expression) => {
                if option_expression.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_expression.unwrap().into(),
                    ));
                }
            },
            pt::Statement::Revert(_, _, vec_expression) => {
                for expression in vec_expression {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
                }
            },
            pt::Statement::Emit(_, expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
            },
            pt::Statement::RevertNamedArgs(_, _, vec_named_arguments) => {
                for named_argument in vec_named_arguments {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        named_argument.expr.into(),
                    ));
                }
            },
            pt::Statement::Expression(_, expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
            },
            pt::Statement::VariableDefinition(_, variable_declaration, option_expression) => {
                matches.append(&mut traverse_node_for_identifiers(
                    identifiers,
                    variable_declaration.ty.into(),
                ));

                if option_expression.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_expression.unwrap().into(),
                    ));
                }
            },
            pt::Statement::Block {
                loc: _,
                unchecked: _,
                statements,
            } => {
                for statement in statements {
                    matches.append(&mut traverse_node_for_identifiers(identifiers, statement.into()));
                }
            },
            pt::Statement::If(_, expression, box_statement, option_box_statement) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));

                matches.append(&mut traverse_node_for_identifiers(identifiers, box_statement.into()));

                if option_box_statement.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_statement.unwrap().into(),
                    ));
                }
            },
            pt::Statement::While(_, expression, box_statement) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));

                matches.append(&mut traverse_node_for_identifiers(identifiers, box_statement.into()));
            },
            pt::Statement::For(
                _,
                option_box_statement,
                option_box_expression,
                option_box_statement_1,
                option_box_statement_2,
            ) => {
                if option_box_statement.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_statement.unwrap().into(),
                    ));
                }

                if option_box_expression.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_expression.unwrap().into(),
                    ));
                }

                if option_box_statement_1.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_statement_1.unwrap().into(),
                    ));
                }
                if option_box_statement_2.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        option_box_statement_2.unwrap().into(),
                    ));
                }
            },
            pt::Statement::DoWhile(_, box_statement, expression) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, box_statement.into()));

                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));
            },
            pt::Statement::Try(_, expression, option_paramlist_box_statement, _) => {
                matches.append(&mut traverse_node_for_identifiers(identifiers, expression.into()));

                if option_paramlist_box_statement.is_some() {
                    let (paramlist, box_statement) = option_paramlist_box_statement.unwrap();

                    for (_, option_param) in paramlist {
                        if option_param.is_some() {
                            matches.append(&mut traverse_node_for_identifiers(
                                identifiers,
                                option_param.unwrap().ty.into(),
                            ));
                        }
                    }

                    matches.append(&mut traverse_node_for_identifiers(identifiers, box_statement.into()));
                }
            },
            _ => {
            /*
                let warning_text = r#"
                    This module currently does not support discovery of the following Statement identifiers:
                    1. Assembly Block
                    2. Continue
                    3. Break
                "#;
                println!("{}", warning_text);
            */
            }
        },
        Node::ContractPart(contract_part) => match contract_part {
            pt::ContractPart::ErrorDefinition(box_error_definition) => {
                for error_parameter in box_error_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        error_parameter.ty.into(),
                    ));
                }
            },
            pt::ContractPart::EventDefinition(box_event_definition) => {
                for event_parameter in box_event_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        event_parameter.ty.into(),
                    ));
                }
            },
            pt::ContractPart::FunctionDefinition(box_function_definition) => {
                //traverse params for identifiers
                for (_, option_parameter) in box_function_definition.params {
                    if option_parameter.is_some() {
                        matches.append(&mut traverse_node_for_identifiers(
                            identifiers,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }
                //traverse return params for identifiers
                for (_, option_parameter) in box_function_definition.returns {
                    if option_parameter.is_some() {
                        matches.append(&mut traverse_node_for_identifiers(
                            identifiers,
                            option_parameter.unwrap().ty.into(),
                        ));
                    }
                }

                //traverse the function body for identifiers
                if box_function_definition.body.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        box_function_definition.body.unwrap().into(),
                    ));
                }
            },
            pt::ContractPart::StructDefinition(box_struct_definition) => {
                for variable_declaration in box_struct_definition.fields {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        variable_declaration.ty.into(),
                    ));
                }
            },
            pt::ContractPart::TypeDefinition(box_type_definition) => {
                matches.append(&mut traverse_node_for_identifiers(
                    identifiers,
                    box_type_definition.ty.into(),
                ));
            },
            pt::ContractPart::Using(box_using) => {
                if box_using.ty.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        box_using.ty.unwrap().into(),
                    ));
                }
            },
            pt::ContractPart::VariableDefinition(box_variable_definition) => {
                matches.append(&mut traverse_node_for_identifiers(
                    identifiers,
                    box_variable_definition.ty.into(),
                ));

                if box_variable_definition.initializer.is_some() {
                    matches.append(&mut traverse_node_for_identifiers(
                        identifiers,
                        box_variable_definition.initializer.unwrap().into(),
                    ));
                }
            },
            _ => {
            /*
                let warning_text = r#"
                    This module currently does not support discovery of the following Expression identifiers:
                    1. Stray Semicolon
                    2. Enum Definition
                "#;
                println!("{}", warning_text);
            */
            }
        },
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

fn test_function() {
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

    let target_nodes = get_identifier_from_node(Identifier::FunctionDefinition, source_unit.into());
    if target_nodes.len() == 0 {
        println!("Not Present in the contract");
    } else {
        for item in target_nodes {
            println!("{:#?}", item.is_contract_part());
        }
    }
}

fn main() {
    // example::view_example_contract();

    //example::view_parsed_example_contract();

    // report_generation::generate_report();

    //parse_contract();

    // let is_keyword = solang_parser::lexer::is_keyword("struct");
    // println!("{}", is_keyword);

    test_function();
}