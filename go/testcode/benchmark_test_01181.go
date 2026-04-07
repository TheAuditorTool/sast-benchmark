package testcode

import (
	"net/http"
)

var benchmarkTest01181GlobalToken = "hardcoded-csrf-secret-v1"

func BenchmarkTest01181(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	token := r.FormValue("csrf_token")
	if token != benchmarkTest01181GlobalToken {
		http.Error(w, "forbidden", http.StatusForbidden)
		return
	}
	userID := r.FormValue("user_id")
	newName := r.FormValue("name")
	_, err := DB.Exec("UPDATE users SET name = ? WHERE id = ?", newName, userID)
	if err != nil {
		http.Error(w, "update failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"status": "name updated"})
}
