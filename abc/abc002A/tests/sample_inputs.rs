use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"10 11
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "11\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"100000000 10000000        
"#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "100000000\n");
    assert!(output.stderr_str().is_empty());
}
