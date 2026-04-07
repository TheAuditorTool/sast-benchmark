// Package channels - Dispatcher routes notifications to appropriate channels
package channels

import (
	"fmt"
)

// Dispatcher routes notifications to the appropriate channel
type Dispatcher struct {
	email   *EmailChannel
	webhook *WebhookChannel
	slack   *SlackChannel
	file    *FileChannel
}

// NewDispatcher creates a new notification dispatcher
func NewDispatcher(email *EmailChannel, webhook *WebhookChannel, slack *SlackChannel, file *FileChannel) *Dispatcher {
	return &Dispatcher{
		email:   email,
		webhook: webhook,
		slack:   slack,
		file:    file,
	}
}

// Dispatch sends a notification via the appropriate channel
func (d *Dispatcher) Dispatch(n *Notification) (map[string]interface{}, error) {
	switch n.Channel {
	case "email":
		return d.email.Send(n)
	case "webhook":
		return d.webhook.Send(n)
	case "slack":
		return d.slack.Send(n)
	case "file":
		return d.file.Send(n)
	default:
		return nil, fmt.Errorf("unknown channel: %s", n.Channel)
	}
}

// WebhookChannel returns the webhook channel for direct access
func (d *Dispatcher) WebhookChannel() *WebhookChannel {
	return d.webhook
}

// DispatchMulti sends to multiple channels
func (d *Dispatcher) DispatchMulti(n *Notification, channels []string) []DeliveryResult {
	results := make([]DeliveryResult, 0, len(channels))

	for _, ch := range channels {
		notifCopy := *n
		notifCopy.Channel = ch

		result, err := d.Dispatch(&notifCopy)

		dr := DeliveryResult{
			Channel:  ch,
			Success:  err == nil,
			Response: result,
		}
		if err != nil {
			dr.Error = err.Error()
		}

		results = append(results, dr)
	}

	return results
}
