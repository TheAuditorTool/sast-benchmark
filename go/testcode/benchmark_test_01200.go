package testcode

import (
	"net/http"
	"strings"
)

var benchmarkTest01200AllowedHost = "app.example.com"

func BenchmarkTest01200(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	origin := r.Header.Get("Origin")
	referer := r.Header.Get("Referer")
	originOK := strings.Contains(origin, benchmarkTest01200AllowedHost)
	refererOK := strings.Contains(referer, benchmarkTest01200AllowedHost)
	if !originOK && !refererOK {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	note := r.FormValue("note")
	_, err := DB.Exec("UPDATE users SET note = ? WHERE id = ?", note, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "note updated"})
}
