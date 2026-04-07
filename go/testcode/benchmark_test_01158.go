package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01158(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	requestedUserID := strings.TrimPrefix(r.URL.Path, "/users/")

	var authUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&authUserID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if authUserID != requestedUserID {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var email string
	err = DB.QueryRow("SELECT email FROM users WHERE id = ?", authUserID).Scan(&email)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"email": email})
}
