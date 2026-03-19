package testcode

import (
	"net/http"
)

func BenchmarkTest00389(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	query := "WITH user_data AS (SELECT * FROM users WHERE id = " + id + ") SELECT * FROM user_data"
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
