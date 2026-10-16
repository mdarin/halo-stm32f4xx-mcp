use std::io::Write;
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_toolkit");
const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/usart.c");
const FIXTURE_CCM_DMA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/ccm_dma.c");

fn run_with_stdin(args: &[&str], input: &str) -> (Option<i32>, String, String) {
    let mut child = Command::new(BIN)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn toolkit");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.as_bytes())
        .expect("write stdin");
    let out = child.wait_with_output().expect("wait");
    (
        out.status.code(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

fn version_output() -> String {
    let out = Command::new(BIN)
        .arg("--version")
        .output()
        .expect("--version");
    assert!(out.status.success());
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn version_reports_contract() {
    let v = version_output();
    assert!(v.starts_with("toolkit 0.1.0 (contract 1.0)"), "got: {v}");
}

#[test]
fn analyze_file_emits_valid_json() {
    let (code, stdout, stderr) = run_with_stdin(&["analyze", "--file", FIXTURE], "");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["schema_version"], 1);
    assert_eq!(resp["command"], "analyze");
    assert!(resp["source_stats"]["bytes"].as_u64().unwrap() > 0);
    assert!(resp["source_stats"]["lines"].as_u64().unwrap() > 0);
}

#[test]
fn analyze_stdin_emits_valid_json() {
    let (code, stdout, stderr) = run_with_stdin(&["analyze"], "int foo(void) { return 1; }");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["command"], "analyze");
}

#[test]
fn missing_file_is_json_error_on_stderr() {
    let (code, stdout, stderr) = run_with_stdin(&["analyze", "--file", "/no/such/file.c"], "");
    assert_ne!(code, Some(0));
    assert!(stdout.trim().is_empty());
    let err: serde_json::Value = serde_json::from_str(stderr.trim()).expect("JSON error on stderr");
    assert_eq!(err["schema_version"], 1);
    assert_eq!(err["error"]["code"], "E_INPUT_READ");
}

#[test]
fn unknown_args_exit_nonzero() {
    let out = Command::new(BIN)
        .arg("--definitely-not-a-flag")
        .output()
        .expect("run");
    assert_ne!(out.status.code(), Some(0));
}

#[test]
fn lint_response_matches_contract_shape() {
    let (code, stdout, stderr) = run_with_stdin(&["lint"], "static int x = 0;");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["command"], "lint");
    assert!(resp["violations"].is_array());
    assert!(resp["flows"].is_null());
    assert_eq!(resp["summary"]["errors"], 0);

    let (code, stdout, _) = run_with_stdin(&["lint", "--flow"], "static int x = 0;");
    assert_eq!(code, Some(0));
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert!(resp["flows"].is_array());
}

#[test]
fn flow_analysis_alias_works() {
    let (code, stdout, stderr) = run_with_stdin(&["flow-analysis"], "int main(void){return 0;}");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["command"], "flow-analysis");
    assert!(resp["flows"].is_array());
}

#[test]
fn generate_docs_returns_insertions() {
    let (code, stdout, stderr) = run_with_stdin(&["generate-docs"], "int foo(void);");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["command"], "generate-docs");
    assert!(resp["insertions"].is_array());
}

#[test]
fn rag_search_contract_shape() {
    let (code, stdout, stderr) = run_with_stdin(&["rag-search"], "int main(void){return 0;}");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["command"], "rag-search");
    assert!(resp["violations"].is_array());
    assert!(resp["rag_refs"].is_array());
    assert_eq!(resp["rag_degraded"], false);
}

#[test]
fn lint_detects_ccm_dma_violation_in_fixture() {
    let (code, stdout, stderr) = run_with_stdin(&["lint", "--file", FIXTURE_CCM_DMA], "");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    assert_eq!(resp["summary"]["errors"], 1);
    let v = &resp["violations"][0];
    assert_eq!(v["rule"], "dma_buffer_in_ccmram");
    assert_eq!(v["severity"], "error");
    assert_eq!(v["position"]["line"], 1);
    assert_eq!(
        v["hint"],
        "Move buffer to .axisram / .sram1-3, or use DMA2D/MDMA"
    );
}

#[test]
fn lint_flow_analysis_reports_reaches_dma() {
    let (code, stdout, stderr) = run_with_stdin(&["flow-analysis", "--file", FIXTURE_CCM_DMA], "");
    assert_eq!(code, Some(0), "stderr: {stderr}");
    let resp: serde_json::Value = serde_json::from_str(stdout.trim()).expect("valid JSON");
    let flow = &resp["flows"][0];
    assert_eq!(flow["variable"], "adc_dma_buffer");
    assert_eq!(flow["reaches_dma"], true);
    let kinds: Vec<&str> = flow["uses"]
        .as_array()
        .unwrap()
        .iter()
        .map(|u| u["kind"].as_str().unwrap())
        .collect();
    assert!(kinds.contains(&"declaration"));
    assert!(kinds.contains(&"dma_call"));
}
