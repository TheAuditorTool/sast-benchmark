package testcode

import (
	"net/http"
)

func BenchmarkTest00049(w http.ResponseWriter, r *http.Request) {
	input := r.URL.Query().Get("name")
	var result string
	err := DB.QueryRow("SELECT email FROM users WHERE name = ?", input).Scan(&result)
	if err != nil {
		http.Error(w, err.Error(), http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"email": result})
}
