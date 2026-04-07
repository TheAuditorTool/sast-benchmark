package testcode

import (
	"net/http"
)

func BenchmarkTest01135(w http.ResponseWriter, r *http.Request) {
	token := r.Header.Get("X-Session")
	if token == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	row := DB.QueryRow(
		"SELECT user_id FROM sessions WHERE token = ? AND expires_at > datetime('now') AND revoked = 0",
		token,
	)
	if err := row.Scan(&userID); err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]interface{}{"user_id": userID})
}
