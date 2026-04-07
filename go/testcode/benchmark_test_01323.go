package testcode

import (
	"fmt"
	"net/http"
)

func BenchmarkTest01323(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		fmt.Fprintf(w, "Error: %v", err)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
