use std::path::PathBuf;

use citation::Citation;

#[test]
fn test() {
    let citation = Citation::read("./tests/CITATION.cff".into());

    println!("{:?}", citation);
}