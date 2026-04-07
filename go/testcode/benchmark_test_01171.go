package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01171(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	recordID := strings.TrimPrefix(r.URL.Path, "/records/")

	var authUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var value string
	err = DB.QueryRow(
		"SELECT value FROM records WHERE id = ? AND user_id = ?",
		recordID, authUserID,
	).Scan(&value)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"value": value})
}
