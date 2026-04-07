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
	transport := &http.Transport{
		TLSClientConfig: &tls.Config{
			InsecureSkipVerify: true,
		},
		DisableKeepAlives: false,
	}

	client := &http.Client{
		Timeout:   timeout,
		Transport: transport,
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
	return nil
}

// Send delivers the notification via webhook
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

	req, err := http.NewRequest("POST", n.Recipient, bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}

	req.Header.Set("Content-Type", "application/json")

	for key, value := range n.Metadata {
		if strings.HasPrefix(key, "header_") {
			headerName := strings.TrimPrefix(key, "header_")
			req.Header.Set(headerName, value)
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
func (w *WebhookChannel) SendToURL(targetURL, method string, headers map[string]string, body string) (map[string]interface{}, error) {
	req, err := http.NewRequest(method, targetURL, strings.NewReader(body))
	if err != nil {
		return nil, err
	}

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
func (w *WebhookChannel) SendWithCurl(targetURL string, headers map[string]string, body string) (string, error) {
	args := []string{"-X", "POST"}

	for key, value := range headers {
		args = append(args, "-H", fmt.Sprintf("%s: %s", key, value))
	}

	args = append(args, "-d", body)
	args = append(args, targetURL)

	cmd := exec.Command("curl", args...)
	output, err := cmd.CombinedOutput()

	return string(output), err
}

// ValidateWebhookURL checks if URL is valid
func (w *WebhookChannel) ValidateWebhookURL(targetURL string) error {
	parsed, err := url.Parse(targetURL)
	if err != nil {
		return err
	}

	// Only checks scheme
	if parsed.Scheme != "http" && parsed.Scheme != "https" {
		return fmt.Errorf("invalid scheme: %s", parsed.Scheme)
	}

	return nil
}

// isInternalIP checks if IP is internal (not actually used - dead code)
func isInternalIP(ip net.IP) bool {
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
func (w *WebhookChannel) FetchURLContent(targetURL string) ([]byte, error) {
	resp, err := w.client.Get(targetURL)
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return io.ReadAll(resp.Body)
}
