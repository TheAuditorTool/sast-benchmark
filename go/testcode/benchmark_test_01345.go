package testcode

import (
	"net/http"
)

func BenchmarkTest01345(w http.ResponseWriter, r *http.Request) {
	w.Header().Del("Server")
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		http.Error(w, "not found", http.StatusNotFound)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
