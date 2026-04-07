package testcode

import (
	"net/http"

	"github.com/gorilla/sessions"
)

var benchmarkTest01192Store = sessions.NewCookieStore([]byte("transfer-session-key-32b"))

func BenchmarkTest01192(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	session, err := benchmarkTest01192Store.Get(r, "app-session")
	if err != nil {
		http.Error(w, "session error", http.StatusInternalServerError)
		return
	}
	expected, ok := session.Values["csrf_token"].(string)
	if !ok || expected == "" {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	if r.Header.Get("X-CSRF-Token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	from := r.FormValue("from")
	to := r.FormValue("to")
	amount := r.FormValue("amount")
	_, err = DB.Exec("INSERT INTO transfers (from_account, to_account, amount) VALUES (?, ?, ?)", from, to, amount)
	if err != nil {
		http.Error(w, "transfer failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "transferred"})
}
