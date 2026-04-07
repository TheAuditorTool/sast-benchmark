package testcode

import (
	"fmt"
	"math/rand"
	"net/http"
)

func benchmarkTest01342NewRequestID() string {
	return fmt.Sprintf("%016x", rand.Int63())
}

func BenchmarkTest01342(w http.ResponseWriter, r *http.Request) {
	reqID := benchmarkTest01342NewRequestID()
	w.Header().Set("X-Request-ID", reqID)
	id := r.URL.Query().Get("id")
	var name string
	err := DB.QueryRow("SELECT name FROM users WHERE id = ?", id).Scan(&name)
	if err != nil {
		http.Error(w, "error", http.StatusInternalServerError)
		return
	}
	RespondJSON(w, http.StatusOK, map[string]string{"name": name})
}
