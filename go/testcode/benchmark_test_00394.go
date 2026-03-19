package testcode

import (
	"net/http"
)

func BenchmarkTest00394(w http.ResponseWriter, r *http.Request) {
	col := r.URL.Query().Get("column")
	allowed := map[string]bool{"name": true, "email": true, "id": true}
	if !allowed[col] {
		col = "id"
	}
	rows, err := DB.Query("SELECT " + col + " FROM users")
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
