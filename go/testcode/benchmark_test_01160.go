package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01160(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	docID := strings.TrimPrefix(r.URL.Path, "/documents/")

	var authUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var title, content string
	err = DB.QueryRow(
		"SELECT title, content FROM documents WHERE id = ? AND owner_id = ?",
		docID, authUserID,
	).Scan(&title, &content)
	if err != nil {
		http.Error(w, "not found or forbidden", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"title": title, "content": content})
}
