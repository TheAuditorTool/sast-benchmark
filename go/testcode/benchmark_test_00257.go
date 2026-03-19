package testcode

import (
	"net/http"
)

func BenchmarkTest00257(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	bar := param
	if 7*42 > 200 {
		bar = "1"
	}
	rows, err := DB.Query("SELECT * FROM users WHERE id = " + bar)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
