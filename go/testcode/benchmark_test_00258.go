package testcode

import (
	"net/http"
)

func BenchmarkTest00258(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("name")
	num := len("abcdefghij")
	bar := param
	if num > 5 {
		bar = "admin"
	}
	rows, err := DB.Query("SELECT * FROM users WHERE username = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
