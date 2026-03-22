package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

func BenchmarkTest00471(w http.ResponseWriter, r *http.Request) {
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

	var username, displayName string
	err = DB.QueryRow("SELECT username, display_name FROM users WHERE id = ?", userID).Scan(&username, &displayName)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	tmplStr := r.FormValue("template")
	if tmplStr == "" {
		tmplStr = "Hello, {{.Name}}!"
	}

	tmpl, err := template.New("user").Parse(tmplStr)
	if err != nil {
		http.Error(w, "invalid template", http.StatusBadRequest)
		return
	}

	data := map[string]string{
		"Name":    displayName,
		"User":    username,
		"UserID":  userID,
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
