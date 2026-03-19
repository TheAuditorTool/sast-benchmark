// Package channels - Webhook notification channel
package channels

import (
	"bytes"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"io"
	"net"
	"net/http"
	"net/url"
	"os/exec"
	"strings"
	"time"
)

// WebhookChannel handles webhook notifications
type WebhookChannel struct {
	client  *http.Client
	timeout time.Duration
}

// NewWebhookChannel creates a new webhook channel
func NewWebhookChannel(timeout time.Duration) *WebhookChannel {
	// VULN: No TLS certificate verification
	transport := &http.Transport{
		TLSClientConfig: &tls.Config{
			InsecureSkipVerify: true, // VULN: Accepts any certificate
		},
		// VULN: Follows redirects to any domain
		DisableKeepAlives: false,
	}

	client := &http.Client{
		Timeout:   timeout,
		Transport: transport,
		// VULN: No redirect policy - follows all redirects
	}

	return &WebhookChannel{
		client:  client,
		timeout: timeout,
	}
}

// Name returns the channel name
func (w *WebhookChannel) Name() string {
	return "webhook"
}

// Validate checks if the notification is valid for webhook
func (w *WebhookChannel) Validate(n *Notification) error {
	if n.Recipient == "" {
		return fmt.Errorf("webhook URL is required")
	}
	// VULN: No URL validation - accepts any URL including internal IPs
	return nil
}

// Send delivers the notification via webhook
// TAINT SINK: URL (Recipient) is user-controlled - SSRF vulnerability
func (w *WebhookChannel) Send(n *Notification) (map[string]interface{}, error) {
	payload := map[string]interface{}{
		"subject": n.Subject,
		"message": n.Message,
		"metadata": n.Metadata,
		"timestamp": time.Now().Unix(),
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return nil, err
	}

	// VULN: SSRF - User-controlled URL, can access internal services
	// n.Recipient could be:
	// - http://169.254.169.254/latest/meta-data/ (AWS metadata)
	// - http://localhost:6379/ (Redis)
	// - http://internal-service:8080/admin (Internal admin)
	req, err := http.NewRequest("POST", n.Recipient, bytes.NewBuffer(body)) // TAINT SINK
	if err != nil {
		return nil, err
	}

	req.Header.Set("Content-Type", "application/json")

	// VULN: User-controlled headers
	for key, value := range n.Metadata {
		if strings.HasPrefix(key, "header_") {
			headerName := strings.TrimPrefix(key, "header_")
			req.Header.Set(headerName, value) // TAINT SINK: Header injection
		}
	}

	resp, err := w.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	respBody, _ := io.ReadAll(resp.Body)

	return map[string]interface{}{
		"status_code": resp.StatusCode,
		"response":    string(respBody),
		"url":         n.Recipient,
	}, nil
}

// SendToURL sends a request to any URL with custom method and headers
// VULN: Full SSRF - arbitrary HTTP requests
func (w *WebhookChannel) SendToURL(targetURL, method string, headers map[string]string, body string) (map[string]interface{}, error) {
	// VULN: No URL validation at all
	// No check for internal IPs, localhost, or cloud metadata endpoints

	req, err := http.NewRequest(method, targetURL, strings.NewReader(body)) // TAINT SINK
	if err != nil {
		return nil, err
	}

	// VULN: All user-provided headers are set
	for key, value := range headers {
		req.Header.Set(key, value)
	}

	resp, err := w.client.Do(req)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	respBody, _ := io.ReadAll(resp.Body)

	return map[string]interface{}{
		"url":         targetURL,
		"method":      method,
		"status_code": resp.StatusCode,
		"headers":     resp.Header,
		"body":        string(respBody),
	}, nil
}

// SendWithCurl uses curl command for webhook delivery
// VULN: Command injection via URL or headers
func (w *WebhookChannel) SendWithCurl(targetURL string, headers map[string]string, body string) (string, error) {
	args := []string{"-X", "POST"}

	// VULN: Headers injected into command
	for key, value := range headers {
		args = append(args, "-H", fmt.Sprintf("%s: %s", key, value))
	}

	args = append(args, "-d", body)
	args = append(args, targetURL) // TAINT SINK: URL in command

	cmd := exec.Command("curl", args...)
	output, err := cmd.CombinedOutput()

	return string(output), err
}

// ValidateWebhookURL checks if URL is valid
// VULN: Incomplete validation - doesn't block internal IPs
func (w *WebhookChannel) ValidateWebhookURL(targetURL string) error {
	parsed, err := url.Parse(targetURL)
	if err != nil {
		return err
	}

	// Only checks scheme
	if parsed.Scheme != "http" && parsed.Scheme != "https" {
		return fmt.Errorf("invalid scheme: %s", parsed.Scheme)
	}

	// VULN: Doesn't check for:
	// - localhost / 127.0.0.1
	// - Private IP ranges (10.x, 172.16.x, 192.168.x)
	// - Link-local (169.254.x)
	// - Cloud metadata IPs

	return nil
}

// isInternalIP checks if IP is internal (not actually used - dead code)
func isInternalIP(ip net.IP) bool {
	// This function exists but is never called - VULN: Dead code
	privateRanges := []string{
		"10.0.0.0/8",
		"172.16.0.0/12",
		"192.168.0.0/16",
		"127.0.0.0/8",
		"169.254.0.0/16",
	}

	for _, cidr := range privateRanges {
		_, network, _ := net.ParseCIDR(cidr)
		if network.Contains(ip) {
			return true
		}
	}
	return false
}

// FetchURLContent fetches content from a URL
// VULN: SSRF for content retrieval
func (w *WebhookChannel) FetchURLContent(targetURL string) ([]byte, error) {
	resp, err := w.client.Get(targetURL) // TAINT SINK: SSRF
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}
