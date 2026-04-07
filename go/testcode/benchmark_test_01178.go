package testcode

import (
	"net/http"
)

func BenchmarkTest01178(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	_ = r.FormValue("csrf_token")
	userID := r.FormValue("user_id")
	newRole := r.FormValue("role")
	if userID == "" || newRole == "" {
		http.Error(w, "missing fields", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("UPDATE users SET role = ? WHERE id = ?", newRole, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "role updated"})
}
