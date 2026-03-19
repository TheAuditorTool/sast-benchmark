// Package channels provides notification delivery implementations
package channels

import "time"

// Notification represents a notification to be sent
type Notification struct {
	ID        int64             `json:"id"`
	Channel   string            `json:"channel"`
	Recipient string            `json:"recipient"`
	Subject   string            `json:"subject"`
	Message   string            `json:"message"`
	Status    string            `json:"status"`
	Error     string            `json:"error,omitempty"`
	Metadata  map[string]string `json:"metadata,omitempty"`
	CreatedAt time.Time         `json:"created_at"`
	SentAt    *time.Time        `json:"sent_at,omitempty"`
}

// Channel defines the interface for notification channels
type Channel interface {
	Name() string
	Send(n *Notification) (map[string]interface{}, error)
	Validate(n *Notification) error
}

// DeliveryResult contains the result of sending a notification
type DeliveryResult struct {
	Success   bool                   `json:"success"`
	Channel   string                 `json:"channel"`
	MessageID string                 `json:"message_id,omitempty"`
	Response  map[string]interface{} `json:"response,omitempty"`
	Error     string                 `json:"error,omitempty"`
}
