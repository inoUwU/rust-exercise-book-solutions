use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("USAGE"));
    Ok(())
}

// ヘルパー関数
fn run(args: &[&str], expected_file: &str) -> TestResult {
    // TODO: スライスとは何か調べる
    let expected = fs::read_to_string(expected_file)?;
    let mut cmd = Command::cargo_bin("echor")?;
    // argsを使用する複数の値を取れる
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

// 出力する値と指定したファイルの内容が同一であかことを確認するテスト
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["Hello", "there", "-n"], "tests/expected/hello2.n.txt")
}
