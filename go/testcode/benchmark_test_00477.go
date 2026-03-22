package testcode

import (
	"net/http"
	"text/template"
)

var benchmarkTest00477AllowedPages = map[string]string{
	"home":     "templates/home.html",
	"about":    "templates/about.html",
	"contact":  "templates/contact.html",
	"faq":      "templates/faq.html",
	"privacy":  "templates/privacy.html",
}

func BenchmarkTest00477(w http.ResponseWriter, r *http.Request) {
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

	page := r.URL.Query().Get("page")
	if page == "" {
		page = "home"
	}

	tmplPath, ok := benchmarkTest00477AllowedPages[page]
	if !ok {
		http.Error(w, "page not found", http.StatusNotFound)
		return
	}

	tmpl, err := template.ParseFiles(tmplPath)
	if err != nil {
		http.Error(w, "page not found", http.StatusNotFound)
		return
	}

	data := map[string]string{
		"Username": username,
		"UserID":   userID,
	}

	w.Header().Set("Content-Type", "text/html")
	if err := tmpl.Execute(w, data); err != nil {
		http.Error(w, "render error", http.StatusInternalServerError)
	}
}
