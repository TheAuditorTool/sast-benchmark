// Package channels - File notification channel (logging)
package channels

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

// FileChannel handles file-based notifications (logging)
type FileChannel struct {
	logDir string
}

// NewFileChannel creates a new file channel
func NewFileChannel(logDir string) *FileChannel {
	// Ensure log directory exists
	os.MkdirAll(logDir, 0755)

	return &FileChannel{
		logDir: logDir,
	}
}

// Name returns the channel name
func (f *FileChannel) Name() string {
	return "file"
}

// Validate checks if the notification is valid for file channel
func (f *FileChannel) Validate(n *Notification) error {
	// VULN: No validation of recipient (filename)
	return nil
}

// Send writes the notification to a file
// TAINT SINK: Recipient is user-controlled filename - path traversal
func (f *FileChannel) Send(n *Notification) (map[string]interface{}, error) {
	// Determine output file
	filename := n.Recipient
	if filename == "" {
		filename = fmt.Sprintf("notification_%d.log", time.Now().Unix())
	}

	// VULN: Path traversal - filename not sanitized
	// n.Recipient could be "../../../etc/cron.d/malicious"
	filepath := filepath.Join(f.logDir, filename) // TAINT SINK

	// Format log entry
	entry := fmt.Sprintf("[%s] Subject: %s\nMessage: %s\nMetadata: %v\n---\n",
		time.Now().Format(time.RFC3339),
		n.Subject, // TAINT: User-controlled
		n.Message, // TAINT: User-controlled, log injection possible
		n.Metadata,
	)

	// VULN: Arbitrary file write
	file, err := os.OpenFile(filepath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return nil, fmt.Errorf("failed to open log file: %w", err)
	}
	defer file.Close()

	if _, err := file.WriteString(entry); err != nil {
		return nil, fmt.Errorf("failed to write to log file: %w", err)
	}

	return map[string]interface{}{
		"file":    filepath,
		"written": len(entry),
	}, nil
}

// WriteJSON writes notification as JSON to file
// VULN: Same path traversal issue
func (f *FileChannel) WriteJSON(n *Notification, filename string) error {
	// VULN: Path traversal
	filepath := filepath.Join(f.logDir, filename)

	data := map[string]interface{}{
		"id":        n.ID,
		"subject":   n.Subject,
		"message":   n.Message,
		"metadata":  n.Metadata,
		"timestamp": time.Now().Unix(),
	}

	content, _ := json.MarshalIndent(data, "", "  ")

	return os.WriteFile(filepath, content, 0644) // TAINT SINK
}

// AppendToFile appends data to a specific file
// VULN: Arbitrary file append
func (f *FileChannel) AppendToFile(filepath, content string) error {
	// VULN: No path validation - arbitrary file append
	file, err := os.OpenFile(filepath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(content) // TAINT SINK
	return err
}

// ReadLogFile reads a log file by name
// VULN: Path traversal for reading
func (f *FileChannel) ReadLogFile(filename string) ([]byte, error) {
	// VULN: Path traversal - can read arbitrary files
	filepath := filepath.Join(f.logDir, filename)
	return os.ReadFile(filepath) // TAINT SINK
}

// DeleteLogFile deletes a log file
// VULN: Arbitrary file deletion via path traversal
func (f *FileChannel) DeleteLogFile(filename string) error {
	filepath := filepath.Join(f.logDir, filename)
	return os.Remove(filepath) // TAINT SINK: Arbitrary file deletion
}

// CompressLogs compresses log files using shell command
// VULN: Command injection via filename
func (f *FileChannel) CompressLogs(pattern string) error {
	// VULN: Pattern injected into shell command
	cmdStr := fmt.Sprintf("cd %s && tar -czf archive.tar.gz %s", f.logDir, pattern)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK: Command injection
	return cmd.Run()
}

// RotateLogs rotates log files using logrotate
// VULN: Command injection
func (f *FileChannel) RotateLogs(configPath string) error {
	// VULN: Config path in shell command
	cmd := exec.Command("logrotate", "-f", configPath) // TAINT SINK
	return cmd.Run()
}

// SendToSyslog sends notification to syslog
// VULN: Log injection via message
func (f *FileChannel) SendToSyslog(priority, tag, message string) error {
	// VULN: Message injected into shell command
	// Newlines in message can inject fake log entries
	cmd := exec.Command("logger", "-p", priority, "-t", tag, message) // TAINT SINK
	return cmd.Run()
}

// WriteToNamedPipe writes to a named pipe
// VULN: Arbitrary file access via named pipe path
func (f *FileChannel) WriteToNamedPipe(pipePath, content string) error {
	// VULN: No validation of pipe path
	file, err := os.OpenFile(pipePath, os.O_WRONLY, 0644) // TAINT SINK
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(content)
	return err
}

// CleanOldLogs removes old log files
// VULN: Command injection via days parameter
func (f *FileChannel) CleanOldLogs(daysOld string) error {
	// VULN: User-controlled value in find command
	cmdStr := fmt.Sprintf("find %s -name '*.log' -mtime +%s -delete", f.logDir, daysOld)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK
	return cmd.Run()
}

// SyncToRemote syncs logs to remote server
// VULN: Command injection via remote path
func (f *FileChannel) SyncToRemote(remotePath string) error {
	// VULN: Remote path in rsync command
	cmdStr := fmt.Sprintf("rsync -avz %s/ %s", f.logDir, remotePath)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK
	return cmd.Run()
}

// TailLogFile tails a log file using tail command
// VULN: Command injection via filename and lines
func (f *FileChannel) TailLogFile(filename, lines string) (string, error) {
	filepath := filepath.Join(f.logDir, filename)
	// VULN: Both filename and lines in command
	cmdStr := fmt.Sprintf("tail -n %s %s", lines, filepath)
	output, err := exec.Command("sh", "-c", cmdStr).Output() // TAINT SINK
	return string(output), err
}

// GrepLogFile searches log file with grep
// VULN: Command injection via pattern and filename
func (f *FileChannel) GrepLogFile(filename, pattern string) (string, error) {
	filepath := filepath.Join(f.logDir, filename)
	// VULN: Pattern and filepath in grep command
	cmdStr := fmt.Sprintf("grep '%s' %s", pattern, filepath)
	output, err := exec.Command("sh", "-c", cmdStr).Output() // TAINT SINK
	return string(output), err
}

// SanitizeFilename attempts to sanitize a filename
// VULN: Incomplete sanitization
func SanitizeFilename(filename string) string {
	// VULN: Only removes some dangerous characters
	// Doesn't handle: ../, encoded characters, null bytes
	sanitized := strings.ReplaceAll(filename, "/", "_")
	sanitized = strings.ReplaceAll(sanitized, "\\", "_")
	// Doesn't handle: .., ~, $, `, etc.
	return sanitized
}
