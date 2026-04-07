package testcode

import (
	"net/http"
)

func BenchmarkTest01175(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	userID := r.FormValue("user_id")
	newEmail := r.FormValue("email")
	if userID == "" || newEmail == "" {
		http.Error(w, "missing fields", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("UPDATE users SET email = ? WHERE id = ?", newEmail, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "settings updated"})
}
