// Package channels - Slack notification channel
package channels

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"os/exec"
	"text/template"
)

// SlackChannel handles Slack notifications
type SlackChannel struct {
	webhookURL string
	client     *http.Client
}

// NewSlackChannel creates a new Slack channel
func NewSlackChannel(webhookURL string) *SlackChannel {
	return &SlackChannel{
		webhookURL: webhookURL,
		client:     &http.Client{},
	}
}

// Name returns the channel name
func (s *SlackChannel) Name() string {
	return "slack"
}

// Validate checks if the notification is valid for Slack
func (s *SlackChannel) Validate(n *Notification) error {
	if s.webhookURL == "" && n.Recipient == "" {
		return fmt.Errorf("slack webhook URL is required")
	}
	return nil
}

// SlackMessage represents a Slack message payload
type SlackMessage struct {
	Text        string            `json:"text"`
	Channel     string            `json:"channel,omitempty"`
	Username    string            `json:"username,omitempty"`
	IconEmoji   string            `json:"icon_emoji,omitempty"`
	Attachments []SlackAttachment `json:"attachments,omitempty"`
	Blocks      []interface{}     `json:"blocks,omitempty"`
}

// SlackAttachment represents a Slack message attachment
type SlackAttachment struct {
	Color      string `json:"color,omitempty"`
	Title      string `json:"title,omitempty"`
	Text       string `json:"text,omitempty"`
	Footer     string `json:"footer,omitempty"`
	AuthorName string `json:"author_name,omitempty"`
}

// Send delivers the notification via Slack
func (s *SlackChannel) Send(n *Notification) (map[string]interface{}, error) {
	webhookURL := s.webhookURL
	if n.Recipient != "" {
		webhookURL = n.Recipient
	}

	// Build Slack message
	msg := SlackMessage{
		Text:     fmt.Sprintf("*%s*\n%s", n.Subject, n.Message),
		Username: n.Metadata["username"],
		Channel:  n.Metadata["channel"],
	}

	// Add attachment if metadata present
	if n.Metadata["color"] != "" || n.Metadata["footer"] != "" {
		msg.Attachments = []SlackAttachment{{
			Color:  n.Metadata["color"],
			Title:  n.Subject,
			Text:   n.Message,
			Footer: n.Metadata["footer"],
		}}
	}

	body, err := json.Marshal(msg)
	if err != nil {
		return nil, err
	}

	resp, err := s.client.Post(webhookURL, "application/json", bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("slack returned status %d", resp.StatusCode)
	}

	return map[string]interface{}{
		"sent_to":  webhookURL,
		"channel":  msg.Channel,
		"username": msg.Username,
	}, nil
}

// SendWithTemplate sends a Slack message using a template
func (s *SlackChannel) SendWithTemplate(n *Notification, templateStr string) (map[string]interface{}, error) {
	tmpl, err := template.New("slack").Parse(templateStr)
	if err != nil {
		return nil, err
	}

	var buf bytes.Buffer
	err = tmpl.Execute(&buf, map[string]interface{}{
		"Subject":  n.Subject,
		"Message":  n.Message,
		"Metadata": n.Metadata,
	})
	if err != nil {
		return nil, err
	}

	n.Message = buf.String()
	return s.Send(n)
}

// SendBlockKit sends a Block Kit formatted message
func (s *SlackChannel) SendBlockKit(n *Notification, blocksJSON string) (map[string]interface{}, error) {
	webhookURL := s.webhookURL
	if n.Recipient != "" {
		webhookURL = n.Recipient
	}

	var blocks []interface{}
	if err := json.Unmarshal([]byte(blocksJSON), &blocks); err != nil {
		return nil, fmt.Errorf("invalid blocks JSON: %w", err)
	}

	msg := SlackMessage{
		Text:   n.Subject, // Fallback text
		Blocks: blocks,
	}

	body, _ := json.Marshal(msg)

	resp, err := s.client.Post(webhookURL, "application/json", bytes.NewBuffer(body))
	if err != nil {
		return nil, err
	}
	defer resp.Body.Close()

	return map[string]interface{}{"status": resp.StatusCode}, nil
}

// PostToChannel uses Slack CLI/API to post
func (s *SlackChannel) PostToChannel(channel, message string) error {
	cmd := exec.Command("slack-cli", "chat", "send", channel, message)
	return cmd.Run()
}

// UploadFile uploads a file to Slack
func (s *SlackChannel) UploadFile(channel, filePath, comment string) error {
	cmdStr := fmt.Sprintf("slack-cli file upload -c %s -f %s -m %s", channel, filePath, comment)
	cmd := exec.Command("sh", "-c", cmdStr)
	return cmd.Run()
}
