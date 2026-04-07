package testcode

import (
	"fmt"
	"net/http"
	"strconv"
)

func BenchmarkTest00691(w http.ResponseWriter, r *http.Request) {
	raw := r.URL.Query().Get("id")
	id, err := strconv.ParseInt(raw, 10, 64)
	if err != nil {
		http.Error(w, "invalid id", http.StatusBadRequest)
		return
	}
	query := fmt.Sprintf("SELECT name FROM users WHERE id = %d", id)
	row := DB.QueryRow(query)
	var name string
	if err := row.Scan(&name); err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
