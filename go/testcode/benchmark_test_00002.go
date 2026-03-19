package testcode

import (
	"net/http"
)

func BenchmarkTest00002(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	rows, err := DB.Query("SELECT * FROM users WHERE id = ?", id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
