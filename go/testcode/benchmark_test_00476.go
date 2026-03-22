package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

const benchmarkTest00476TemplateStr = `Notification for {{.Username}}:

{{.Content}}

Posted at: {{.Timestamp}}`

func BenchmarkTest00476(w http.ResponseWriter, r *http.Request) {
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

	var username string
	err = DB.QueryRow("SELECT username FROM users WHERE id = ?", userID).Scan(&username)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	content := r.FormValue("content")
	if content == "" {
		http.Error(w, "missing content", http.StatusBadRequest)
		return
	}

	tmpl, err := template.New("notification").Parse(benchmarkTest00476TemplateStr)
	if err != nil {
		http.Error(w, "internal error", http.StatusInternalServerError)
		return
	}

	data := struct {
		Username  string
		Content   string
		Timestamp string
	}{
		Username:  username,
		Content:   content,
		Timestamp: r.Header.Get("X-Request-Time"),
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
