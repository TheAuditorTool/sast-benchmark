package testcode

import (
	"net/http"
)

func BenchmarkTest00270(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("name")
	m := make(map[string]string)
	m["user_input"] = param
	m["default"] = "guest"
	bar := m["default"]
	rows, err := DB.Query("SELECT * FROM users WHERE name = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
