package testcode

import (
	"net/http"
	"time"
)

func BenchmarkTest00452(w http.ResponseWriter, r *http.Request) {
	cookie, err := r.Cookie("session_token")
	if err != nil {
		http.Error(w, "no session", http.StatusUnauthorized)
		return
	}

	var userID int64
	var expiresAt time.Time

	row := DB.QueryRow("SELECT user_id, expires_at FROM sessions WHERE token = ?", cookie.Value)
	if err := row.Scan(&userID, &expiresAt); err != nil {
		http.Error(w, "invalid session", http.StatusUnauthorized)
		return
	}

	if time.Now().After(expiresAt) {
		http.Error(w, "session expired", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{
		"user_id":    userID,
		"expires_at": expiresAt.UTC().Format(time.RFC3339),
	})
}
