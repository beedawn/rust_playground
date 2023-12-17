use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn hello1(){
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    println!("expected: {}",expected);    
    let mut cmd = Command::cargo_bin("echor").unwrap();
    println!("{:?}", cmd);
    cmd.arg("Hello there").assert().success().stdout(expected);
}   

