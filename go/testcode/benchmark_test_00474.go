package testcode

import (
	"bytes"
	"net/http"
	"os/exec"
	"text/template"
)

func benchmarkTest00474RunCommand(name string) string {
	out, err := exec.Command("sh", "-c", name).Output()
	if err != nil {
		return ""
	}
	return string(out)
}

func BenchmarkTest00474(w http.ResponseWriter, r *http.Request) {
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

	funcMap := template.FuncMap{
		"run": benchmarkTest00474RunCommand,
		"upper": func(s string) string {
			result := ""
			for _, c := range s {
				if c >= 'a' && c <= 'z' {
					result += string(c - 32)
				} else {
					result += string(c)
				}
			}
			return result
		},
	}

	tmplStr := r.FormValue("template")
	if tmplStr == "" {
		tmplStr = "Hello, {{.Username}}!"
	}

	tmpl, err := template.New("report").Funcs(funcMap).Parse(tmplStr)
	if err != nil {
		http.Error(w, "invalid template", http.StatusBadRequest)
		return
	}

	data := map[string]string{
		"Username": username,
		"UserID":   userID,
	}

	var buf bytes.Buffer
	if err := tmpl.Execute(&buf, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/plain")
	w.Write(buf.Bytes())
}
