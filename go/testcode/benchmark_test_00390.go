package testcode

import (
	"net/http"
)

func BenchmarkTest00390(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	rows, err := DB.Query("WITH user_data AS (SELECT * FROM users WHERE id = ?) SELECT * FROM user_data", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
