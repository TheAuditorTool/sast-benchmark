package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01196Store = sessions.NewCookieStore([]byte("password-change-session-key"))

func BenchmarkTest01196(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01196Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	if r.FormValue("csrf_token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newPassword := r.FormValue("new_password")
	_, err = DB.Exec("UPDATE users SET password = ? WHERE id = ?", newPassword, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "password changed"})
}
