use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"15000"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "65\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"75000"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "89\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(r#"200"#)
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "02\n");
    assert!(output.stderr_str().is_empty());
}
