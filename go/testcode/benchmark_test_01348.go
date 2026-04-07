package testcode

import (
	"fmt"
	"log"
	"net/http"
)

func BenchmarkTest01348(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		wrapped := fmt.Errorf("operation failed: %w", err)
		log.Printf("%v", wrapped)
		http.Error(w, "operation failed", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
