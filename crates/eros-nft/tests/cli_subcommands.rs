use std::process::Command;

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_eros-nft"))
}

#[test]
fn schema_export_draft() {
    let out = bin().args(["schema", "export", "draft"]).output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("PersonaDraft"));
}

#[test]
fn sample_list_shows_15() {
    let out = bin().args(["sample", "list"]).output().unwrap();
    assert!(out.status.success());
    let s = String::from_utf8_lossy(&out.stdout);
    let lines: Vec<&str> = s.lines().collect();
    assert_eq!(lines.len(), 15);
    assert!(lines.contains(&"yuki-warm-senpai"));
}

#[test]
fn validate_a_bundled_sample_via_path() {
    use std::io::Write;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("manifest.json");
    let (_, m) = eros_nft::load_sample("yuki-warm-senpai").unwrap();
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(serde_json::to_string_pretty(&m).unwrap().as_bytes()).unwrap();
    let out = bin().args(["validate", path.to_str().unwrap()]).output().unwrap();
    assert!(out.status.success(), "stderr: {}", String::from_utf8_lossy(&out.stderr));
}
