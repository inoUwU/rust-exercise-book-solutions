use assert_cmd::Command;
// use std::process::Command;

// #[test]
// fn works() {
//     let mut cmd = Command::new("ls");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }
//

// memo
// #[test]を指定するとCargo testで実行される
// Rustでは標準ライブラリとして外部コマンドを std::process::Command で実行可能
// assert系も標準である。https://doc.rust-jp.rs/book-ja/ch11-01-writing-tests.html

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}
