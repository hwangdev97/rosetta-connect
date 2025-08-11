use serde_json::Value;
use anyhow::{Context, Result};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::process::Command;
use std::process::Stdio;

// Node.js bridge implementation using subprocess communication
pub async fn asc_upload(meta: Value) -> Result<Value> {
    call_node_function("asc_upload", meta).await
}

pub async fn asc_download(app_id: String) -> Result<Value> {
    call_node_function("asc_download", serde_json::json!(app_id)).await
}

pub async fn asc_validate(content: Value) -> Result<Value> {
    call_node_function("asc_validate", content).await
}

pub async fn ai_translate(request: Value) -> Result<Value> {
    call_node_function("ai_translate", request).await
}

pub async fn ai_estimate_cost(request: Value) -> Result<Value> {
    call_node_function("ai_estimate_cost", request).await
}

pub async fn asc_get_version_status(app_id: String) -> Result<Value> {
    call_node_function("asc_get_version_status", serde_json::json!(app_id)).await
}

async fn call_node_function(function_name: &str, args: Value) -> Result<Value> {
    let js_dir = std::env::current_dir()?.join("js");
    
    // Check if TypeScript is built
    let js_file = js_dir.join("dist").join("asc.js");
    if !js_file.exists() {
        // Try to build TypeScript first
        println!("Building TypeScript...");
        let build_output = Command::new("npm")
            .args(["run", "build"])
            .current_dir(&js_dir)
            .output()
            .await
            .context("Failed to run npm build")?;
            
        if !build_output.status.success() {
            return Err(anyhow::anyhow!("TypeScript build failed: {}", 
                String::from_utf8_lossy(&build_output.stderr)));
        }
    }

    // Determine which module to require based on function name
    let require_module = if function_name.starts_with("ai_") {
        "./dist/openai-service.js"
    } else {
        "./dist/asc.js"
    };

    // Create the Node.js script to call our function
    let script = format!(r#"
const {{ {} }} = require('{}');

async function main() {{
    try {{
        const result = await {}({});
        console.log(JSON.stringify({{ success: true, data: result }}));
    }} catch (error) {{
        console.log(JSON.stringify({{ success: false, error: error.message }}));
    }}
}}

main();
"#, function_name, require_module, function_name, args);

    // Execute the Node.js script
    let mut child = Command::new("node")
        .args(["-e", &script])
        .current_dir(&js_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn Node.js process")?;

    // Stream stderr lines in real-time for progress/debug logs
    let mut stderr_buf = String::new();
    let mut stdout_buf: Vec<u8> = Vec::new();

    let mut stderr_reader = child.stderr.take().map(BufReader::new);
    let mut stdout_reader = child.stdout.take();

    let stderr_task = tokio::spawn(async move {
        if let Some(ref mut s) = stderr_reader {
            let mut line = String::new();
            loop {
                line.clear();
                match s.read_line(&mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if !line.trim().is_empty() {
                            eprintln!("{}", line.trim_end());
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    });

    let stdout_task = tokio::spawn(async move {
        if let Some(mut o) = stdout_reader {
            let mut buf = Vec::new();
            let _ = o.read_to_end(&mut buf).await; // ignore error, handled later
            buf
        } else {
            Vec::new()
        }
    });

    let status = child
        .wait()
        .await
        .context("Failed to wait for Node.js process")?;

    // Ensure readers finished
    let _ = stderr_task.await;
    stdout_buf = stdout_task
        .await
        .unwrap_or_default();

    if !status.success() {
        // We already streamed stderr; provide a concise error
        return Err(anyhow::anyhow!(
            "Node.js process failed with exit code {}",
            status.code().unwrap_or(-1)
        ));
    }

    let stdout = String::from_utf8_lossy(&stdout_buf);
    if std::env::var("ROSETTA_DEBUG_JS").is_ok() {
        eprintln!("Node.js response: {}", stdout.trim());
    }

    let response: serde_json::Value = serde_json::from_str(&stdout)
        .context("Failed to parse Node.js response")?;

    if response.get("success").and_then(|v| v.as_bool()).unwrap_or(false) {
        Ok(response.get("data").cloned().unwrap_or(serde_json::Value::Null))
    } else {
        let error_msg = response.get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error");
        Err(anyhow::anyhow!("Node.js function error: {}", error_msg))
    }
}

// Initialize the Node.js runtime
pub fn init_node_runtime() -> Result<()> {
    println!("Initializing Node.js runtime...");
    
    // Check if Node.js is available
    if let Err(_) = which::which("node") {
        return Err(anyhow::anyhow!("Node.js not found. Please install Node.js to use this feature."));
    }
    
    // Check if npm dependencies are installed
    let js_dir = std::env::current_dir()?.join("js");
    let node_modules = js_dir.join("node_modules");
    if !node_modules.exists() {
        return Err(anyhow::anyhow!("Node.js dependencies not installed. Please run 'npm install' in the js directory."));
    }
    
    Ok(())
}