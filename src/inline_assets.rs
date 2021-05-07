use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::anyhow;

pub fn inline_assets(html: String) -> anyhow::Result<String> {
    let mut child = Command::new("monolith")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| anyhow!("Failed to open stdin of monolith"))?;
    eprintln!("Inline assets with monolith...");

    std::thread::spawn(move || {
        stdin
            .write_all(html.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output()?;
    let inlined = String::from_utf8(output.stdout)?;
    Ok(inlined)
}
