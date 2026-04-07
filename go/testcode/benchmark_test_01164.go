package testcode

import (
	"net/http"
)

func BenchmarkTest01164(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")

	var userID string
	err := DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", sessionToken).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var role string
	err = DB.QueryRow("SELECT role FROM user_roles WHERE user_id = ?", userID).Scan(&role)
	if err != nil || role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	rows, err := DB.Query("SELECT id, username, email FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
