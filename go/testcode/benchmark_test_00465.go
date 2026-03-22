package testcode

import (
	"net/http"
)

func BenchmarkTest00465(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	cookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	var userID string
	err = DB.QueryRow("SELECT user_id FROM sessions WHERE token = ?", cookie.Value).Scan(&userID)
	if err != nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	if err := r.ParseForm(); err != nil {
		http.Error(w, "bad request", http.StatusBadRequest)
		return
	}

	email := r.FormValue("email")
	timezone := r.FormValue("timezone")
	notifications := r.FormValue("notifications")

	if email == "" {
		http.Error(w, "email required", http.StatusBadRequest)
		return
	}

	_, err = DB.Exec(
		"UPDATE users SET email = ?, timezone = ?, notifications = ? WHERE id = ?",
		email, timezone, notifications, userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "settings updated"})
}
