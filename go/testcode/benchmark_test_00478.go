package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

var benchmarkTest00478KnownTemplates = map[string]*template.Template{
	"summary":  template.Must(template.New("summary").Parse("Account: {{.Username}} | Balance: {{.Balance}} | Status: {{.Status}}")),
	"detail":   template.Must(template.New("detail").Parse("User: {{.Username}}\nEmail: {{.Email}}\nJoined: {{.CreatedAt}}\nStatus: {{.Status}}")),
	"greeting": template.Must(template.New("greeting").Parse("Welcome back, {{.Username}}! Last login: {{.LastLogin}}")),
}

func BenchmarkTest00478(w http.ResponseWriter, r *http.Request) {
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

	var username, email, createdAt, lastLogin, status, balance string
	err = DB.QueryRow(
		"SELECT username, email, created_at, last_login, status, balance FROM users WHERE id = ?",
		userID,
	).Scan(&username, &email, &createdAt, &lastLogin, &status, &balance)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	tmplName := r.URL.Query().Get("view")
	if tmplName == "" {
		tmplName = "greeting"
	}

	tmpl, ok := benchmarkTest00478KnownTemplates[tmplName]
	if !ok {
		http.Error(w, "unknown view", http.StatusBadRequest)
		return
	}

	data := map[string]string{
		"Username":  username,
		"Email":     email,
		"CreatedAt": createdAt,
		"LastLogin": lastLogin,
		"Status":    status,
		"Balance":   balance,
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
