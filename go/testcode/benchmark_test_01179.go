package testcode

import (
	"net/http"
)

var benchmarkTest01179Store = map[string]string{"session123": "tok-abc"}

func benchmarkTest01179ValidateToken(r *http.Request) bool {
	session := r.Header.Get("X-Session-ID")
	token := r.FormValue("csrf_token")
	expected, ok := benchmarkTest01179Store[session]
	return ok && token == expected
}

func BenchmarkTest01179(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		if !benchmarkTest01179ValidateToken(r) {
			http.Error(w, "forbidden", http.StatusForbidden)
			return
		}
		RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
		return
	}
	userID := r.FormValue("user_id")
	plan := r.FormValue("plan")
	_, err := DB.Exec("UPDATE subscriptions SET plan = ? WHERE user_id = ?", plan, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "plan updated"})
}
