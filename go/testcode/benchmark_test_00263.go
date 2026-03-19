package testcode

import (
	"net/http"
)

func BenchmarkTest00263(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("name")
	bar := param
	bar = "admin"
	rows, err := DB.Query("SELECT * FROM users WHERE name = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
