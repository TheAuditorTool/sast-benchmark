//! File operations demonstrating path traversal sinks.
//!
//! All file operations with user-controlled paths are potential vulnerabilities.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

/// TAINT SINK: fs::read_to_string with user-controlled path
/// Path traversal vulnerability - user can read arbitrary files
// vuln-code-snippet start pathtraverReadFile
pub fn read_file(path: &str) -> io::Result<String> {
    // TAINT SINK: User-controlled file path
    std::fs::read_to_string(path) // vuln-code-snippet target-line pathtraverReadFile
}
// vuln-code-snippet end pathtraverReadFile

/// TAINT SINK: fs::write with user-controlled path
/// Path traversal vulnerability - user can write to arbitrary locations
// vuln-code-snippet start pathtraverWriteFile
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    // TAINT SINK: User-controlled file path
    std::fs::write(path, content) // vuln-code-snippet target-line pathtraverWriteFile
}
// vuln-code-snippet end pathtraverWriteFile

/// TAINT SINK: File::create with user-controlled path
pub fn create_file(path: &str) -> io::Result<File> {
    // TAINT SINK: User-controlled path to File::create
    File::create(path)
}

/// TAINT SINK: File::open with user-controlled path
pub fn open_file(path: &str) -> io::Result<File> {
    // TAINT SINK: User-controlled path
    File::open(path)
}

/// TAINT SINK: OpenOptions with user-controlled path
pub fn open_file_with_options(path: &str, append: bool) -> io::Result<File> {
    // TAINT SINK: OpenOptions::open with user path
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .open(path)
}

/// TAINT SINK: fs::remove_file with user-controlled path
/// Very dangerous - can delete arbitrary files!
// vuln-code-snippet start pathtraverDeleteFile
pub fn delete_file(path: &str) -> io::Result<()> {
    // TAINT SINK: User-controlled path to remove
    fs::remove_file(path) // vuln-code-snippet target-line pathtraverDeleteFile
}
// vuln-code-snippet end pathtraverDeleteFile

/// TAINT SINK: fs::create_dir_all with user-controlled path
pub fn create_directory(path: &str) -> io::Result<()> {
    // TAINT SINK: User-controlled directory creation
    fs::create_dir_all(path)
}

/// TAINT SINK: fs::copy with user-controlled paths
pub fn copy_file(source: &str, dest: &str) -> io::Result<u64> {
    // TAINT SINK: Both source and destination are user-controlled
    fs::copy(source, dest)
}

/// TAINT SINK: fs::rename with user-controlled paths
pub fn move_file(source: &str, dest: &str) -> io::Result<()> {
    // TAINT SINK: Both source and destination are user-controlled
    fs::rename(source, dest)
}

/// Read file lines into vector
/// TAINT SOURCE + SINK combination
pub fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    // TAINT SINK: Path from user
    let content = fs::read_to_string(path)?;
    // TAINT SOURCE: File content (could be malicious)
    Ok(content.lines().map(String::from).collect())
}

/// Append to file with user content
pub fn append_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;  // TAINT SINK: path

    // Writing user content to file
    file.write_all(content.as_bytes())
}

/// Read binary file
pub fn read_binary_file(path: &str) -> io::Result<Vec<u8>> {
    // TAINT SINK + SOURCE: Read binary from user-controlled path
    fs::read(path)
}

/// Write binary file
pub fn write_binary_file(path: &str, data: &[u8]) -> io::Result<()> {
    // TAINT SINK: Write binary to user-controlled path
    fs::write(path, data)
}

/// List directory contents
pub fn list_directory(path: &str) -> io::Result<Vec<String>> {
    // TAINT SINK: User-controlled directory listing
    let entries = fs::read_dir(path)?;

    let mut files = Vec::new();
    for entry in entries {
        let entry = entry?;
        files.push(entry.path().display().to_string());
    }
    Ok(files)
}

/// Get file metadata
pub fn get_file_info(path: &str) -> io::Result<fs::Metadata> {
    // TAINT SINK: User-controlled path for metadata
    fs::metadata(path)
}

/// Normalize path (attempt at sanitization, but still vulnerable)
pub fn normalize_path(base: &str, user_path: &str) -> String {
    // WARNING: This is NOT sufficient sanitization!
    // Path traversal can still occur via symlinks, etc.
    let base_path = Path::new(base);
    let full_path = base_path.join(user_path);

    // Attempt to canonicalize - still not safe!
    match full_path.canonicalize() {
        Ok(p) => p.display().to_string(),
        Err(_) => full_path.display().to_string(),
    }
}

/// Read env file (source of tainted data)
pub fn read_env_var(name: &str) -> Option<String> {
    // This reads environment variables which could be tainted
    std::env::var(name).ok()
}

/// Process uploaded file (common attack vector)
// vuln-code-snippet start pathtraverProcessUpload
pub fn process_upload(filename: &str, content: &[u8], upload_dir: &str) -> io::Result<String> {
    //filename could contain path traversal sequences
    // e.g., "../../../etc/passwd" or "..\\..\\windows\\system32\\config"

    let path = format!("{}/{}", upload_dir, filename);

    // TAINT SINK: Writing to user-controlled path
    fs::write(&path, content)?; // vuln-code-snippet target-line pathtraverProcessUpload

    Ok(path)
}
// vuln-code-snippet end pathtraverProcessUpload

/// Upload with path validation
// vuln-code-snippet start pathtraverProcessUpload2
pub fn process_upload_safe(
    filename: &str,
    content: &[u8],
    upload_dir: &str,
) -> io::Result<String> {
    // Sanitize filename - remove path components
    let safe_filename = Path::new(filename)
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;

    // Ensure no path traversal in the sanitized name
    if safe_filename.contains("..") || safe_filename.contains('/') || safe_filename.contains('\\') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid characters in filename",
        ));
    }

    let path = Path::new(upload_dir).join(safe_filename);

    // Verify the final path is within upload_dir
    let canonical_upload = Path::new(upload_dir).canonicalize()?;
    let canonical_path = path.canonicalize().unwrap_or_else(|_| path.clone());

    if !canonical_path.starts_with(&canonical_upload) { // vuln-code-snippet target-line pathtraverProcessUpload2
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Path traversal detected",
        ));
    }

    fs::write(&path, content)?;
    Ok(path.display().to_string())
}
// vuln-code-snippet end pathtraverProcessUpload2
