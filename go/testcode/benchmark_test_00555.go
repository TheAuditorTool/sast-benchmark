package testcode

import (
	"net/http"
	"net/smtp"
	"os"
)

func BenchmarkTest00555(w http.ResponseWriter, r *http.Request) {
	var req struct {
		To      string `json:"to"`
		Subject string `json:"subject"`
		Body    string `json:"body"`
	}
	if err := ParseJSONBody(r, &req); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	smtpHost := os.Getenv("SMTP_HOST")
	smtpUser := os.Getenv("SMTP_USER")
	smtpPass := os.Getenv("SMTP_PASS")

	auth := smtp.PlainAuth("", smtpUser, smtpPass, smtpHost)

	msg := []byte(
		"To: " + req.To + "\r\n" +
			"Subject: " + req.Subject + "\r\n" +
			"\r\n" +
			req.Body + "\r\n",
	)

	err := smtp.SendMail(smtpHost+":587", auth, smtpUser, []string{req.To}, msg)
	if err != nil {
		RespondJSON(w, http.StatusInternalServerError, map[string]string{"error": "send failed"})
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "sent"})
}
