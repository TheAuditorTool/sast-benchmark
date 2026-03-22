package testcode

import (
	"bytes"
	"net/http"
	"text/template"
)

var benchmarkTest00475Template = template.Must(template.New("welcome").Parse("Hello, {{.Name}}! Your account was created on {{.CreatedAt}}."))

func BenchmarkTest00475(w http.ResponseWriter, r *http.Request) {
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

	var displayName, createdAt string
	err = DB.QueryRow("SELECT display_name, created_at FROM users WHERE id = ?", userID).Scan(&displayName, &createdAt)
	if err != nil {
		http.Error(w, "user not found", http.StatusNotFound)
		return
	}

	data := struct {
		Name      string
		CreatedAt string
	}{
		Name:      displayName,
		CreatedAt: createdAt,
	}

	var buf bytes.Buffer
	if err := benchmarkTest00475Template.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
