package testcode

import (
	"net/http"
)

func BenchmarkTest01187(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	csrfCookie, err := r.Cookie("csrf")
	if err != nil {
		http.Error(w, "missing csrf cookie", http.StatusForbidden)
		return
	}
	formToken := r.FormValue("csrf_token")
	if csrfCookie.Value != formToken {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newEmail := r.FormValue("email")
	_, err = DB.Exec("UPDATE users SET email = ? WHERE id = ?", newEmail, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "email updated"})
}
