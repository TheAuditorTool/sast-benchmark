package testcode

import (
	"log"
	"net/http"
)

func BenchmarkTest01338(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		log.Printf("db error: %v", err)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
