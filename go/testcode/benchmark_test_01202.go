package testcode

import (
	"crypto/subtle"
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01202Store = sessions.NewCookieStore([]byte("constant-time-session-key-32b"))

func BenchmarkTest01202(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01202Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	provided := r.FormValue("csrf_token")
	if subtle.ConstantTimeCompare([]byte(provided), []byte(expected)) != 1 {
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
