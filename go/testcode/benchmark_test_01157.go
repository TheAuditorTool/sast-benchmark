package testcode

import (
	"net/http"
	"strings"
)

func BenchmarkTest01157(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")
	userID := strings.TrimPrefix(r.URL.Path, "/profile/")

	var sessionUserID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&sessionUserID)
	if err != nil || sessionUserID != userID {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var body map[string]interface{}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	if name, ok := body["name"].(string); ok {
		DB.Exec("UPDATE users SET name = ? WHERE id = ?", name, userID)
	}
	if role, ok := body["role"].(string); ok {
		DB.Exec("UPDATE users SET role = ? WHERE id = ?", role, userID)
	}
	if email, ok := body["email"].(string); ok {
		DB.Exec("UPDATE users SET email = ? WHERE id = ?", email, userID)
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
