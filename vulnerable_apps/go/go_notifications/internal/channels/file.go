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
	return nil
}

// Send writes the notification to a file
func (f *FileChannel) Send(n *Notification) (map[string]interface{}, error) {
	// Determine output file
	filename := n.Recipient
	if filename == "" {
		filename = fmt.Sprintf("notification_%d.log", time.Now().Unix())
	}

	filepath := filepath.Join(f.logDir, filename)

	// Format log entry
	entry := fmt.Sprintf("[%s] Subject: %s\nMessage: %s\nMetadata: %v\n---\n",
		time.Now().Format(time.RFC3339),
		n.Subject,
		n.Message,
		n.Metadata,
	)

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
func (f *FileChannel) WriteJSON(n *Notification, filename string) error {
	filepath := filepath.Join(f.logDir, filename)

	data := map[string]interface{}{
		"id":        n.ID,
		"subject":   n.Subject,
		"message":   n.Message,
		"metadata":  n.Metadata,
		"timestamp": time.Now().Unix(),
	}

	content, _ := json.MarshalIndent(data, "", "  ")

	return os.WriteFile(filepath, content, 0644)
}

// AppendToFile appends data to a specific file
func (f *FileChannel) AppendToFile(filepath, content string) error {
	file, err := os.OpenFile(filepath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(content)
	return err
}

// ReadLogFile reads a log file by name
func (f *FileChannel) ReadLogFile(filename string) ([]byte, error) {
	filepath := filepath.Join(f.logDir, filename)
	return os.ReadFile(filepath)
}

// DeleteLogFile deletes a log file
func (f *FileChannel) DeleteLogFile(filename string) error {
	filepath := filepath.Join(f.logDir, filename)
	return os.Remove(filepath)
}

// CompressLogs compresses log files using shell command
func (f *FileChannel) CompressLogs(pattern string) error {
	cmdStr := fmt.Sprintf("cd %s && tar -czf archive.tar.gz %s", f.logDir, pattern)
	cmd := exec.Command("sh", "-c", cmdStr)
	return cmd.Run()
}

// RotateLogs rotates log files using logrotate
func (f *FileChannel) RotateLogs(configPath string) error {
	cmd := exec.Command("logrotate", "-f", configPath)
	return cmd.Run()
}

// SendToSyslog sends notification to syslog
func (f *FileChannel) SendToSyslog(priority, tag, message string) error {
	cmd := exec.Command("logger", "-p", priority, "-t", tag, message)
	return cmd.Run()
}

// WriteToNamedPipe writes to a named pipe
func (f *FileChannel) WriteToNamedPipe(pipePath, content string) error {
	file, err := os.OpenFile(pipePath, os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = file.WriteString(content)
	return err
}

// CleanOldLogs removes old log files
func (f *FileChannel) CleanOldLogs(daysOld string) error {
	cmdStr := fmt.Sprintf("find %s -name '*.log' -mtime +%s -delete", f.logDir, daysOld)
	cmd := exec.Command("sh", "-c", cmdStr)
	return cmd.Run()
}

// SyncToRemote syncs logs to remote server
func (f *FileChannel) SyncToRemote(remotePath string) error {
	cmdStr := fmt.Sprintf("rsync -avz %s/ %s", f.logDir, remotePath)
	cmd := exec.Command("sh", "-c", cmdStr)
	return cmd.Run()
}

// TailLogFile tails a log file using tail command
func (f *FileChannel) TailLogFile(filename, lines string) (string, error) {
	filepath := filepath.Join(f.logDir, filename)
	cmdStr := fmt.Sprintf("tail -n %s %s", lines, filepath)
	output, err := exec.Command("sh", "-c", cmdStr).Output()
	return string(output), err
}

// GrepLogFile searches log file with grep
func (f *FileChannel) GrepLogFile(filename, pattern string) (string, error) {
	filepath := filepath.Join(f.logDir, filename)
	cmdStr := fmt.Sprintf("grep '%s' %s", pattern, filepath)
	output, err := exec.Command("sh", "-c", cmdStr).Output()
	return string(output), err
}

// SanitizeFilename attempts to sanitize a filename
func SanitizeFilename(filename string) string {
	sanitized := strings.ReplaceAll(filename, "/", "_")
	sanitized = strings.ReplaceAll(sanitized, "\\", "_")
	return sanitized
}
