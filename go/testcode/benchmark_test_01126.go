package testcode

import (
	"net/http"
)

func BenchmarkTest01126(w http.ResponseWriter, r *http.Request) {
	token := r.URL.Query().Get("reset_token")
	if token == "" {
		http.Error(w, "missing token", http.StatusBadRequest)
		return
	}

	var userID int
	row := DB.QueryRow("SELECT user_id FROM password_reset_tokens WHERE token = ?", token)
	if err := row.Scan(&userID); err != nil {
		http.Error(w, "invalid token", http.StatusUnauthorized)
		return
	}

	newPassword := r.FormValue("new_password")
	DB.Exec("UPDATE users SET password = ? WHERE id = ?", newPassword, userID)
	DB.Exec("DELETE FROM password_reset_tokens WHERE token = ?", token)

	RespondJSON(w, http.StatusOK, map[string]string{"status": "password updated"})
}
