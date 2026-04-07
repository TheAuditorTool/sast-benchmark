package testcode

import (
	"net/http"
)

func BenchmarkTest01176(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	userID := r.FormValue("user_id")
	if userID == "" {
		http.Error(w, "missing user_id", http.StatusBadRequest)
		return
	}
	_, err := DB.Exec("DELETE FROM users WHERE id = ?", userID)
	if err != nil {
		http.Error(w, "delete failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "user deleted"})
}
