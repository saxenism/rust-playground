mod example;
mod report_generation;

use std::collections::HashSet;
use std::vec;

use solang_parser::pt;

#[derive(Clone)]
enum Node {
    SourceUnit(pt::SourceUnit),
    SourceUnitPart(pt::SourceUnitPart)
}

impl Node {
    pub fn as_identifier(&self) -> Identifier {
        match self {
            Self::SourceUnit(_) => return Identifier::SourceUnit,
            Self::SourceUnitPart(source_unit_part) => {
                return convert_source_unit_into_taintoooor_identifier(source_unit_part)
            }
        }
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
                matches.append(&mut traverse_node_for_identifiers(identifiers, source_unit_part.into()));
            }
        },
        Node::SourceUnitPart(source_unit_part) => {
            
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