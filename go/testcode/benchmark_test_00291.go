package testcode

import (
	"net/http"
	"strconv"
)

func BenchmarkTest00291(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	n, err := strconv.Atoi(param)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	bar := strconv.Itoa(n)
	rows, err := DB.Query("SELECT * FROM users WHERE id = " + bar)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
