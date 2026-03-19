// Package channels - Email notification channel
package channels

import (
	"crypto/tls"
	"fmt"
	"net/smtp"
	"os/exec"
	"strings"

	"github.com/project-anarchy/go_notifications/internal/config"
)

// EmailChannel handles email notifications
type EmailChannel struct {
	config config.SMTPConfig
}

// NewEmailChannel creates a new email channel
func NewEmailChannel(cfg config.SMTPConfig) *EmailChannel {
	return &EmailChannel{config: cfg}
}

// Name returns the channel name
func (e *EmailChannel) Name() string {
	return "email"
}

// Validate checks if the notification is valid for email
func (e *EmailChannel) Validate(n *Notification) error {
	if n.Recipient == "" {
		return fmt.Errorf("recipient email is required")
	}
	// VULN: No email format validation
	return nil
}

// Send delivers the notification via email
// TAINT SINK: Recipient, Subject, and Message from user input
func (e *EmailChannel) Send(n *Notification) (map[string]interface{}, error) {
	// Build email message
	// VULN: No sanitization of header values - header injection possible
	msg := fmt.Sprintf("From: %s\r\n", e.config.From)
	msg += fmt.Sprintf("To: %s\r\n", n.Recipient) // TAINT SINK: User-controlled recipient
	msg += fmt.Sprintf("Subject: %s\r\n", n.Subject) // TAINT SINK: Header injection via subject
	msg += "MIME-Version: 1.0\r\n"
	msg += "Content-Type: text/html; charset=UTF-8\r\n"
	msg += "\r\n"
	msg += n.Message // TAINT SINK: User-controlled body

	// Add custom headers from metadata
	// VULN: User-controlled headers - header injection
	for key, value := range n.Metadata {
		if strings.HasPrefix(key, "header_") {
			headerName := strings.TrimPrefix(key, "header_")
			msg = fmt.Sprintf("%s: %s\r\n", headerName, value) + msg // TAINT SINK
		}
	}

	// Send via SMTP
	addr := fmt.Sprintf("%s:%d", e.config.Host, e.config.Port)

	var auth smtp.Auth
	if e.config.Username != "" {
		auth = smtp.PlainAuth("", e.config.Username, e.config.Password, e.config.Host)
	}

	// VULN: TLS verification disabled
	tlsConfig := &tls.Config{
		InsecureSkipVerify: true, // VULN: No certificate validation
		ServerName:         e.config.Host,
	}

	var err error
	if e.config.UseTLS {
		err = e.sendWithTLS(addr, auth, e.config.From, n.Recipient, []byte(msg), tlsConfig)
	} else {
		err = smtp.SendMail(addr, auth, e.config.From, []string{n.Recipient}, []byte(msg))
	}

	if err != nil {
		return nil, fmt.Errorf("failed to send email: %w", err)
	}

	return map[string]interface{}{
		"sent_to": n.Recipient,
		"subject": n.Subject,
	}, nil
}

func (e *EmailChannel) sendWithTLS(addr string, auth smtp.Auth, from, to string, msg []byte, tlsConfig *tls.Config) error {
	// Connect with TLS
	conn, err := tls.Dial("tcp", addr, tlsConfig)
	if err != nil {
		return err
	}
	defer conn.Close()

	client, err := smtp.NewClient(conn, e.config.Host)
	if err != nil {
		return err
	}
	defer client.Close()

	if auth != nil {
		if err = client.Auth(auth); err != nil {
			return err
		}
	}

	if err = client.Mail(from); err != nil {
		return err
	}
	if err = client.Rcpt(to); err != nil {
		return err
	}

	w, err := client.Data()
	if err != nil {
		return err
	}

	_, err = w.Write(msg)
	if err != nil {
		return err
	}

	return w.Close()
}

// SendViaMailCommand sends email using system mail command
// VULN: Command injection via recipient or subject
func (e *EmailChannel) SendViaMailCommand(n *Notification) error {
	// VULN: User input directly in shell command
	// n.Recipient could be "user@example.com; rm -rf /"
	cmd := exec.Command("mail",
		"-s", n.Subject, // TAINT SINK: Command injection
		n.Recipient,     // TAINT SINK: Command injection
	)

	stdin, err := cmd.StdinPipe()
	if err != nil {
		return err
	}

	go func() {
		defer stdin.Close()
		stdin.Write([]byte(n.Message))
	}()

	return cmd.Run()
}

// SendViaSendmail uses sendmail binary
// VULN: Command injection
func (e *EmailChannel) SendViaSendmail(n *Notification) error {
	// VULN: Recipient injected into command
	cmdStr := fmt.Sprintf("echo '%s' | sendmail -t %s", n.Message, n.Recipient)
	cmd := exec.Command("sh", "-c", cmdStr) // TAINT SINK: Shell injection
	return cmd.Run()
}
