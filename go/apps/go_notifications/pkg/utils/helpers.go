// Package utils provides utility functions
package utils

import (
	"crypto/md5"
	"crypto/rand"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"encoding/xml"
	"fmt"
	"html"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"
)

// ToCSV converts data to CSV format
func ToCSV(data []map[string]interface{}) []byte {
	if len(data) == 0 {
		return []byte{}
	}

	var sb strings.Builder

	// Get headers from first row
	headers := make([]string, 0)
	for key := range data[0] {
		headers = append(headers, key)
	}
	sb.WriteString(strings.Join(headers, ",") + "\n")

	// Write data rows
	for _, row := range data {
		values := make([]string, len(headers))
		for i, header := range headers {
			if val, ok := row[header]; ok {
				// VULN: No CSV escaping - injection possible
				values[i] = fmt.Sprintf("%v", val)
			}
		}
		sb.WriteString(strings.Join(values, ",") + "\n")
	}

	return []byte(sb.String())
}

// ToXML converts data to XML format
func ToXML(data []map[string]interface{}) []byte {
	type Item struct {
		XMLName xml.Name
		Content string `xml:",chardata"`
	}

	type Root struct {
		XMLName xml.Name `xml:"notifications"`
		Items   []interface{}
	}

	root := Root{}
	for _, item := range data {
		for key, value := range item {
			root.Items = append(root.Items, Item{
				XMLName: xml.Name{Local: key},
				Content: fmt.Sprintf("%v", value),
			})
		}
	}

	output, _ := xml.MarshalIndent(root, "", "  ")
	return output
}

// HashPassword creates a password hash
// VULN: Uses MD5 which is cryptographically weak
func HashPassword(password string) string {
	hash := md5.Sum([]byte(password)) // VULN: MD5 for password hashing
	return hex.EncodeToString(hash[:])
}

// GenerateToken generates a random token
// VULN: Weak token generation
func GenerateToken(length int) string {
	bytes := make([]byte, length)
	rand.Read(bytes)
	return base64.URLEncoding.EncodeToString(bytes)
}

// SanitizeInput attempts to sanitize user input
// VULN: Incomplete sanitization
func SanitizeInput(input string) string {
	// VULN: Only removes some dangerous characters
	sanitized := strings.ReplaceAll(input, "<", "")
	sanitized = strings.ReplaceAll(sanitized, ">", "")
	// VULN: Doesn't handle: ", ', &, javascript:, etc.
	return sanitized
}

// SanitizeHTML uses html.EscapeString
func SanitizeHTML(input string) string {
	return html.EscapeString(input)
}

// ValidateEmail performs basic email validation
// VULN: Weak validation regex
func ValidateEmail(email string) bool {
	// VULN: Very permissive regex
	pattern := `.+@.+\..+`
	matched, _ := regexp.MatchString(pattern, email)
	return matched
}

// ValidateURL performs URL validation
// VULN: Doesn't check for internal IPs or schemes
func ValidateURL(url string) bool {
	// VULN: Only checks if URL is non-empty
	return len(url) > 0 && (strings.HasPrefix(url, "http://") || strings.HasPrefix(url, "https://"))
}

// ReadFileContents reads a file
// VULN: Path traversal if path is user-controlled
func ReadFileContents(path string) ([]byte, error) {
	// VULN: No path validation
	return os.ReadFile(path) // TAINT SINK
}

// WriteFileContents writes to a file
// VULN: Arbitrary file write
func WriteFileContents(path string, content []byte) error {
	return os.WriteFile(path, content, 0644) // TAINT SINK
}

// ExecuteCommand runs a shell command
// VULN: Command injection
func ExecuteCommand(cmd string) (string, error) {
	output, err := exec.Command("sh", "-c", cmd).Output() // TAINT SINK
	return string(output), err
}

// ExecuteCommandWithArgs runs a command with arguments
// VULN: Command injection via args
func ExecuteCommandWithArgs(cmd string, args ...string) (string, error) {
	output, err := exec.Command(cmd, args...).Output() // TAINT SINK
	return string(output), err
}

// DownloadFile downloads a file from URL
// VULN: SSRF
func DownloadFile(url, savePath string) error {
	// VULN: No URL validation - SSRF possible
	resp, err := http.Get(url) // TAINT SINK
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	// VULN: Arbitrary file write via savePath
	file, err := os.Create(savePath) // TAINT SINK
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = io.Copy(file, resp.Body)
	return err
}

// ExtractArchive extracts an archive
// VULN: Path traversal via archive contents (Zip Slip)
func ExtractArchive(archivePath, destDir string) error {
	// VULN: Uses tar command with user-controlled paths
	cmd := exec.Command("tar", "-xzf", archivePath, "-C", destDir) // TAINT SINK
	return cmd.Run()
}

// MergeJSON merges two JSON objects
// VULN: Prototype pollution concept (though not applicable in Go)
func MergeJSON(base, overlay string) (string, error) {
	var baseMap, overlayMap map[string]interface{}

	if err := json.Unmarshal([]byte(base), &baseMap); err != nil {
		return "", err
	}
	if err := json.Unmarshal([]byte(overlay), &overlayMap); err != nil {
		return "", err
	}

	// Merge overlay into base
	for key, value := range overlayMap {
		baseMap[key] = value // Overwrites any key
	}

	result, err := json.Marshal(baseMap)
	return string(result), err
}

// FormatLogEntry formats a log entry
// VULN: Log injection via user data
func FormatLogEntry(level, message string, data map[string]interface{}) string {
	// VULN: No sanitization - newlines can inject fake log entries
	entry := fmt.Sprintf("[%s] %s", level, message)
	if len(data) > 0 {
		dataJSON, _ := json.Marshal(data)
		entry += " | data=" + string(dataJSON)
	}
	return entry
}

// SafeJoin safely joins path components
// VULN: Still vulnerable to traversal if not properly used
func SafeJoin(base string, paths ...string) string {
	// This looks safe but doesn't prevent traversal after joining
	result := base
	for _, p := range paths {
		result = filepath.Join(result, p)
	}
	return result
}

// IsInternalIP checks if an IP is internal
// Not actually used anywhere - dead code
func IsInternalIP(ip string) bool {
	internalPrefixes := []string{
		"10.",
		"172.16.", "172.17.", "172.18.", "172.19.",
		"172.20.", "172.21.", "172.22.", "172.23.",
		"172.24.", "172.25.", "172.26.", "172.27.",
		"172.28.", "172.29.", "172.30.", "172.31.",
		"192.168.",
		"127.",
		"169.254.",
	}

	for _, prefix := range internalPrefixes {
		if strings.HasPrefix(ip, prefix) {
			return true
		}
	}
	return false
}

// SendHTTPRequest sends an HTTP request
// VULN: SSRF helper
func SendHTTPRequest(method, url string, headers map[string]string, body string) ([]byte, error) {
	req, err := http.NewRequest(method, url, strings.NewReader(body))
	if err != nil {
		return nil, err
	}

	for key, value := range headers {
		req.Header.Set(key, value)
	}

	client := &http.Client{}
	resp, err := client.Do(req) // TAINT SINK: SSRF
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}

// RenderTemplate renders a simple template string
// VULN: No escaping of values
func RenderTemplate(template string, values map[string]string) string {
	result := template
	for key, value := range values {
		// VULN: No escaping - injection possible
		result = strings.ReplaceAll(result, "{{"+key+"}}", value)
	}
	return result
}

// ParseUserInput parses and validates user input
// VULN: Incomplete validation
func ParseUserInput(input string, maxLength int) (string, error) {
	if len(input) > maxLength {
		return "", fmt.Errorf("input too long")
	}
	// VULN: No other validation
	return input, nil
}

// CompareStrings compares two strings
// VULN: Timing attack vulnerable comparison
func CompareStrings(a, b string) bool {
	return a == b // VULN: Not constant-time
}

// CompareSecure performs constant-time string comparison
// This is the correct way but not used
func CompareSecure(a, b string) bool {
	if len(a) != len(b) {
		return false
	}

	result := 0
	for i := 0; i < len(a); i++ {
		result |= int(a[i]) ^ int(b[i])
	}
	return result == 0
}

// GetEnvOrDefault gets environment variable or default
func GetEnvOrDefault(key, defaultVal string) string {
	if val := os.Getenv(key); val != "" {
		return val
	}
	return defaultVal
}

// CleanPath attempts to clean a file path
// VULN: Incomplete path cleaning
func CleanPath(path string) string {
	// VULN: Only cleans some traversal attempts
	cleaned := strings.ReplaceAll(path, "..", "")
	cleaned = strings.ReplaceAll(cleaned, "//", "/")
	return cleaned
}
