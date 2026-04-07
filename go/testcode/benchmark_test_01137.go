package testcode

import (
	"net/http"

	"github.com/google/uuid"
)

func BenchmarkTest01137(w http.ResponseWriter, r *http.Request) {
	oldToken := r.Header.Get("X-Refresh-Token")
	if oldToken == "" {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID int
	row := DB.QueryRow(
		"SELECT user_id FROM refresh_tokens WHERE token = ? AND expires_at > datetime('now')",
		oldToken,
	)
	if err := row.Scan(&userID); err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	DB.Exec("DELETE FROM refresh_tokens WHERE token = ?", oldToken)

	newToken := uuid.New().String()
	DB.Exec(
		"INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES (?, ?, datetime('now', '+30 days'))",
		userID, newToken,
	)

	RespondJSON(w, http.StatusOK, map[string]string{"refresh_token": newToken})
}
