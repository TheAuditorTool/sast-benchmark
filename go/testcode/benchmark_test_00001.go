package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest00001(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	query := fmt.Sprintf("SELECT * FROM users WHERE id = %s", id)
	rows, err := DB.Query(query)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	defer rows.Close()
	RespondJSON(w, http.StatusOK, map[string]string{"status": "ok"})
}
