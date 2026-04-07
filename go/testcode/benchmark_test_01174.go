package testcode

import (
	"net/http"
)

func BenchmarkTest01174(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	userID := r.FormValue("user_id")
	newPassword := r.FormValue("new_password")
	if userID == "" || newPassword == "" {
		http.Error(w, "missing fields", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("UPDATE users SET password = ? WHERE id = ?", newPassword, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "password updated"})
}
