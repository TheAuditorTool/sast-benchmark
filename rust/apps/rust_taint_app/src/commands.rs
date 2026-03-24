//! Command execution module with Command::new and shell invocation patterns.

use std::process::{Command, Output, Stdio};
use std::io;

/// TAINT SINK: Command::new with user input
// vuln-code-snippet start cmdiExecuteCommand
pub fn execute_command(cmd: &str, args: &[String]) -> io::Result<String> {
    // TAINT SINK: Command::new with user-controlled command name
    let output: Output = Command::new(cmd) // vuln-code-snippet target-line cmdiExecuteCommand
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if output.status.success() {
        Ok(stdout.to_string())
    } else {
        Ok(format!("Error: {}", stderr))
    }
}
// vuln-code-snippet end cmdiExecuteCommand

// vuln-code-snippet start cmdiExecuteCommand2
///Command name from allowlist only
pub fn execute_command_allowlisted(cmd_type: &str, args: &[String]) -> io::Result<String> {
    let command = match cmd_type {
        "ls" => "ls",
        "cat" => "cat",
        "grep" => "grep",
        "find" => "find",
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown command")),
    };
    let output: Output = Command::new(command) // vuln-code-snippet target-line cmdiExecuteCommand2
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(format!("Error: {}", String::from_utf8_lossy(&output.stderr)))
    }
}
// vuln-code-snippet end cmdiExecuteCommand2

/// TAINT SINK: Shell command execution
// vuln-code-snippet start cmdiExecuteShellCommand
pub fn execute_shell_command(shell_cmd: &str) -> io::Result<String> {
    // TAINT SINK: Shell command with user input (CRITICAL!)
    #[cfg(target_os = "windows")]
    let output = Command::new("cmd")
        .args(["/C", shell_cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = Command::new("sh") // vuln-code-snippet target-line cmdiExecuteShellCommand
        .args(["-c", shell_cmd])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}
// vuln-code-snippet end cmdiExecuteShellCommand

// vuln-code-snippet start cmdiExecuteShellCommand2
///Only allowlisted operations, no raw shell string
pub fn execute_shell_operation(operation: &str, target: &str) -> io::Result<String> {
    let safe_cmd = match operation {
        "echo" => format!("echo {}", shell_escape(target)),
        "wc" => format!("wc -l {}", shell_escape(target)),
        "date" => "date".to_string(),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown operation")),
    };
    let output = Command::new("sh")
        .args(["-c", &safe_cmd]) // vuln-code-snippet target-line cmdiExecuteShellCommand2
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
fn shell_escape(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}
// vuln-code-snippet end cmdiExecuteShellCommand2

/// TAINT SINK: Execute with working directory from user input
// vuln-code-snippet start cmdiExecuteInDirectory
pub fn execute_in_directory(cmd: &str, working_dir: &str) -> io::Result<String> {
    // TAINT SINK: Both command and working directory are user-controlled
    let output = Command::new(cmd)
        .current_dir(working_dir)  // Path traversal risk! // vuln-code-snippet target-line cmdiExecuteInDirectory
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiExecuteInDirectory

// vuln-code-snippet start cmdiExecuteInDirectory2
///Working directory validated against base directory
pub fn execute_in_directory_validated(cmd: &str, working_dir: &str, base_dir: &str) -> io::Result<String> {
    use std::path::Path;
    let canonical_base = Path::new(base_dir).canonicalize()?;
    let canonical_dir = Path::new(base_dir).join(working_dir).canonicalize()?;
    if !canonical_dir.starts_with(&canonical_base) { // vuln-code-snippet target-line cmdiExecuteInDirectory2
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path traversal blocked"));
    }
    let output = Command::new(cmd)
        .current_dir(&canonical_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiExecuteInDirectory2

/// TAINT SINK: Execute with environment variables from user input
pub fn execute_with_env(
    cmd: &str,
    env_vars: &std::collections::HashMap<String, String>,
) -> io::Result<String> {
    // TAINT SINK: Command with user-controlled environment
    let mut command = Command::new(cmd);

    for (key, value) in env_vars {
        command.env(key, value);
    }

    let output = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// TAINT SINK: Command with piped stdin from user
pub fn execute_with_stdin(cmd: &str, stdin_data: &str) -> io::Result<String> {
    use std::io::Write;

    // TAINT SINK: Command execution with piped input
    let mut child = Command::new(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // Write tainted data to stdin
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(stdin_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Build a command line string (also dangerous)
pub fn build_command_line(program: &str, args: &[&str]) -> String {
    // Building command strings is dangerous even before execution
    let mut cmd_line = program.to_string();
    for arg in args {
        cmd_line.push(' ');
        cmd_line.push_str(arg);
    }
    cmd_line
}

/// TAINT SINK: std::process::Command::new direct usage
pub fn run_system_command(command_name: &str) -> io::Result<Output> {
    // TAINT SINK: Direct command execution
    std::process::Command::new(command_name)
        .output()
}

///Command with argument injection
// vuln-code-snippet start cmdiRunCommandWithArg
pub fn run_command_with_arg(base_cmd: &str, user_arg: &str) -> io::Result<String> {
    //User input directly passed as argument
    // Could include shell metacharacters like ; | && etc.
    let output = Command::new(base_cmd)
        .arg(user_arg)  // TAINT SINK: User-controlled argument // vuln-code-snippet target-line cmdiRunCommandWithArg
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiRunCommandWithArg

// vuln-code-snippet start cmdiRunCommandWithArg2
///Argument validated for shell metacharacters
pub fn run_command_with_arg_checked(base_cmd: &str, user_arg: &str) -> io::Result<String> {
    if user_arg.contains(|c: char| matches!(c, ';' | '|' | '&' | '>' | '<' | '$' | '`' | '\\')) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid characters in argument"));
    }
    if user_arg.len() > 1024 { // vuln-code-snippet target-line cmdiRunCommandWithArg2
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Argument too long"));
    }
    let output = Command::new(base_cmd)
        .arg(user_arg)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiRunCommandWithArg2

/// Example of how commands could be used in a "safer" way
/// Still requires careful validation of inputs
// vuln-code-snippet start cmdiExecuteAllowedCommand
pub fn execute_allowed_command(cmd_type: &str, target: &str) -> io::Result<String> {
    // Allowlist approach - but still needs validation of 'target'
    let command = match cmd_type {
        "ping" => "ping",
        "nslookup" => "nslookup",
        "traceroute" => "traceroute",
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown command")),
    };

    // STILL VULNERABLE: target could contain shell metacharacters
    let output = Command::new(command)
        .arg(target) // vuln-code-snippet target-line cmdiExecuteAllowedCommand
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiExecuteAllowedCommand

// vuln-code-snippet start cmdiExecuteAllowedCommand2
///Both command and target validated
pub fn execute_allowed_command_checked(cmd_type: &str, target: &str) -> io::Result<String> {
    let command = match cmd_type {
        "ping" => "ping",
        "nslookup" => "nslookup",
        "traceroute" => "traceroute",
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown command")),
    };
    if !target.chars().all(|c| c.is_alphanumeric() || matches!(c, '.' | '-')) || target.is_empty() || target.len() > 253 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid target format")); // vuln-code-snippet target-line cmdiExecuteAllowedCommand2
    }
    let output = Command::new(command)
        .arg(target)
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
// vuln-code-snippet end cmdiExecuteAllowedCommand2
