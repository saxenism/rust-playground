mod example;
mod report_generation;

fn main() {
    example::view_example_contract();

    //example::view_parsed_example_contract();

    report_generation::generate_report();
}