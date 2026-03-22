package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest00504(w http.ResponseWriter, r *http.Request) {
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

	docID := strings.TrimPrefix(r.URL.Path, "/api/documents/")

	var title, content, ownerID string
	err = DB.QueryRow("SELECT title, content, owner_id FROM documents WHERE id = ?", docID).Scan(&title, &content, &ownerID)
	if err != nil {
		http.Error(w, "document not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{
		"id":      docID,
		"title":   title,
		"content": content,
	})
}
