package testcode

import (
	"net/http"
)

func BenchmarkTest00276(w http.ResponseWriter, r *http.Request) {
	param := r.URL.Query().Get("id")
	ch := make(chan string, 1)
	ch <- param
	bar := <-ch
	rows, err := DB.Query("SELECT * FROM users WHERE id = '" + bar + "'")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
