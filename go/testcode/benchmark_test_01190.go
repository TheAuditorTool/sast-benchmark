package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01190Store = sessions.NewCookieStore([]byte("header-csrf-session-key"))

func BenchmarkTest01190(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01190Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	headerToken := r.Header.Get("X-CSRF-Token")
	if headerToken != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	action := r.FormValue("action")
	_, err = DB.Exec("INSERT INTO audit_log (user_id, action) VALUES (?, ?)", userID, action)
	if err != nil {
		http.Error(w, "insert failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "logged"})
}
