package testcode

import (
	"net/http"
)

var benchmarkTest01185TokenStore = map[string]string{"sess-1": "valid-token"}

func BenchmarkTest01185(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	if r.Header.Get("Content-Type") == "application/json" {
		var body struct {
			UserID string `json:"user_id"`
			Data   string `json:"data"`
		}
		if err := ParseJSONBody(r, &body); err != nil {
			http.Error(w, "invalid json", http.StatusBadRequest)
			return
		}
		DB.Exec("UPDATE records SET data = ? WHERE user_id = ?", body.Data, body.UserID)
		RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
		return
	}
	sessionID := r.FormValue("session_id")
	token := r.FormValue("csrf_token")
	expected, ok := benchmarkTest01185TokenStore[sessionID]
	if !ok || token != expected {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	data := r.FormValue("data")
	DB.Exec("UPDATE records SET data = ? WHERE user_id = ?", data, userID)
	RespondJSON(w, http.StatusOK, map[string]string{"status": "updated"})
}
