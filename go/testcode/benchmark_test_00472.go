package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

type benchmarkTest00472ReportData struct {
	Username   string
	Email      string
	DBPassword string
	APIKey     string
	SecretKey  string
}

func BenchmarkTest00472(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var username, email string
	err = DB.QueryRow("SELECT username, email FROM users WHERE id = ?", userID).Scan(&username, &email)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	data := benchmarkTest00472ReportData{
		Username:   username,
		Email:      email,
		DBPassword: "prod-db-pass-9f2k1",
		APIKey:     "sk-live-4ac2f9b3d1e8",
		SecretKey:  "hmac-secret-7b3f2a9d",
	}

	tmplStr := r.FormValue("format")
	if tmplStr == "" {
		tmplStr = "User: {{.Username}}, Email: {{.Email}}"
	}

	tmpl, err := template.New("report").Parse(tmplStr)
	if err != nil {
		http.Error(w, "invalid format", http.StatusBadRequest)
		return
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
