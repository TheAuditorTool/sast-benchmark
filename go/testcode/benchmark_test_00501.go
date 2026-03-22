package testcode

import (
	"net/http"
)

func BenchmarkTest00501(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
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

	theme := r.FormValue("theme")
	language := r.FormValue("language")
	notifications := r.FormValue("notifications")

	_, err = DB.Exec(
		"UPDATE user_settings SET theme = ?, language = ?, notifications = ? WHERE user_id = ?",
		theme, language, notifications, userID,
	)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}

	RespondJSON(w, http.StatusOK, map[string]string{"status": "settings_updated"})
}
