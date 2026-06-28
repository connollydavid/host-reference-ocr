//! Conformance for the OCR helper: a byte-for-byte golden over a synthetic text image. The helper is
//! run-to-run deterministic on a host with the embedded models. Never auto-blessed; set
//! `HOST_REFERENCE_BLESS=1` to rewrite the golden deliberately.

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn hello_world_scan() {
    let helper = env!("CARGO_BIN_EXE_host-reference-ocr-helper");
    let base = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/scan");
    let output = Command::new(helper)
        .arg(base.join("input.png"))
        .output()
        .expect("run helper");
    assert!(
        output.status.success(),
        "helper failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let got = String::from_utf8(output.stdout).expect("utf8 stdout");

    let golden = base.join("expected.golden");
    if std::env::var("HOST_REFERENCE_BLESS").is_ok() {
        fs::write(&golden, &got).expect("write golden");
        return;
    }
    let want = fs::read_to_string(&golden)
        .expect("read golden; bless it first with HOST_REFERENCE_BLESS=1");
    assert_eq!(got, want, "recognised text drifted from the golden");
}
