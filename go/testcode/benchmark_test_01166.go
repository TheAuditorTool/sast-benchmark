package testcode

import (
	"net/http"
)

func BenchmarkTest01166(w http.ResponseWriter, r *http.Request) {
	sessionToken := r.Header.Get("X-Session-Token")

	var userID, role string
	err := DB.QueryRow(
		"SELECT u.id, u.role FROM users u JOIN sessions s ON u.id = s.user_id WHERE s.token = ?",
		sessionToken,
	).Scan(&userID, &role)
	if err != nil || role != "admin" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}

	var body struct {
		TargetID string `json:"target_id"`
		Action   string `json:"action"`
	}
	if err := ParseJSONBody(r, &body); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	DB.Exec("INSERT INTO audit_log (actor_id, target_id, action) VALUES (?, ?, ?)", userID, body.TargetID, body.Action)
	DB.Exec("UPDATE users SET suspended = 1 WHERE id = ?", body.TargetID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "action performed"})
}
