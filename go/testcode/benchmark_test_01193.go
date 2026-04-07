package testcode

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"net/http"
)

var benchmarkTest01193Secret = []byte("server-side-hmac-secret-key")

func benchmarkTest01193Sign(sessionID string) string {
	mac := hmac.New(sha256.New, benchmarkTest01193Secret)
	mac.Write([]byte(sessionID))
	return hex.EncodeToString(mac.Sum(nil))
}

func BenchmarkTest01193(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	sessionCookie, err := r.Cookie("session_id")
	if err != nil {
		http.Error(w, "missing session", http.StatusForbidden)
		return
	}
	expected := benchmarkTest01193Sign(sessionCookie.Value)
	csrfCookie, err := r.Cookie("csrf")
	if err != nil || csrfCookie.Value != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	if r.FormValue("csrf_token") != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newPlan := r.FormValue("plan")
	_, err = DB.Exec("UPDATE subscriptions SET plan = ? WHERE user_id = ?", newPlan, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "plan updated"})
}
