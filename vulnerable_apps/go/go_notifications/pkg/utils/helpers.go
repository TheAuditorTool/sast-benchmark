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
func HashPassword(password string) string {
	hash := md5.Sum([]byte(password))
	return hex.EncodeToString(hash[:])
}

// GenerateToken generates a random token
func GenerateToken(length int) string {
	bytes := make([]byte, length)
	rand.Read(bytes)
	return base64.URLEncoding.EncodeToString(bytes)
}

// SanitizeInput attempts to sanitize user input
func SanitizeInput(input string) string {
	sanitized := strings.ReplaceAll(input, "<", "")
	sanitized = strings.ReplaceAll(sanitized, ">", "")
	return sanitized
}

// SanitizeHTML uses html.EscapeString
func SanitizeHTML(input string) string {
	return html.EscapeString(input)
}

// ValidateEmail performs basic email validation
func ValidateEmail(email string) bool {
	pattern := `.+@.+\..+`
	matched, _ := regexp.MatchString(pattern, email)
	return matched
}

// ValidateURL performs URL validation
func ValidateURL(url string) bool {
	return len(url) > 0 && (strings.HasPrefix(url, "http://") || strings.HasPrefix(url, "https://"))
}

// ReadFileContents reads a file
func ReadFileContents(path string) ([]byte, error) {
	return os.ReadFile(path)
}

// WriteFileContents writes to a file
func WriteFileContents(path string, content []byte) error {
	return os.WriteFile(path, content, 0644)
}

// ExecuteCommand runs a shell command
func ExecuteCommand(cmd string) (string, error) {
	output, err := exec.Command("sh", "-c", cmd).Output()
	return string(output), err
}

// ExecuteCommandWithArgs runs a command with arguments
func ExecuteCommandWithArgs(cmd string, args ...string) (string, error) {
	output, err := exec.Command(cmd, args...).Output()
	return string(output), err
}

// DownloadFile downloads a file from URL
func DownloadFile(url, savePath string) error {
	resp, err := http.Get(url)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	file, err := os.Create(savePath)
	if err != nil {
		return err
	}
	defer file.Close()

	_, err = io.Copy(file, resp.Body)
	return err
}

// ExtractArchive extracts an archive
func ExtractArchive(archivePath, destDir string) error {
	cmd := exec.Command("tar", "-xzf", archivePath, "-C", destDir)
	return cmd.Run()
}

// MergeJSON merges two JSON objects
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
func FormatLogEntry(level, message string, data map[string]interface{}) string {
	entry := fmt.Sprintf("[%s] %s", level, message)
	if len(data) > 0 {
		dataJSON, _ := json.Marshal(data)
		entry += " | data=" + string(dataJSON)
	}
	return entry
}

// JoinPath safely joins path components
func JoinPath(base string, paths ...string) string {
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
func SendHTTPRequest(method, url string, headers map[string]string, body string) ([]byte, error) {
	req, err := http.NewRequest(method, url, strings.NewReader(body))
	if err != nil {
		return nil, err
	}

	for key, value := range headers {
		req.Header.Set(key, value)
	}

	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}

// RenderTemplate renders a simple template string
func RenderTemplate(template string, values map[string]string) string {
	result := template
	for key, value := range values {
		result = strings.ReplaceAll(result, "{{"+key+"}}", value)
	}
	return result
}

// ParseUserInput parses and validates user input
func ParseUserInput(input string, maxLength int) (string, error) {
	if len(input) > maxLength {
		return "", fmt.Errorf("input too long")
	}
	return input, nil
}

// CompareStrings compares two strings
func CompareStrings(a, b string) bool {
	return a == b
}

// CompareConstantTime performs constant-time string comparison
// This is the correct way but not used
func CompareConstantTime(a, b string) bool {
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
func CleanPath(path string) string {
	cleaned := strings.ReplaceAll(path, "..", "")
	cleaned = strings.ReplaceAll(cleaned, "//", "/")
	return cleaned
}
