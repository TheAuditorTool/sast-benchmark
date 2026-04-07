package testcode

import (
	"net/http"
)

func BenchmarkTest01182(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	csrfParam := r.URL.Query().Get("csrf")
	if csrfParam == "" {
		http.Error(w, "missing csrf", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newPhone := r.FormValue("phone")
	_, err := DB.Exec("UPDATE users SET phone = ? WHERE id = ?", newPhone, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "phone updated"})
}
