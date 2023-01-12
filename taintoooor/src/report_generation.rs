use std::fs;

pub fn generate_report() {
    let mut taintoooor_report = String::from("");

    let report_header: &str = "# Taintoooor Taint Analysis";

    taintoooor_report.push_str(report_header);

    fs::write("taintoooor_report.md", taintoooor_report).expect("Unable to solstat_report to file");
}