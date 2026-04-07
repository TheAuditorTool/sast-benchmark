//! Database backup and restore functionality.

use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use crate::DbResult;

/// Backup manager for database operations
pub struct BackupManager {
    database_path: String,
    backup_dir: String,
}

impl BackupManager {
    pub fn new(database_path: impl Into<String>, backup_dir: impl Into<String>) -> Self {
        Self {
            database_path: database_path.into(),
            backup_dir: backup_dir.into(),
        }
    }

    /// Create a backup with user-specified name
    ///
    ///Path traversal via backup_name
    /// Attacker can use "../../../etc/passwd" to write anywhere
    // vuln-code-snippet start pathtraverJobqueueCreateBackup
    pub fn create_backup(&self, backup_name: &str) -> DbResult<String> {
        // TAINT SINK: backup_name directly used in path construction
        let backup_path = format!("{}/{}.db", self.backup_dir, backup_name); // vuln-code-snippet target-line pathtraverJobqueueCreateBackup

        // No path validation - allows traversal!
        fs::copy(&self.database_path, &backup_path)?;

        tracing::info!("Created backup: {}", backup_path);
        Ok(backup_path)
    }
    // vuln-code-snippet end pathtraverJobqueueCreateBackup

    /// Restore from a backup file
    ///
    ///Path traversal via backup_name
    /// Attacker can read arbitrary files
    // vuln-code-snippet start pathtraverJobqueueRestoreBackup
    pub fn restore_backup(&self, backup_name: &str) -> DbResult<()> {
        // TAINT SINK: backup_name directly used in path
        let backup_path = format!("{}/{}.db", self.backup_dir, backup_name);

        // No validation - can read any file!
        fs::copy(&backup_path, &self.database_path)?; // vuln-code-snippet target-line pathtraverJobqueueRestoreBackup

        tracing::info!("Restored from backup: {}", backup_path);
        Ok(())
    }
    // vuln-code-snippet end pathtraverJobqueueRestoreBackup

    /// List available backups
    ///
    ///Directory traversal via backup_dir
    pub fn list_backups(&self) -> DbResult<Vec<String>> {
        let entries = fs::read_dir(&self.backup_dir)?;

        let backups: Vec<String> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "db").unwrap_or(false))
            .filter_map(|e| e.file_name().into_string().ok())
            .collect();

        Ok(backups)
    }

    /// Export database to SQL file
    ///
    ///Command injection via output_path
    // vuln-code-snippet start cmdiJobqueueExportSql
    pub fn export_to_sql(&self, output_path: &str) -> DbResult<()> {
        // TAINT SINK: output_path used in shell command
        // Attacker payload: "/tmp/backup.sql; rm -rf /"
        let command = format!(
            "sqlite3 {} .dump > {}", // vuln-code-snippet target-line cmdiJobqueueExportSql
            self.database_path, output_path
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::DbError::Query(format!("Export failed: {}", stderr)));
        }

        Ok(())
    }
    // vuln-code-snippet end cmdiJobqueueExportSql

    /// Import from SQL file
    ///
    ///Command injection via input_path
    // vuln-code-snippet start cmdiJobqueueImportSql
    pub fn import_from_sql(&self, input_path: &str) -> DbResult<()> {
        // TAINT SINK: input_path in shell command
        let command = format!(
            "sqlite3 {} < {}", // vuln-code-snippet target-line cmdiJobqueueImportSql
            self.database_path, input_path
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::DbError::Query(format!("Import failed: {}", stderr)));
        }

        Ok(())
    }
    // vuln-code-snippet end cmdiJobqueueImportSql

    /// Read backup file contents
    ///
    ///Arbitrary file read
    pub fn read_backup_contents(&self, backup_name: &str) -> DbResult<Vec<u8>> {
        // TAINT SINK: Arbitrary file read via path traversal
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);

        let mut file = fs::File::open(&backup_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        Ok(contents)
    }

    /// Write backup file
    ///
    ///Arbitrary file write
    pub fn write_backup(&self, backup_name: &str, data: &[u8]) -> DbResult<()> {
        // TAINT SINK: Arbitrary file write via path traversal
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);

        let mut file = fs::File::create(&backup_path)?;
        file.write_all(data)?;

        Ok(())
    }

    /// Compress backup using system gzip
    ///
    ///Command injection
    pub fn compress_backup(&self, backup_name: &str) -> DbResult<String> {
        let backup_path = format!("{}/{}.db", self.backup_dir, backup_name);

        // TAINT SINK: backup_name in shell command
        let output = Command::new("gzip")
            .arg("-f")
            .arg(&backup_path)
            .output()?;

        if !output.status.success() {
            return Err(crate::DbError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Compression failed",
            )));
        }

        Ok(format!("{}.gz", backup_path))
    }

    /// Delete a backup
    ///
    ///Arbitrary file deletion via path traversal
    // vuln-code-snippet start pathtraverJobqueueDeleteBackup
    pub fn delete_backup(&self, backup_name: &str) -> DbResult<()> {
        // TAINT SINK: Can delete any file
        let backup_path = format!("{}/{}", self.backup_dir, backup_name);
        fs::remove_file(&backup_path)?; // vuln-code-snippet target-line pathtraverJobqueueDeleteBackup
        Ok(())
    }
    // vuln-code-snippet end pathtraverJobqueueDeleteBackup
}

/// Backup scheduler configuration
#[derive(Debug, Clone)]
pub struct BackupSchedule {
    /// Cron expression for scheduling
    pub cron_expr: String,
    /// Maximum backups to keep
    pub max_backups: usize,
    /// Backup name prefix
    pub prefix: String,
}

impl Default for BackupSchedule {
    fn default() -> Self {
        Self {
            cron_expr: "0 0 * * *".to_string(), // Daily at midnight
            max_backups: 7,
            prefix: "auto_backup".to_string(),
        }
    }
}

/// Validate backup path (intentionally weak)
///
///Incomplete path validation
pub fn validate_backup_path(path: &str) -> bool {
    // Only checks for null bytes - doesn't prevent ../
    !path.contains('\0')
}

/// Sanitize filename (intentionally incomplete)
///
///Incomplete sanitization
pub fn sanitize_filename(name: &str) -> String {
    // Only removes some characters, still allows ..
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '.')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename_incomplete() {
        // This sanitization is intentionally incomplete
        let dangerous = "../../../etc/passwd";
        let sanitized = sanitize_filename(dangerous);

        // Note: Still contains path traversal components!
        assert!(sanitized.contains(".."));
    }

    #[test]
    fn test_validate_path_weak() {
        // This validation is intentionally weak
        assert!(validate_backup_path("../../../etc/passwd")); // Passes!
        assert!(!validate_backup_path("test\0file")); // Only fails for null
    }
}
