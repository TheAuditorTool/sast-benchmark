package testcode

import (
	"net/http"
)

func BenchmarkTest01180(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	csrfCookie, err := r.Cookie("csrf")
	if err != nil || csrfCookie.Value == "" {
		http.Error(w, "missing csrf cookie", http.StatusForbidden)
		return
	}
	csrfCookie2, err := r.Cookie("csrf")
	if err != nil || csrfCookie.Value != csrfCookie2.Value {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	addr := r.FormValue("address")
	_, err = DB.Exec("UPDATE users SET address = ? WHERE id = ?", addr, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "address updated"})
}
