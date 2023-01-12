pub fn view_example_contract() {
    let contract_text = r#"        
    pragma solidity ^0.8.16;
    contract SimpleStore {
        uint x;
        function set(uint newValue) {
            x = newValue;
        }
        
        function get() returns (uint) {
            return x;
        }
    }"#;

    println!("{}", contract_text);
}

pub fn view_parsed_example_contract() {
    let contract_text = r#"        
    pragma solidity ^0.8.16;
    contract SimpleStore {
        uint x;
        function set(uint newValue) {
            x = newValue;
        }
        
        function get() returns (uint) {
            return x;
        }
    }"#;

    let source_unit = solang_parser::parse(contract_text, 0).unwrap().0;

    println!("{:#?}", source_unit);
}