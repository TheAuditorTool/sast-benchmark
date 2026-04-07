package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01165(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	docID := strings.TrimPrefix(r.URL.Path, "/docs/")

	var authUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var title string
	err = DB.QueryRow(
		"SELECT title FROM documents WHERE id = ? AND user_id = ?",
		docID, authUserID,
	).Scan(&title)
	if err != nil {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"title": title})
}
