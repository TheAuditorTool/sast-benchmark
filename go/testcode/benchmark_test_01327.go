package testcode

import (
	"fmt"
	"net/http"
	"os"
)

func BenchmarkTest01327(w http.ResponseWriter, r *http.Request) {
	id := r.URL.Query().Get("id")
	var count int
	err := DB.QueryRow("SELECT COUNT(*) FROM orders WHERE user_id = ?", id).Scan(&count)
	if err != nil {
		hostname, _ := os.Hostname()
		fmt.Fprintf(w, "server: %s error: %v", hostname, err)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]int{"count": count})
}
